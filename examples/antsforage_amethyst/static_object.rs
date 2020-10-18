use std::{fmt, hash::Hash, hash::Hasher};

use amethyst::ecs::{Component, DenseVecStorage};
use abm::{location::{DiscreteLocation2D, Int2D}};


/*
    A static object, like a wall, an obstacle or anything that doesn't move.
*/

#[derive(Clone, Copy)]
pub enum StaticObjectType {
    HOME,
    FOOD,
    OBSTACLE,
}

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

impl DiscreteLocation2D for StaticObject {
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