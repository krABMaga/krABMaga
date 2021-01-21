use crate::engine::agent::Agent;
use crate::visualization::renderable::Render;
use crate::visualization::systems::renderer_system::RendererSystem;
use crate::visualization::systems::simulation_system::SimulationSystem;
use amethyst::core::ecs::storage::DistinctStorage;
use amethyst::core::ecs::Component;
use amethyst::core::SystemBundle;
use amethyst::shred::{DispatcherBuilder, World};
use amethyst::Error;
use std::marker::PhantomData;

/// The main bundle inserted in the Amethyst App, composed of the Simulation and Renderer Systems.
pub struct MainSystemBundle<T>
where
    T: 'static + Agent + Render + Clone + Send + Sync,
    <T as Component>::Storage: DistinctStorage,
{
    pub marker: PhantomData<T>,
}

impl<'a, 'b, T> SystemBundle<'a, 'b> for MainSystemBundle<T>
where
    T: 'static + Agent + Render + Clone + Send + Sync,
    <T as Component>::Storage: DistinctStorage,
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
        builder.add(rendering, "renderer_system", &[]);
        Ok(())
    }
}
