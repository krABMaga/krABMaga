use crate::visualization::on_state_init::OnStateInit;
use crate::visualization::sprite_render_factory::SpriteRenderFactory;
use amethyst::assets::{Directory, Loader};
use amethyst::core::Transform;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::Camera;
use amethyst::utils::application_root_dir;
use amethyst::window::ScreenDimensions;
use amethyst::{GameData, SimpleState, StateData};

/// A wrapper of the simulation state which allows setting up the graphical environment of our simulation.
pub struct VisualizationState {
    /// The width of the simulation, which will be used to define the camera's width.
    simulation_width: f32,
    /// The height of the simulation, which will be used to define the camera's height.
    simulation_height: f32,
    /// A custom struct that allows the user to define a process to be executed at the start of the visualization.
    /// Primary use is to create a number of agents.
    on_state_init: Box<dyn OnStateInit>,
    /// Passed to the on_state_init trait object to allow the user to fetch SpriteRenders for their agents.
    sprite_render_factory: SpriteRenderFactory,
}

impl VisualizationState {
    pub fn new(
        simulation_width: f32,
        simulation_height: f32,
        on_state_init: Box<dyn OnStateInit>,
    ) -> VisualizationState {
        VisualizationState {
            simulation_width,
            simulation_height,
            on_state_init,
            sprite_render_factory: SpriteRenderFactory::new(),
        }
    }

    /// Initialize the 2D camera by moving it in the correct position to look at the simulation as a whole.
    /// The camera will be automatically zoomed based on the simulation width and height, allowing the actual
    /// window to be larger and still visualize the simulation correctly.
    fn initialize_camera(&self, world: &mut World) {
        let mut transform = Transform::default();

        let (window_width, window_height) = {
            let window_dimensions = world.read_resource::<ScreenDimensions>();
            (window_dimensions.width(), window_dimensions.height())
        };
        // Make the camera target a slightly bigger area, and offset it a bit
        transform.set_translation_xyz(
            (window_width * 0.5) - (window_width - self.simulation_width as f32) / 2.,
            (window_height * 0.5) - (window_height - self.simulation_height as f32) / 2.,
            1.,
        );

        world
            .create_entity()
            .with(Camera::standard_2d(
                self.simulation_width as f32,
                self.simulation_height as f32,
            ))
            .with(transform)
            .build();
    }
}

impl SimpleState for VisualizationState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.initialize_camera(world);
        let path = application_root_dir()
            .unwrap()
            .join("src")
            .join("visualization")
            .join("assets")
            .join("emojis");
        /* CLion debugger starts the built executable directly from the target folder, so the path must
            be hardcoded like this to be able to debug simulations.
        let path = PathBuf::from(
            "C:\\Users\\frafo\\Desktop\\Tirocinio\\abm\\src\\visualization\\assets\\emojis",
        );*/
        let mut loader = world.write_resource::<Loader>();
        // This source will be used internally to fetch emojis
        loader.add_source("visualization_framework", Directory::new(path));
        drop(loader);
        self.on_state_init
            .on_init(world, &mut self.sprite_render_factory);
    }
}
