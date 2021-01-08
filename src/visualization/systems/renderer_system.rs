use crate::agent::Agent;
use crate::visualization::renderable::Render;
use amethyst::core::ecs::{Join, ReadStorage, WriteStorage};
use amethyst::core::Transform;
use amethyst::shred::System;
use std::marker::PhantomData;

pub struct RendererSystem<T: 'static + Agent + Render + Clone + Send + Sync> {
    pub marker: PhantomData<T>,
}

impl<'s, T: 'static + Agent + Render + Clone + Send + Sync> System<'s> for RendererSystem<T> {
    type SystemData = (WriteStorage<'s, T>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut render_objects, mut transforms): Self::SystemData) {
        for (render_object, transform) in (&mut render_objects, &mut transforms).join() {
            println!("Calling update!");
            render_object.update(transform);
        }
    }
}
