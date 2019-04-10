use crate::simulstate::SimState;
//use std::hash::{Hash};
use crate::location::Location2D;
pub trait Agent{
    fn step(self, simstate: &SimState);
}
