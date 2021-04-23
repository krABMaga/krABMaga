use bevy::prelude::ResMut;

use crate::engine::agent::Agent;
use crate::engine::schedule::Schedule;
use std::hash::Hash;

/// The simulation system steps the schedule once per frame, effectively synchronizing frames and schedule steps.
pub fn simulation_system<A: 'static + Agent + Clone + Send + Sync + Hash + Eq>(
    mut schedule: ResMut<Schedule<A>>,
    mut state: ResMut<A::SimState>,
) {
    schedule.step(&mut *state);
}
