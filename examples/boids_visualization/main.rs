mod model;
mod visualization;

use crate::model::bird::Bird;
use crate::model::boids_state::{HEIGHT, WIDTH};
use crate::visualization::vis_state::VisState;
use abm::visualization::visualization::Visualization;
use abm::visualization::visualization_state::VisualizationState;
use amethyst::utils::application_root_dir;
use std::path::PathBuf;

static NUM_AGENT: u128 = 1;

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?
        .join("examples")
        .join("boids_visualization");
    // TODO remove: debug
    let app_root =
        PathBuf::from("C:/Users/frafo/Desktop/Tirocinio/abm/examples/boids_visualization");
    println!("App_root: {}", app_root.to_str().unwrap());

    let visualization_state =
        VisualizationState::new(WIDTH as f32, HEIGHT as f32, Box::new(VisState));
    //initialize_agents(&mut state, &mut schedule, &mut visualization_state);

    Visualization::<Bird>::default()
        .with_custom_root_dir(app_root)
        .start(visualization_state)
}

/*
fn initialize_agents(
    state: &mut BoidsState,
    schedule: &mut Schedule<Bird>,
    vis_state: &mut VisualizationState,
) {
    let mut rng = rand::thread_rng();
    for bird_id in 0..NUM_AGENT {
        let r1: f64 = rng.gen();
        let r2: f64 = rng.gen();
        let last_d = Real2D { x: 0., y: 0. };
        let bird = Bird::new(
            bird_id,
            Real2D {
                x: WIDTH * r1,
                y: HEIGTH * r2,
            },
            last_d,
        );
        state
            .field1
            .lock()
            .unwrap()
            .set_object_location(bird, bird.pos);

        schedule.schedule_repeating(bird, 0., 0);
        vis_state.setup_visualization(Box::new(bird));
    }
}
*/
