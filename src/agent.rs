//use crate::field2D::Field2D;

pub trait Agent<'a> {
    fn step(&'a self);
}
