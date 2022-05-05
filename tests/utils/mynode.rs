use std::fmt::Display;
use std::hash::{Hash, Hasher};

use krABMaga::engine::agent::Agent;

#[derive(Clone, Debug, Copy)]
pub struct MyNode {
    pub id: u32,
    pub flag: bool,
}

impl Agent for MyNode{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn step(&mut self, state: &mut dyn krABMaga::engine::state::State) {
        
    }
}

impl Eq for MyNode {}

impl PartialEq for MyNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for MyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} flag {}", self.id, self.flag)
    }
}

impl Hash for MyNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}
