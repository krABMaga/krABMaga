use std::fmt;

pub trait Location2D<T: fmt::Display + Eq + PartialEq + Copy> {
    fn get_location(self) -> T;
    fn set_location(&mut self, loc: T);
}

#[derive(Clone, Default, Debug, Copy)]
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

#[derive(Clone, Hash, Copy)]
pub struct Int2D {
    pub x: i64,
    pub y: i64,
}

impl fmt::Display for Int2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Eq for Int2D {}

impl PartialEq for Int2D {
    fn eq(&self, other: &Int2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}
