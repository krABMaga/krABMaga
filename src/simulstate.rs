//use std::collections::HashMap;
use crate::field2D::Field2D;
use crate::location::Location2D;

pub struct SimState<A: Location2D + Clone>{
    pub field: Field2D<A>,
}

impl<A: Location2D + Clone> SimState<A> {
    pub fn new() -> SimState<A> {
        SimState {
            field: Field2D::new(),
        }
    }
}
