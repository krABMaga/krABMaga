use crate::engine::{schedule::Schedule, state::State};

use crate::visualization::visualization_state::VisualizationState;

use std::sync::{Arc, Mutex};

use std::marker::PhantomData;

// A wrapper of the currently active state, used as a Bevy resource.
pub struct ActiveState<S: State>(pub Arc<Mutex<S>>);
// A wrapper of the currently active schedule, used as a Bevy resource.
pub struct ActiveSchedule(pub Arc<Mutex<Schedule>>);
// Initialization method to set up state and agents, wrapped as a Bevy resource.
pub struct Initializer<I: VisualizationState<S> + 'static, S: State>(
    pub I,
    pub PhantomData<Arc<Mutex<S>>>,
);
