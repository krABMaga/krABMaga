use crate::engine::agent::Agent;
use crate::visualization::renderable::Render;
use crate::Schedule;
use amethyst::shred::{System, WriteExpect};
use std::marker::PhantomData;

pub struct SimulationSystem<T: 'static + Agent + Render + Clone + Send + Sync> {
    pub marker: PhantomData<T>,
}

impl<'s, T: 'static + Agent + Render + Clone + Send + Sync> System<'s> for SimulationSystem<T> {
    type SystemData = (WriteExpect<'s, Schedule<T>>, WriteExpect<'s, T::SimState>);

    fn run(&mut self, (mut schedule, mut state): Self::SystemData) {
        schedule.step(&mut state);
    }
}
