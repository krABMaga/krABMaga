use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use crate::bevy::ecs::system::Resource;
use crate::engine::{schedule::Schedule, state::State};
use crate::visualization::visualization_state::VisualizationState;

// A wrapper of the currently active state, used as a Bevy resource.
#[derive(Resource)]
pub struct ActiveState<S: State>(pub Arc<Mutex<S>>);

// A wrapper of the currently active schedule, used as a Bevy resource.
#[derive(Resource)]
pub struct ActiveSchedule(pub Arc<Mutex<Schedule>>);

// Initialization method to set up state and agents, wrapped as a Bevy resource.
#[derive(Resource)]
pub struct Initializer<I: VisualizationState<S> + 'static + bevy::prelude::Resource, S: State>(
    pub I,
    pub PhantomData<Arc<Mutex<S>>>,
);
