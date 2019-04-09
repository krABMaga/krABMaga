//use std::collections::HashMap;
use crate::field2D::Field2D;
use crate::location::Location2D;

pub struct SimState<P: Location2D>{
    pub field: Field2D<P>,
}

impl <P: Location2D> SimState<P> {
    pub fn new() -> SimState<P> {
        SimState {
            field: Field2D::new(),
        }
    }
}
