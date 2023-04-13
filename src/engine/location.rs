#![allow(warnings)]
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use crate::mpi::topology::Communicator;
use crate::mpi::point_to_point::Destination;
use crate::mpi::datatype::UncommittedDatatypeRef;
use crate::mpi::datatype::UserDatatype;
use core::mem::size_of;
use crate::lazy_static;
use crate::mpi::traits::*;
use crate::mpi::Address;
use crate::mpi::environment::Universe;
use crate::mpi::topology::SystemCommunicator;
use crate::mpi::point_to_point::Source;
use crate::mpi::Threading;
use crate::mpi::ffi::MPI_Finalize;

/// A structure describing a two-dimensional, f32 location, for use in continuous fields.
#[derive(Clone, Default, Copy, Debug, Equivalence)]
pub struct Real2D {
    pub x: f32,
    pub y: f32,
}

/* unsafe impl Equivalence for Real2D{
    type Out = UserDatatype;
    fn equivalent_datatype() -> Self::Out {
        UserDatatype::structured(
            &[1, 1],
            &[
                (size_of::<f32>() * 2) as mpi::Address,
                size_of::<f32>() as mpi::Address,
            ],
            &[f32::equivalent_datatype(); 2],
        )
    }
} */

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
