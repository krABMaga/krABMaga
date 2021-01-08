use crate::visualization::on_state_init::OnStateInit;
use crate::visualization::sprite_render_factory::SpriteRenderFactory;
use amethyst::assets::{Directory, Loader};
use amethyst::core::Transform;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::Camera;
use amethyst::utils::application_root_dir;
use amethyst::{GameData, SimpleState, StateData};
use std::path::{Path, PathBuf};

pub struct VisualizationState {
    width: f32,
    height: f32,
    on_state_init: Box<dyn OnStateInit>,
    sprite_render_factory: SpriteRenderFactory,
}

impl VisualizationState {
    pub fn new(width: f32, height: f32, on_state_init: Box<dyn OnStateInit>) -> VisualizationState {
        VisualizationState {
            width,
            height,
            on_state_init: on_state_init,
            sprite_render_factory: SpriteRenderFactory::new(),
        }
    }

    fn initialize_camera(&self, world: &mut World) {
        let mut transform = Transform::default();
        let (window_dimension_width, window_dimension_height) =
            (self.width + 100., self.height + 100.);
        // TODO: not correct, w and h are the window dimensions!
        let (simulation_dimension_width, simulation_dimension_height) = (self.width, self.height);

        // Make the camera target a slightly bigger area, and offset it a bit to center the Field2D.
        transform.set_translation_xyz(
            (window_dimension_width * 0.5)
                - (window_dimension_width - simulation_dimension_width as f32) / 2.,
            (window_dimension_height * 0.5)
                - (window_dimension_height - simulation_dimension_height as f32) / 2.,
            1.,
        );

        let width =
            window_dimension_width + (window_dimension_width - simulation_dimension_width as f32);
        let height = window_dimension_height
            + (window_dimension_height - simulation_dimension_height as f32);

        world
            .create_entity()
            .with(Camera::standard_2d(width * 0.35, height * 0.35))
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
            .join("assets");
        // TODO remove: debug
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
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
