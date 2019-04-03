use crate::simstate::SimState;
use std::hash::{Hash};

pub trait Agent{
    fn step<A: Agent + Clone + Copy + Hash + Eq>(self, simstate: &SimState<A>);
}
