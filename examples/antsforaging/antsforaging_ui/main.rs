use crate::model::ant::Ant;
use crate::visualization::custom_system_bundle::CustomSystemBundle;
use crate::visualization::vis_state::VisState;
use amethyst::renderer::palette::Srgba;
use amethyst::utils::application_root_dir;
use rust_ab::visualization::visualization::Visualization;
use rust_ab::visualization::visualization_state::VisualizationState;

pub mod model;
pub mod visualization;

pub const WIDTH: i64 = 200;
pub const HEIGHT: i64 = 200;
pub const NUM_AGENT: u128 = 500;
pub const EVAPORATION: f64 = 0.999;
pub const STEP: u128 = 50000;
// Nest coordinate range
pub const HOME_XMIN: i64 = 175;
pub const HOME_XMAX: i64 = 175;
pub const HOME_YMIN: i64 = 175;
pub const HOME_YMAX: i64 = 175;
// Food coordinate range
pub const FOOD_XMIN: i64 = 25;
pub const FOOD_XMAX: i64 = 25;
pub const FOOD_YMIN: i64 = 25;
pub const FOOD_YMAX: i64 = 25;

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?
        .join("examples")
        .join("antsforaging")
        .join("antsforaging_ui");
    /* CLion debugger starts the built executable directly from the target folder, so the path must
            be hardcoded like this to be able to debug simulations.
    let app_root = PathBuf::from(
        "C:\\Users\\frafo\\Desktop\\Tirocinio\\abm\\examples\\antsforaging\\antsforaging_ui",
    );*/

    let visualization_state =
        VisualizationState::new(WIDTH as f32, HEIGHT as f32, Box::new(VisState));

    Visualization::<Ant>::default()
        .with_background_color(Srgba::new(255., 255., 255., 1.))
        .with_custom_root_dir(app_root)
        .start_with_custom_bundle(visualization_state, Some(CustomSystemBundle))
}
