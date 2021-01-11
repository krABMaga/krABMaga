use crate::visualization::on_state_init::OnStateInit;
use crate::visualization::sprite_render_factory::SpriteRenderFactory;
use amethyst::assets::{Directory, Loader};
use amethyst::core::Transform;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::Camera;
use amethyst::utils::application_root_dir;
use amethyst::window::ScreenDimensions;
use amethyst::{GameData, SimpleState, StateData};

pub struct VisualizationState {
    simulation_width: f32,
    simulation_height: f32,
    on_state_init: Box<dyn OnStateInit>,
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
            on_state_init: on_state_init,
            sprite_render_factory: SpriteRenderFactory::new(),
        }
    }

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
                window_width as f32,
                window_height as f32,
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
        let mut loader = world.write_resource::<Loader>();
        loader.add_source("visualization_framework", Directory::new(path));
        drop(loader);
        self.on_state_init
            .on_init(world, &mut self.sprite_render_factory);
    }
}
