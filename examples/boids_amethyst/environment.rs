use crate::agent_adapter::AgentAdapter;
use abm::{field_2d::Field2D, location::Real2D};
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
pub const FIELD_HEIGHT: f32 = 500.;
pub const FIELD_WIDTH: f32 = 500.;
pub const WIDTH: f64 = 400.;
pub const HEIGHT: f64 = 400.;
pub const DISCRETIZATION: f64 = 10. / 1.5;
pub const TOROIDAL: bool = true;
pub const NUM_AGENT: u128 = 250;
pub const DEAD_PROB: f64 = 0.1;
// Red
pub const LIVE_FLOCKER_RGBA: (f32, f32, f32, f32) = (255., 0., 0., 1.);
// Black
pub const DEAD_FLOCKER_RGBA: (f32, f32, f32, f32) = (0., 0., 0., 1.);

pub struct Environment;

impl SimpleState for Environment {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        let mut field: Field2D<AgentAdapter> =
            Field2D::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL);

        initialize_camera(world);
        initialize_flockers(world, sprite_sheet_handle, &mut field);

        // Insert the Field2D in the world as a resource, so it can be fetched (in read or write mode).
        // There's no need for a mutex, Amethyst will handle synchronization for us by checking what Systems require.
        world.insert(field);
    }
}

/// Initializes a simple camera for a 2d scenario.
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    // Make the camera target a slightly bigger area, and offset it a bit to center the Field2D.
    transform.set_translation_xyz((FIELD_WIDTH * 0.5) - 50., (FIELD_HEIGHT * 0.5) - 50., 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(FIELD_WIDTH + 100., FIELD_HEIGHT + 100.))
        .with(transform)
        .build();
}

/// Initialize the flockers randomly on the field, with the previously loaded sprite and
/// a tint to differentiate the dead ones from the alive.
fn initialize_flockers(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    field: &mut Field2D<AgentAdapter>,
) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    let (r, g, b, a) = LIVE_FLOCKER_RGBA;
    let global_tint = Tint(Srgba::new(r, g, b, a));
    let (r, g, b, a) = DEAD_FLOCKER_RGBA;
    let dead_tint = Tint(Srgba::new(r, g, b, a));

    let mut rng = rand::thread_rng();
    for bird_id in 0..NUM_AGENT {
        let mut tint_to_use = global_tint;
        let x: f64 = WIDTH * rng.gen::<f64>();
        let y: f64 = HEIGHT * rng.gen::<f64>();

        let last_d = Real2D { x: 0., y: 0. };

        let mut transform = Transform::default();
        transform.set_translation_xyz(x as f32, y as f32, 0.);
        // Sprite size is 64x64, we scale it down.
        transform.set_scale(Vector3::new(0.15, 0.15, 1.));
        // Chance for the flocker to be dead from the start of the simulation
        let dead = rng.gen_bool(DEAD_PROB);
        if dead {
            tint_to_use = dead_tint;
        }
        // An adapter that will handle communication with the RustAB framework, mainly to fetch the neighbor agents
        let agent_adapter = AgentAdapter::new(bird_id, Real2D { x, y }, last_d, dead);

        field.set_object_location(agent_adapter, agent_adapter.pos);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(tint_to_use)
            .with(transform)
            .with(agent_adapter)
            .build();
    }
}

/// Loads the assets asynchronously. Load an image as a texture, then load the single sprite
/// contained within the texture through the .ron file specifying position and such.
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load("flocker.png", ImageFormat::default(), (), &texture_storage)
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "flocker_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
