//use std::sync::{Arc, Mutex};

pub trait Agent {
    fn step(&mut self);
}

// pub trait Stat {
//     type I;
//     fn get_state(self) -> Self::I;
// }
