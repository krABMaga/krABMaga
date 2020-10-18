use abm::{location::Int2D};
use amethyst::{assets::{AssetStorage, Loader, Handle}, core::math::Vector3, core::transform::Transform, prelude::*, renderer::{
    Camera,
    ImageFormat,
    SpriteRender,
    SpriteSheet,
    SpriteSheetFormat,
    Texture,
    resources::Tint,
    palette::Srgba,
}};
use rand::Rng;
use crate::{agent_adapter::AgentAdapter, resources::AntsGrid, resources::ObstaclesGrid, resources::SitesGrid, resources::ToFoodGrid, resources::ToHomeGrid, static_object::StaticObject, static_object::StaticObjectType};
use amethyst::renderer::Transparent;

// Constants regarding the window and the Field2D.
//pub const FIELD_HEIGHT: f32 = 400.0;
//pub const FIELD_WIDTH: f32 = 400.0;
pub const WIDTH: i64 = 200;
pub const HEIGHT: i64 = 200;
pub const FIELD_WIDTH: f32 = WIDTH as f32 + 100.0;
pub const FIELD_HEIGHT: f32 = HEIGHT as f32 + 100.0;
pub const NUM_AGENT: u128 = 1000;
// Nest
pub const HOME_XMIN: i64 = 175;
pub const HOME_XMAX: i64 = 175;
pub const HOME_YMIN: i64 = 175;
pub const HOME_YMAX: i64 = 175;
// Food
pub const FOOD_XMIN: i64 = 25;
pub const FOOD_XMAX: i64 = 25;
pub const FOOD_YMIN: i64 = 25;
pub const FOOD_YMAX: i64 = 25;

// Black
pub const ANT_RGBA: (f32, f32, f32, f32) = (0., 0., 0., 1.);

// Event used to update the tint's alpha
#[derive(Debug)]
pub enum TintEvent {
    UpdateTint(u32, f64, bool)
}


pub struct Environment;

impl SimpleState for Environment {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
        world.register::<StaticObject>(); // Needed because the component is not used within some system's data

        let mut ants_grid = AntsGrid::new(WIDTH, HEIGHT);
        let mut obstacles_grid = ObstaclesGrid::new(WIDTH, HEIGHT);
        let mut sites_grid = SitesGrid::new(WIDTH, HEIGHT);
        let mut to_food_grid = ToFoodGrid::new(WIDTH, HEIGHT);
        let mut to_home_grid = ToHomeGrid::new(WIDTH, HEIGHT);

        initialize_camera(world);
        initialize_static_objects(world, sprite_render.clone(), &mut sites_grid, &mut obstacles_grid, &mut to_food_grid, &mut to_home_grid);
        initialize_ants(world, sprite_render.clone(), &mut ants_grid);

        // Insert the grids in the world as resources, so it can be fetched (in read or write mode).
        // There's no need for a mutex, Amethyst will handle synchronization for us, by checking what Systems require.
        world.insert(ants_grid);
        world.insert(obstacles_grid);
        world.insert(sites_grid);
        world.insert(to_food_grid);
        world.insert(to_home_grid);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    // Make the camera target a slightly bigger area, and offset it a bit to center the Field2D.
    transform.set_translation_xyz((FIELD_WIDTH * 0.5) - 50.0, (FIELD_HEIGHT * 0.5) - 50.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(FIELD_WIDTH + (FIELD_WIDTH - WIDTH as f32), FIELD_HEIGHT + (FIELD_HEIGHT - HEIGHT as f32)))
        .with(transform)
        .build();
}

// TODO: obstacles
fn initialize_static_objects(
    world: &mut World,
    sprite_render: SpriteRender,
    sites_grid: &mut SitesGrid,
    _obstacles_grid: &mut ObstaclesGrid,
    to_food_grid: &mut ToFoodGrid,
    to_home_grid: &mut ToHomeGrid,
) {
    let mut rng = rand::thread_rng();

    // Generate the nest
    let x: i64 = if HOME_XMIN == HOME_XMAX {
        HOME_XMIN
    } else {
        rng.gen_range(HOME_XMIN, HOME_XMAX)
    };
    let y: i64 = if HOME_YMIN == HOME_YMAX {
        HOME_YMIN
    } else {
        rng.gen_range(HOME_YMIN, HOME_YMAX)
    };
    let loc = Int2D { x, y };

    let nest = StaticObject::new(1, loc, StaticObjectType::HOME);
    sites_grid.grid.locs[x as usize][y as usize] = Some(nest.object_type);

    let mut transform = Transform::default();
    transform.set_translation_xyz(x as f32, y as f32, 0.);
    transform.set_scale(Vector3::new(2., 2., 1.));

    let tint = Tint(Srgba::new(1., 0., 0., 1.));
    world.create_entity()
        .with(sprite_render.clone())
        .with(tint)
        .with(transform)
        .with(nest)
        .build();

    // Generate the food resource
    let x: i64 = if FOOD_XMIN == FOOD_XMAX {
        FOOD_XMIN
    } else {
        rng.gen_range(FOOD_XMIN, FOOD_XMAX)
    };
    let y: i64 = if FOOD_YMIN == FOOD_YMAX {
        FOOD_YMIN
    } else {
        rng.gen_range(FOOD_YMIN, FOOD_YMAX)
    };
    let loc = Int2D { x, y };

    let food = StaticObject::new(1, loc, StaticObjectType::FOOD);
    sites_grid.grid.locs[x as usize][y as usize] = Some(food.object_type);

    let mut transform = Transform::default();
    transform.set_translation_xyz(x as f32, y as f32, 0.);
    transform.set_scale(Vector3::new(2., 2., 1.));

    let tint = Tint(Srgba::new(1., 0., 0., 1.));
    world.create_entity()
        .with(sprite_render.clone())
        .with(tint)
        .with(transform)
        .with(food)
        .build();

    // Generate 1x1 tints that represent pheromones. We save the entity index in the grid to be able to
    // send an event containing said index and a f32 representing the pheromone's intensity, to update the tint.
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let tint = Tint(Srgba::new(0., 0., 0., 0.));
            let mut transform = Transform::default();
            transform.set_translation_xyz(i as f32, j as f32, -0.1);
            transform.set_scale(Vector3::new(2., 2., 1.));
            let cell = world.create_entity()
                .with(sprite_render.clone())
                .with(tint)
                .with(transform)
                .with(Transparent) // Required to make the alpha actually do something
                .build();
            to_food_grid.grid.set_value_at_pos(&Int2D { x: i, y: j }, (cell.id(), 0.));
            to_home_grid.grid.set_value_at_pos(&Int2D { x: i, y: j }, (cell.id(), 0.));
        }
    }
}

fn initialize_ants(world: &mut World, sprite_render: SpriteRender, ants_grid: &mut AntsGrid) {
    let (r, g, b, a) = ANT_RGBA;
    let global_tint = Tint(Srgba::new(r, g, b, a));

    for ant_id in 0..NUM_AGENT {
        let x = (HOME_XMAX + HOME_XMIN) / 2;
        let y = (HOME_YMAX + HOME_YMIN) / 2;

        let loc = Int2D { x, y };

        let mut transform = Transform::default();
        transform.set_translation_xyz(x as f32, y as f32, 0.);
        transform.set_scale(Vector3::new(2., 2., 1.));
        let mut agent_adapter = AgentAdapter::new(ant_id, loc, false, 1.); // Reward=1. to deposit home pheromones
        ants_grid.grid.set_object_location(&mut agent_adapter, loc);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(global_tint)
            .with(transform)
            .with(agent_adapter)
            .build();
    }
}

// Load the assets asynchronously.
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "pixel.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "pixel_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}