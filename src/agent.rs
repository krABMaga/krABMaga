use std::cmp::Eq;
use std::hash::{Hash, Hasher};

pub struct Agent{
    pub id: String
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            id: String::new(),
        }
    }
}

impl Eq for Agent {}

impl PartialEq for Agent {
    fn eq(&self, other: &Agent) -> bool {
        self.id == other.id
    }
}
impl Hash for Agent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
