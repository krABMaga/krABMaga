use amethyst::core::SystemBundle;
use amethyst::prelude::World;
use amethyst::core::ecs::DispatcherBuilder;
use amethyst::Error;
use crate::systems::{AntSystem, FPSSystem, TintHandler};

pub struct MainSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MainSystemBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(AntSystem, "ant_system", &[]);
        builder.add(FPSSystem { print_elapsed: 0. }, "fps", &[]);
        builder.add(TintHandler::new(world), "tint_handler", &[]);
        Ok(())
    }
}