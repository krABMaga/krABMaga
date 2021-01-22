mod model;
mod visualization;

use crate::model::bird::Bird;
use crate::model::boids_state::{HEIGHT, WIDTH};
use crate::visualization::vis_state::VisState;
use amethyst::renderer::palette::Srgba;
use amethyst::utils::application_root_dir;
use rust_ab::visualization::visualization::Visualization;
use rust_ab::visualization::visualization_state::VisualizationState;

static NUM_AGENT: u128 = 500;

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?
        .join("examples")
        .join("boids")
        .join("boids_ui");

    let visualization_state =
        VisualizationState::new(WIDTH as f32, HEIGHT as f32, Box::new(VisState));

    Visualization::<Bird>::default()
        // Black background
        .with_background_color(Srgba::new(0., 0., 0., 1.))
        .with_custom_root_dir(app_root)
        .start(visualization_state)
}
