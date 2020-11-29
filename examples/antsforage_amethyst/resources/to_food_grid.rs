use crate::environment::TintEvent;
use crate::environment::TintEvent::UpdateTint;
use abm::simple_grid_2d::SimpleGrid2D;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::ecs::world::Index;

/// Extremely low pheromone, under which the value gets rounded to 0
const LOW_PHEROMONE: f64 = 0.00000000000001;

/// Represents food pheromones. Higher f64 means more concentrated pheromone
pub struct ToFoodGrid {
    pub grid: SimpleGrid2D<(Index, f64)>,
}

impl ToFoodGrid {
    pub fn new(width: i64, height: i64) -> ToFoodGrid {
        ToFoodGrid {
            grid: SimpleGrid2D::new(width, height),
        }
    }

    /// Custom implementation of multiply, to fire off an UpdateTint event everytime the value of a
    /// pheromone is changed, to update the corresponding tint.
    pub fn multiply(&mut self, value: f64, event_channel: &mut EventChannel<TintEvent>) {
        for i in self.grid.locs.iter_mut() {
            for j in i.iter_mut() {
                if let Some((index, val)) = j {
                    *val *= value;
                    if *val < LOW_PHEROMONE {
                        *val = 0.
                    }
                    event_channel.single_write(UpdateTint(*index, *val, true));
                }
            }
        }
    }
}
