use crate::engine::agent::Agent;
use crate::visualization::renderable::Render;
use amethyst::core::ecs::storage::DistinctStorage;
use amethyst::core::ecs::{Component, Join, WriteStorage};
use amethyst::core::Transform;
use amethyst::shred::{ReadExpect, System};
use std::marker::PhantomData;

/// The Renderer System handles the iteration, for each frame, of the graphics update() method of our
/// primary agent of our simulation.
pub struct RendererSystem<T>
where
    T: 'static + Agent + Render + Clone + Send + Sync,
    <T as Component>::Storage: DistinctStorage,
{
    pub marker: PhantomData<T>,
}

impl<'s, T> System<'s> for RendererSystem<T>
where
    T: 'static + Agent + Render + Clone + Send + Sync,
    <T as Component>::Storage: DistinctStorage,
{
    type SystemData = (
        WriteStorage<'s, T>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, T::SimState>,
    );

    fn run(&mut self, (mut render_objects, mut transforms, state): Self::SystemData) {
        // TODO: implement parallel join, currently unfeasible due to transforms storage not implementing DistinctStorage
        /*(&mut transforms, &mut render_objects)
        .par_join()
        .for_each(|(transform, render_object)| {
            render_object.update(transform, &*state);
        });*/

        for (render_object, transform) in (&mut render_objects, &mut transforms).join() {
            render_object.update(transform, &*state);
        }
    }
}
