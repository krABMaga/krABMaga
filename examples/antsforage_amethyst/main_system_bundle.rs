use crate::systems::{AntSystem, FPSSystem, PheromoneTintHandler};
use amethyst::core::ecs::DispatcherBuilder;
use amethyst::core::SystemBundle;
use amethyst::prelude::World;
use amethyst::Error;

/// A simple bundle containing all the systems of our simulation.
pub struct MainSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MainSystemBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(AntSystem, "ant_system", &[]);
        builder.add(FPSSystem { print_elapsed: 0. }, "fps", &[]);
        builder.add(
            PheromoneTintHandler::new(world),
            "pheromone_tint_handler",
            &[],
        );
        Ok(())
    }
}
