use crate::environment::TintEvent;
use crate::environment::TintEvent::UpdateTint;
use amethyst::core::ecs::shred::SystemData;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Entities, Read, ReaderId, System, WriteStorage};
use amethyst::prelude::World;
use amethyst::renderer::resources::Tint;

/// Handles updating the tint of the pheromones. The potency of the pheromone decides the tint's alpha,
/// home pheromones are displayed as green and food pheromones as blue. The update is handled with an
/// event system, everytime a pheromone's value is changed it fires off an UpdateTint event which this system
/// reads and acts according to it.
pub struct PheromoneTintHandler {
    reader_id: ReaderId<TintEvent>,
}

impl PheromoneTintHandler {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        // Register to the event channel as a reader
        let reader_id = world
            .fetch_mut::<EventChannel<TintEvent>>()
            .register_reader();
        Self { reader_id }
    }
}

impl<'s> System<'s> for PheromoneTintHandler {
    type SystemData = (
        Read<'s, EventChannel<TintEvent>>,
        Entities<'s>,
        WriteStorage<'s, Tint>,
    );

    fn run(&mut self, (event_channel, entities, mut tint_storage): Self::SystemData) {
        for event in event_channel.read(&mut self.reader_id) {
            let UpdateTint(index, value, has_food) = event;
            if *value == 0. {
                continue;
            }
            // Fetch the tint related to the index passed with the event
            let entity = entities.entity(*index);
            let mut tint = tint_storage.get_mut(entity).unwrap();
            if !*has_food && tint.0.blue == 1. {
                // Do not overwrite food pheromones with home ones
                continue;
            }
            // Food pheromones are blue, home ones are green
            if *has_food {
                tint.0.green = 0.;
                tint.0.blue = 1.;
            } else {
                tint.0.green = 1.;
                tint.0.blue = 0.;
            }

            tint.0.alpha = *value as f32;
        }
    }
}
