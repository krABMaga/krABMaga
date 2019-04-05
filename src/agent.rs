use crate::simulstate::SimState;
//use std::hash::{Hash};

pub trait Agent{
    fn step(self, simstate: &SimState);
}
