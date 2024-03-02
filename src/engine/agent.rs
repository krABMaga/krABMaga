use bevy::prelude::{Component, EntityWorldMut};

use crate::engine::components::double_buffer::DoubleBuffered;
use crate::engine::simulation::Simulation;

#[derive(Component)]
pub struct Agent;

/// A helper struct to properly create agents and add data to it with the ECS architecture
pub struct AgentFactory<'w> {
    entity: EntityWorldMut<'w>,
}

impl<'w> AgentFactory<'w> {
    /// Spawn a new agent in the simulation.
    pub fn new(simulation: &'w mut Simulation) -> Self {
        let mut entity = simulation.spawn_agent();
        entity.insert((Agent,));
        AgentFactory { entity }
    }

    /// Insert constant data associated to this agent. We assume this data will not be changed by user defined systems, so we can avoid keeping a copy of the original data, for example to perform a reset of the simulation.
    pub fn insert_const<T: Component>(&mut self, value: T) -> &mut Self {
        self.entity.insert(value);
        self
    }

    /// Insert data associated to this agent. This method stores a readonly copy of the original data to be used to reset the simulation.
    pub fn insert_data<T: Component + Copy>(&mut self, value: T) -> &mut Self {
        self.entity.insert(value);
        // TODO
        // #[cfg(feature = "reset_simulation")]
        // self.entity.insert(Original(value));
        self
    }

    /// Insert double buffered data associated to this agent. This method automatically generates two buffers that the user can use, one to read values from and one to write updated values in.
    pub fn insert_double_buffered<T: Component + Copy>(&mut self, value: T) -> &mut Self {
        self.entity.insert(DoubleBuffered::new(value));
        // #[cfg(feature = "reset_simulation")]
        // self.entity.insert(Original(value));
        self
    }
}
