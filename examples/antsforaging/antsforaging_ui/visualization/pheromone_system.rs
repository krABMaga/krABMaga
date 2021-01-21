use crate::model::state::State;
use crate::visualization::pheromone::Pheromone;
use amethyst::core::ecs::{ParJoin, ReadStorage, WriteStorage};
use amethyst::core::Time;
use amethyst::renderer::resources::Tint;
use amethyst::shred::{ReadExpect, System};
use rayon::iter::ParallelIterator;

/// Number of frames that must pass before this system gets triggered.
/// Most of the pheromones will evaporate slowly, so we can wait around 5 seconds before triggering a
/// graphics update.
const NUM_FRAMES: u64 = 200;

/// A custom system defined to update the pheromones covering the whole grid of the simulation, based on
/// the data structures of RustAB.
pub struct PheromoneSystem;

impl<'s> System<'s> for PheromoneSystem {
    type SystemData = (
        ReadExpect<'s, Time>,
        ReadStorage<'s, Pheromone>,
        WriteStorage<'s, Tint>,
        ReadExpect<'s, State>,
    );

    fn run(&mut self, (time, pheromones, mut tints, state): Self::SystemData) {
        if time.frame_number() % NUM_FRAMES == 0 {
            // Pheromones do not act upon each other, so we can safely parallelize this task for a small
            // performance benefit.
            (&pheromones, &mut tints)
                .par_join()
                .for_each(|(pheromone, tint)| {
                    // If there's a food pheromone in the simulation grid at this loc, it has a higher
                    // precedence over home pheromones
                    let food_pheromone_val = state.get_food_pheromone(&pheromone.loc);
                    if let Some(val) = food_pheromone_val {
                        tint.0.alpha = *val as f32;
                        tint.0.blue = 255.;
                        tint.0.green = 0.;
                        tint.0.red = 0.;
                    } else {
                        let home_pheromone_val = state.get_home_pheromone(&pheromone.loc);
                        if let Some(val) = home_pheromone_val {
                            tint.0.alpha = *val as f32;
                            tint.0.blue = 0.;
                            tint.0.green = 255.;
                            tint.0.red = 0.;
                        }
                    }
                });
        }
    }
}
