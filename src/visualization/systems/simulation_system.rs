use crate::engine::agent::Agent;
use crate::visualization::renderable::Render;
use crate::Schedule;
use amethyst::core::ecs::storage::DistinctStorage;
use amethyst::core::ecs::Component;
use amethyst::shred::{System, WriteExpect};
use std::marker::PhantomData;

/// The Simulation System handles the communication from the visualization system to the simulation system,
/// aka RustAB, which simply relies on the schedule, added as an Amethyst Resource, which is fetched as
/// mutable and a single step is done per frame, by passing it the simulation state, which is also maintained
/// as an Amethyst Resource.
pub struct SimulationSystem<T>
where
    T: 'static + Agent + Render + Clone + Send + Sync,
    <T as Component>::Storage: DistinctStorage,
{
    pub marker: PhantomData<T>,
}

impl<'s, T> System<'s> for SimulationSystem<T>
where
    T: 'static + Agent + Render + Clone + Send + Sync,
    <T as Component>::Storage: DistinctStorage,
{
    type SystemData = (WriteExpect<'s, Schedule<T>>, WriteExpect<'s, T::SimState>);

    fn run(&mut self, (mut schedule, mut state): Self::SystemData) {
        schedule.step(&mut state);
    }
}
