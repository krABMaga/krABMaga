use krabmaga::engine::agent::Agent;
use krabmaga::engine::components::double_buffer::{DBRead, DBWrite};
use krabmaga::engine::components::position::Real2DTranslation;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::resources::engine_configuration::EngineConfiguration;
use krabmaga::engine::rng::RNG;
use krabmaga::engine::simulation::Simulation;
use krabmaga::engine::{Component, Query, Res};

pub static STEPS: u32 = 500;
pub static DIM_X: f32 = 200.;
pub static DIM_Y: f32 = 200.;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;
pub static NUM_AGENTS: u32 = 100;
pub static SEED: u64 = 1337;

#[derive(Clone, Copy, Component)]
pub struct MyAgent {
    pub id: u32,
}

fn step_handler(
    mut query: Query<(
        &MyAgent,
        &DBRead<Real2DTranslation>,
        &mut DBWrite<Real2DTranslation>,
    )>,
    config: Res<EngineConfiguration>,
) {
    println!("Handling step #{}", config.current_step);
    for (agent_info, cur_pos, mut next_pos) in query.iter_mut() {
        next_pos.0 .0.x = cur_pos.0 .0.x + 1.;
        next_pos.0 .0.y = cur_pos.0 .0.y + 1.;
        println!(
            "Agent {}\nOld position: ({}, {})\nNew position: ({}, {})",
            agent_info.id, cur_pos.0 .0.x, cur_pos.0 .0.y, next_pos.0 .0.x, next_pos.0 .0.y
        );
    }
}

fn init(simulation: &mut Simulation) {
    for id in 0..NUM_AGENTS {
        let mut rng = RNG::new(SEED, id as u64);
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();

        let position = Real2D {
            x: DIM_X * r1,
            y: DIM_Y * r2,
        };
        let current_pos = Real2DTranslation(position);

        let mut agent = Agent::new(simulation);

        agent
            .insert_data(MyAgent { id })
            .insert_double_buffered(current_pos);
    }
}

fn main() {
    let mut simulation = Simulation::build()
        .register_step_handler(step_handler)
        .register_double_buffer::<Real2DTranslation>()
        .with_steps(10)
        .with_engine_configuration(EngineConfiguration::new(Real2D { x: 200.0, y: 200.0 }, 0));
    init(&mut simulation);

    simulation.run();
}
