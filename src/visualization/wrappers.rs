use crate::engine::{schedule::Schedule, state::State};

use crate::bevy::ecs::system::Resource;
use crate::visualization::visualization_state::VisualizationState;
use std::sync::{Arc, Mutex};

use std::marker::PhantomData;

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