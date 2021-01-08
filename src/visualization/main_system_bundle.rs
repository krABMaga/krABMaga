use crate::agent::Agent;
use crate::visualization::renderable::Render;
use crate::visualization::systems::renderer_system::RendererSystem;
use crate::visualization::systems::simulation_system::SimulationSystem;
use amethyst::core::SystemBundle;
use amethyst::shred::{DispatcherBuilder, World};
use amethyst::Error;
use std::marker::PhantomData;

pub struct MainSystemBundle<T: 'static + Agent + Render + Clone + Send + Sync> {
    pub marker: PhantomData<T>,
}

impl<'a, 'b, T: 'static + Agent + Render + Clone + Send + Sync> SystemBundle<'a, 'b>
    for MainSystemBundle<T>
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let sim: SimulationSystem<T> = SimulationSystem {
            marker: PhantomData,
        };
        let rendering: RendererSystem<T> = RendererSystem {
            marker: PhantomData,
        };
        builder.add(sim, "simulation_system", &[]);
        // TODO: capire cosa clona i bird
        //builder.add(rendering, "renderer_system", &[]);
        Ok(())
    }
}
