use bevy::prelude::Component;

// TODO: remove completely. Users will use granular bevy resources for data and startup? systems for logic.
#[derive(Component)]
pub struct State; // Marker
