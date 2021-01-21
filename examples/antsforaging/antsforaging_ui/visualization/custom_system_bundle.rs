use crate::visualization::pheromone_system::PheromoneSystem;
use amethyst::core::SystemBundle;
use amethyst::shred::{DispatcherBuilder, World};
use amethyst::Error;

/// An example of a custom bundle with a single custom system within it. This system will handle
/// the update of the graphics representing the simulation pheromones deposited by ants, every 200
/// frame to avoid putting a heavy load on the renderer considering all pheromones evaporate each frame.
pub struct CustomSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CustomSystemBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(PheromoneSystem, "pheromone_system", &[]);
        Ok(())
    }
}
