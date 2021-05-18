use std::fmt;

/// A trait specifying the position of a struct.
///
/// # Safety
///
/// The generic type T bounds are lax, they can support types different than Real2D and Int2D,
/// but it has been tested properly only with those two.
pub trait Location2D<T: fmt::Display + Eq + PartialEq + Copy> {
    fn get_location(self) -> T;
    fn set_location(&mut self, loc: T);
}

/// A structure describing a two-dimensional, f64 position, for use in continuous fields.
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

/// A structure describing a two-dimensional, i64 position, for use in discrete fields such as a grid.
#[derive(Clone, Hash, Copy, Debug)]
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
