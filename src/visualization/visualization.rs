use crate::agent::Agent;
use crate::visualization::main_system_bundle::MainSystemBundle;
use crate::visualization::renderable::Render;
use crate::visualization::visualization_state::VisualizationState;
use amethyst::core::TransformBundle;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle};
use amethyst::utils::application_root_dir;
use amethyst::{Application, GameDataBuilder};
use std::marker::PhantomData;
use std::path::PathBuf;

// TODO: consider converting this into a trait, and replace T with an associated type
pub struct Visualization<T: 'static + Agent + Render + Clone + Send + Sync> {
    root_path: PathBuf,
    background_color: Srgba,
    marker: PhantomData<T>,
}

impl<T: 'static + Agent + Render + Clone + Send + Sync> Visualization<T> {
    pub fn with_custom_root_dir(mut self, path: PathBuf) -> Visualization<T> {
        self.root_path = path;
        self
    }

    pub fn with_background_color(mut self, background_color: Srgba) -> Visualization<T> {
        self.background_color = background_color;
        self
    }

    pub fn start(self, visualization_state: VisualizationState) -> amethyst::Result<()> {
        amethyst::start_logger(Default::default());

        // Window config such as size and window title
        let display_config_path = self.root_path.join("config").join("display.ron");

        // The folder containing our assets, graphical and ron spritesheet configs
        let assets_dir = self.root_path.join("assets");
        let (r, g, b, a) = self.background_color.into_components();
        let main_bundle: MainSystemBundle<T> = MainSystemBundle {
            marker: PhantomData,
        };

        let game_data = GameDataBuilder::default()
            .with_bundle(
                RenderingBundle::<DefaultBackend>::new()
                    .with_plugin(
                        RenderToWindow::from_config_path(display_config_path)?
                            .with_clear([r, g, b, a]), // white opaque
                    )
                    .with_plugin(RenderFlat2D::default()),
            )?
            .with_bundle(TransformBundle::new())?
            .with_bundle(main_bundle)?;

        let mut game = Application::new(assets_dir, visualization_state, game_data)?;
        game.run();

        Ok(())
    }
}

impl<T: 'static + Agent + Render + Clone + Send + Sync> Default for Visualization<T> {
    fn default() -> Self {
        Self {
            root_path: application_root_dir().unwrap(), // TODO: can we avoid using unwrap somehow?
            background_color: Srgba::new(255., 255., 255., 1.),
            marker: PhantomData,
        }
    }
}
