use crate::agent::Agent;
use crate::visualization::renderable::Render;
use crate::Schedule;
use amethyst::core::ecs::{Join, WriteStorage};
use amethyst::core::Transform;
use amethyst::shred::{System, WriteExpect};
use std::marker::PhantomData;

// TODO: supertrait T
pub struct SimulationSystem<T: 'static + Agent + Render + Clone + Send + Sync> {
    pub marker: PhantomData<T>,
}

impl<'s, T: 'static + Agent + Render + Clone + Send + Sync> System<'s> for SimulationSystem<T> {
    type SystemData = (
        WriteExpect<'s, Schedule<T>>,
        WriteExpect<'s, T::SimState>,
        WriteStorage<'s, T>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (mut schedule, mut state, mut render_objects, mut transforms): Self::SystemData,
    ) {
        println!(
            "Calling step! There are currently {} render objects",
            render_objects.count()
        );
        schedule.step(&mut state);
        for (render_object, transform) in (&mut render_objects, &mut transforms).join() {
            render_object.update(transform);
        }
    }
}
