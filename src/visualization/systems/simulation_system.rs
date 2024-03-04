use bevy::prelude::ResMut;

use crate::bevy::prelude::Res;
use crate::engine::state::State;
use crate::visualization::{
    simulation_descriptor::SimulationDescriptor,
    wrappers::{ActiveSchedule, ActiveState},
};

/// The simulation system steps the schedule once per frame, effectively synchronizing frames and schedule steps.
pub fn simulation_system<S: State>(
    schedule_wrapper: ResMut<ActiveSchedule>,
    state_wrapper: ResMut<ActiveState<S>>,
    sim_data: Res<SimulationDescriptor>,
) {
    if !sim_data.paused {
        schedule_wrapper
            .0
            .lock()
            .expect("error on lock")
            .step(&mut *(*state_wrapper).0.lock().expect("error on lock"));
    }
}
