use crate::model::bird::Bird;
use abm::field::DoubleBufferedField;
use abm::state::State;
use abm::Field2D;
use std::sync::Mutex;

pub static WIDTH: f64 = 400.;
pub static HEIGHT: f64 = 400.;

pub static COHESION: f64 = 1.0;
pub static AVOIDANCE: f64 = 1.0;
pub static RANDOMNESS: f64 = 1.0;
pub static CONSISTENCY: f64 = 1.0;
pub static MOMENTUM: f64 = 1.0;
pub static JUMP: f64 = 0.7;
pub static DISCRETIZATION: f64 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

pub struct BoidsState {
    pub field1: Field2D<Bird>,
}

impl BoidsState {
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> BoidsState {
        BoidsState {
            field1: Field2D::new(w, h, d, t),
        }
    }
}

impl State for BoidsState {
    fn update(&mut self) {
        self.field1.update();
    }
}
