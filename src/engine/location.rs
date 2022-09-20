use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

/// A structure describing a two-dimensional, f32 location, for use in continuous fields.
#[derive(Clone, Default, Copy, Debug)]
pub struct Real2D {
    pub x: f32,
    pub y: f32,
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

/// A structure describing a two-dimensional, i32 location, for use in discrete fields such as a grid.
#[derive(Clone, Copy)]
pub struct Int2D {
    pub x: i32,
    pub y: i32,
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

impl Hash for Int2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

/// A structure describing a two-dimensional, f32 location, for use in continuous fields.
#[derive(Clone, Default, Copy, Debug)]
pub struct Real3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Real3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Eq for Real3D {}

impl PartialEq for Real3D {
    fn eq(&self, other: &Real3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

/// A structure describing a two-dimensional, i32 location, for use in discrete fields such as a grid.
#[derive(Clone, Copy)]
pub struct Int3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl fmt::Display for Int3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Eq for Int3D {}

impl PartialEq for Int3D {
    fn eq(&self, other: &Int3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Hash for Int3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}
