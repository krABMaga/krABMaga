use bevy::prelude::{Query, Res, Transform};

use crate::engine::agent::Agent;
use crate::visualization::renderable::Render;

/// The system that updates the visual representation of each agent of our simulation.
pub fn renderer_system<A: 'static + Agent + Render + Clone + Send>(
    mut query: Query<(&mut A, &mut Transform)>,
    state: Res<A::SimState>,
) {
    for (mut render, mut transform) in query.iter_mut() {
        render.update(&mut *transform, &state);
    }
}
