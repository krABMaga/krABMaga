use crate::schedule::Schedule;
use crate::agent::Agent;
use std::hash::{Hash};

pub struct SimState<A: Agent + Clone + Copy + Hash + Eq> {
    pub schedule: Schedule<A>,
}
