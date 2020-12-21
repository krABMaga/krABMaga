use std::{fmt, hash::Hash, hash::Hasher};

use abm::location::{Int2D, Location2D};
use amethyst::ecs::{Component, DenseVecStorage};

/// The type of the static object
#[derive(Clone, Copy)]
pub enum StaticObjectType {
    HOME,
    FOOD,
    OBSTACLE,
}

/// A simple wrapper over StaticObjectType to implement Component on it. Used for objects that are not
/// supposed to move during the simulation's run, like obstacles, nests and food sources.
#[derive(Clone, Copy)]
pub struct StaticObject {
    // An unique id.
    pub id: u128,
    // The position of the object.
    pub loc: Int2D,
    // The type of the object
    pub object_type: StaticObjectType,
}

impl StaticObject {
    pub fn new(id: u128, loc: Int2D, object_type: StaticObjectType) -> StaticObject {
        StaticObject {
            id,
            loc,
            object_type,
        }
    }
}

// Implements Component so that we can attach it to entities and fetch it in systems.
impl Component for StaticObject {
    type Storage = DenseVecStorage<Self>;
}

impl Hash for StaticObject {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u128(self.id);
        state.finish();
    }
}

impl Eq for StaticObject {}

impl PartialEq for StaticObject {
    fn eq(&self, other: &StaticObject) -> bool {
        self.id == other.id
    }
}

impl Location2D<Int2D> for StaticObject {
    fn get_location(self) -> Int2D {
        self.loc
    }

    fn set_location(&mut self, loc: Int2D) {
        self.loc = loc;
    }
}

impl fmt::Display for StaticObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
