//use crate::field2D::Field2D;

pub trait Agent {
    fn step<B: S>(&self, state: B);
}

pub trait S {
    type I;
    fn get_state(self) -> Self::I;
}
