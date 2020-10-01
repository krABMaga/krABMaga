use std::{fmt, hash::Hash, hash::Hasher};

use amethyst::ecs::{Component, DenseVecStorage};
use abm::location::{Location2D, Real2D};


/*
	Adapter that represents an entity as an agent for RustAB calculations.
	Used, in particular, to represent the agent in the Field2D instance, to be able to fetch neighbors.
*/

#[derive(Clone, Copy)]
pub struct AgentAdapter {
    // An unique id, so that we do not run neighbor calculations with ourself.
    pub id: u128,
    // The position of the agent in a 2D field.
    pub pos: Real2D,
    // The last known position, used to calculate momentum.
    pub last_d: Real2D
}

impl AgentAdapter {
	pub fn new(id: u128, pos: Real2D, last_d: Real2D) -> AgentAdapter {
		AgentAdapter {
            id,
            pos,
            last_d
		}
	}
}

// Implements Component so that we can attach it to entities and fetch it in systems.
impl Component for AgentAdapter {
	type Storage = DenseVecStorage<Self>;
}

impl Hash for AgentAdapter {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u128(self.id);
        state.finish();
    }
}

impl Eq for AgentAdapter {}

impl PartialEq for AgentAdapter {
    fn eq(&self, other: &AgentAdapter) -> bool {
        self.id == other.id
    }
}

impl Location2D for AgentAdapter {
    fn get_location(self) -> Real2D {
        self.pos
    }

    fn set_location(&mut self, loc: Real2D) {
        self.pos = loc;
    }
}

impl fmt::Display for AgentAdapter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}