use std::fmt;

pub trait Location2D {
    fn get_location(self) -> Real2D;
    fn set_location(&mut self, loc: Real2D);
}

#[derive(Clone, Default, Debug)]
pub struct Real2D {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Real2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Eq for Real2D {}

impl PartialEq for Real2D {
    fn eq(&self, other: &Real2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Hash)]
pub struct Int2D {
    pub x: i64,
    pub y: i64,
}

impl Eq for Int2D {}

impl PartialEq for Int2D {
    fn eq(&self, other: &Int2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}
