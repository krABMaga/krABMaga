use bevy::prelude::*;

use crate::engine::resources::engine_configuration::EngineConfiguration;

pub fn engine_config_update(mut engine_config: ResMut<EngineConfiguration>) {
    engine_config.current_step += 1;
}
