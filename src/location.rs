pub trait Location2D {
    fn get_location(self) -> Real2D;
    fn set_location(&mut self, loc: Real2D);
}

#[derive(Clone, Default)]
pub struct Real2D {
    pub x: f64,
    pub y: f64,
}
