// Bevy reexports, so that we can prevent exposing bevy directly
// TODO can we simplify those for the user without sacrificing flexibility?
pub use bevy::ecs as bevy_ecs;
pub use bevy::prelude::Component;
pub use bevy::prelude::Entity;
pub use bevy::prelude::Query;
pub use bevy::prelude::Res;
pub use bevy::prelude::ResMut;

/// Module to define Agent methods
pub mod agent;

/// Folder containing all the fields available on the engine
pub mod fields;
/// File to define the basic structs for locations of the agents used for Fields
pub mod location;

pub mod components;
pub mod resources;
pub mod rng;
pub mod simulation;
/// Module to define State methods
pub mod state;
pub mod systems;

// TODO consider removing/abstracting away
