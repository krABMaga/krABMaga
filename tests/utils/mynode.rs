use std::fmt::Display;
use std::hash::{Hash, Hasher};

use krabmaga::engine::agent::Agent;
use krabmaga::engine::state::State;

#[derive(Clone, Debug, Copy)]
pub struct MyNode {
    pub id: u32,
    pub flag: bool,
}

impl Agent for MyNode {
    fn step(&mut self, _state: &mut dyn State) {}
    fn is_stopped(&self, _state: &mut dyn State) -> bool {
        false
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
