use crate::{
    agent_adapter::AgentAdapter, resources::AntsGrid, resources::ObstaclesGrid,
    resources::SitesGrid, resources::ToFoodGrid, resources::ToHomeGrid,
    static_object::StaticObject, static_object::StaticObjectType,
};
use abm::location::Int2D;
use amethyst::renderer::Transparent;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector3,
    core::transform::Transform,
    prelude::*,
    renderer::{
        palette::Srgba, resources::Tint, Camera, ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture,
    },
};
use rand::Rng;

// Constants regarding the window and the Field2D.
pub const WIDTH: i64 = 200;
pub const HEIGHT: i64 = 200;
pub const FIELD_WIDTH: f32 = WIDTH as f32 + 300.0;
pub const FIELD_HEIGHT: f32 = HEIGHT as f32 + 300.0;
pub const NUM_AGENT: u128 = 500;
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

/// Black
pub const ANT_RGBA: (f32, f32, f32, f32) = (0., 0., 0., 1.);

/// Event used to update the tint's alpha
#[derive(Debug)]
pub enum TintEvent {
    UpdateTint(u32, f64, bool),
}

/// Our simulation's first and only state.
pub struct Environment;

impl SimpleState for Environment {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // Load the required graphical assets for our simulation.
        let sprite_sheet_handle = load_sprite_sheet(world);
        let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

        // Register our StaticObject struct.
        // Needed because the component is not used within any system's data
        world.register::<StaticObject>();

        // Initialize the grids
        let mut ants_grid = AntsGrid::new(WIDTH, HEIGHT);
        let mut obstacles_grid = ObstaclesGrid::new(WIDTH, HEIGHT);
        let mut sites_grid = SitesGrid::new(WIDTH, HEIGHT);
        let mut to_food_grid = ToFoodGrid::new(WIDTH, HEIGHT);
        let mut to_home_grid = ToHomeGrid::new(WIDTH, HEIGHT);

        initialize_camera(world);
        let ants_spawnpoint = initialize_static_objects(
            world,
            sprite_render.clone(),
            &mut sites_grid,
            &mut obstacles_grid,
            &mut to_food_grid,
            &mut to_home_grid,
        );
        initialize_ants(
            world,
            sprite_render.clone(),
            &mut ants_grid,
            ants_spawnpoint,
        );

        // Insert the grids in the world as resources, so they can be fetched (in read or write mode).
        // There's no need for a mutex, Amethyst will handle synchronization for us by checking what Systems require.
        world.insert(ants_grid);
        world.insert(obstacles_grid);
        world.insert(sites_grid);
        world.insert(to_food_grid);
        world.insert(to_home_grid);
    }
}

/// Initialize the camera_2d object to point at the simulation by centering it in the window.
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    // Make the camera target a slightly bigger area, and offset it a bit to center the Field2D.
    transform.set_translation_xyz(
        (FIELD_WIDTH * 0.5) - (FIELD_WIDTH - WIDTH as f32) / 2.,
        (FIELD_HEIGHT * 0.5) - (FIELD_HEIGHT - HEIGHT as f32) / 2.,
        1.,
    );

    let width = FIELD_WIDTH + (FIELD_WIDTH - WIDTH as f32);
    let height = FIELD_HEIGHT + (FIELD_HEIGHT - HEIGHT as f32);

    world
        .create_entity()
        .with(Camera::standard_2d(width * 0.35, height * 0.35))
        .with(transform)
        .build();
}

/// Initialize the simulation's static objects: food sources, nests and obstacles, along with the tints
/// for each object and tints for pheromones with a starting alpha value of 0.
fn initialize_static_objects(
    world: &mut World,
    sprite_render: SpriteRender,
    sites_grid: &mut SitesGrid,
    obstacles_grid: &mut ObstaclesGrid,
    to_food_grid: &mut ToFoodGrid,
    to_home_grid: &mut ToHomeGrid,
) -> Int2D {
    let nest_loc = initialize_home_sites(world, sites_grid, sprite_render.clone());
    initialize_food_sites(world, sites_grid, sprite_render.clone());

    /* General formula to calculate an ellipsis, used to draw obstacles.
       x and y define a specific cell
       horizontal and vertical define the ellipsis position (bottom left: 0,0)
       size defines the ellipsis' size (smaller value = bigger ellipsis)
    */
    let ellipsis = |x: f32, y: f32, horizontal: f32, vertical: f32, size: f32| -> bool {
        ((x - horizontal) * size + (y - vertical) * size)
            * ((x - horizontal) * size + (y - vertical) * size)
            / 36.
            + ((x - horizontal) * size - (y - vertical) * size)
                * ((x - horizontal) * size - (y - vertical) * size)
                / 1024.
            <= 1.
    };

    // Generate 1x1 tints that represent pheromones. We save the entity index in the grid to be able to
    // send an event containing said index and a f32 representing the pheromone's intensity, to update the tint.
    // Generate the obstacles too in the same loop to avoid looping the grid twice uselessly.
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let tint = Tint(Srgba::new(0., 0., 0., 0.));
            let mut transform = Transform::default();
            transform.set_translation_xyz(x as f32, y as f32, -0.1);
            transform.set_scale(Vector3::new(2., 2., 1.));
            let cell = world
                .create_entity()
                .with(sprite_render.clone())
                .with(tint)
                .with(transform)
                .with(Transparent) // Required to make the alpha actually do something
                .build();
            to_food_grid
                .grid
                .set_value_at_pos(&Int2D { x, y }, (cell.id(), 0.));
            to_home_grid
                .grid
                .set_value_at_pos(&Int2D { x, y }, (cell.id(), 0.));

            // Generates elliptic obstacles (first one's top, second's bottom)
            if ellipsis(x as f32, y as f32, 100., 145., 0.407)
                || ellipsis(x as f32, y as f32, 90., 55., 0.407)
            {
                let obstacle = StaticObject::new(1, Int2D { x, y }, StaticObjectType::OBSTACLE);
                obstacles_grid
                    .grid
                    .set_value_at_pos(&Int2D { x, y }, obstacle);
                // Generate the entity for visualization
                let mut transform = Transform::default();
                transform.set_translation_xyz(x as f32, y as f32, 0.);
                world
                    .create_entity()
                    .with(sprite_render.clone())
                    .with(obstacle)
                    .with(transform)
                    .with(Tint(Srgba::new(0.5, 0.25, 0.25, 1.))) // Brown
                    .build();
            }
        }
    }

    nest_loc // Used as a spawnpoint for ants
}

/// Initialize the home site, in a random location in the square defined by the HOME_X/YMIN/MAX consts
fn initialize_home_sites(
    world: &mut World,
    sites_grid: &mut SitesGrid,
    sprite_render: SpriteRender,
) -> Int2D {
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
    sites_grid.grid.locs[x as usize][y as usize] = Some(nest);

    let mut transform = Transform::default();
    transform.set_translation_xyz(x as f32, y as f32, 0.);
    transform.set_scale(Vector3::new(2., 2., 1.));

    let tint = Tint(Srgba::new(1., 0., 0., 1.));
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(tint)
        .with(transform)
        .with(nest)
        .build();
    loc // Used as a spawnpoint for the ants
}

/// Initialize the food source, in a random location in the square defined by the FOOD_X/YMIN/MAX consts
fn initialize_food_sites(
    world: &mut World,
    sites_grid: &mut SitesGrid,
    sprite_render: SpriteRender,
) {
    let mut rng = rand::thread_rng();

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
    sites_grid.grid.locs[x as usize][y as usize] = Some(food);

    let mut transform = Transform::default();
    transform.set_translation_xyz(x as f32, y as f32, 0.);
    transform.set_scale(Vector3::new(2., 2., 1.));

    let tint = Tint(Srgba::new(1., 0., 0., 1.));
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(tint)
        .with(transform)
        .with(food)
        .build();
}

/// Initialize the ants by spawning them on the nest
fn initialize_ants(
    world: &mut World,
    sprite_render: SpriteRender,
    ants_grid: &mut AntsGrid,
    ants_spawnpoint: Int2D,
) {
    let (r, g, b, a) = ANT_RGBA;
    let global_tint = Tint(Srgba::new(r, g, b, a));

    for ant_id in 0..NUM_AGENT {
        let x = ants_spawnpoint.x;
        let y = ants_spawnpoint.y;

        let loc = Int2D { x, y };

        let mut transform = Transform::default();
        transform.set_translation_xyz(x as f32, y as f32, 0.);
        transform.set_scale(Vector3::new(2., 2., 1.));
        // Reward set to 1 to deposit home pheromones
        let mut agent_adapter = AgentAdapter::new(ant_id, loc, false, 1.);
        ants_grid.grid.set_object_location(&mut agent_adapter, &loc);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(global_tint)
            .with(transform)
            .with(agent_adapter)
            .build();
    }
}

/// Load the assets asynchronously.
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load("pixel.png", ImageFormat::default(), (), &texture_storage)
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
