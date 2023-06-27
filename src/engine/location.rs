#[cfg(feature = "distributed_mpi")]
use crate::mpi::traits::*;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

#[cfg(feature = "distributed_mpi")]
#[derive(Clone, Default, Copy, Debug, Equivalence)]
pub struct Real2D {
    pub x: f32,
    pub y: f32,
}

#[cfg(not(feature = "distributed_mpi"))]
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
