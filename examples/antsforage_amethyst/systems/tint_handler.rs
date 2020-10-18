use amethyst::core::ecs::{ReaderId, System, Read, Entities, WriteStorage};
use crate::environment::TintEvent;
use amethyst::prelude::World;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::shred::SystemData;
use amethyst::renderer::resources::Tint;
use crate::environment::TintEvent::UpdateTint;

pub struct TintHandler {
    reader_id: ReaderId<TintEvent>
}

impl TintHandler {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<TintEvent>>().register_reader();
        Self { reader_id }
    }
}

impl<'s> System<'s> for TintHandler {
    type SystemData = (
        Read<'s, EventChannel<TintEvent>>,
        Entities<'s>,
        WriteStorage<'s, Tint>
    );

    fn run(&mut self, (event_channel, entities, mut tint_storage): Self::SystemData) {
        for event in event_channel.read(&mut self.reader_id) {
            let UpdateTint(index, value, has_food) = event;
            let entity = entities.entity(*index);
            let mut tint = tint_storage.get_mut(entity).unwrap();
            if !*has_food && tint.0.blue == 1. { // Do not overwrite food pheromones with home ones
                continue;
            }
            if *has_food {
                tint.0.green = 0.;
                tint.0.blue = 1.;
            } else {
                tint.0.green = 1.;
                tint.0.blue = 0.;
            }
            let value = (*value / 3.) as f32;
            tint.0.alpha = value.max(0.1); // Sets minimum alpha to 0.1 so that the pheromone actually shows up even in small intensities
        }
    }
}
