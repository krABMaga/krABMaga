use bevy::prelude::Resource;

use crate::engine::location::Real2D;

/// Specifies the krABMaga configuration, always present for any kind of simulation developed with this framework.
#[derive(Resource)]
pub struct EngineConfiguration {
    pub current_step: u32,
    pub simulation_dim: Real2D,
    pub paused: bool,
    pub rand_seed: u64,
}

impl EngineConfiguration {
    pub fn new(simulation_dim: Real2D, rand_seed: u64) -> Self {
        EngineConfiguration {
            current_step: 0,
            simulation_dim,
            paused: false,
            rand_seed,
        }
    }
}
