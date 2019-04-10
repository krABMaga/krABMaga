//use std::collections::HashMap;
use crate::field2D::Field2D;
use crate::location::Location2D;

pub struct SimState{
    pub field: Field2D,
}

impl SimState {
    pub fn new() -> SimState {
        SimState {
            field: Field2D::new(),
        }
    }
}
