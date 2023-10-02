use crate::engine::agent::Agent;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone)]
/// Concrete type for the Agent
///
/// id: id of the AgentImpl
///
/// agent: wrap inside a Box a dyn Agent trait to allow the use of a custom agent type
///
/// repeating: boolean used for the scheduling option
///
pub struct AgentImpl {
    pub id: u32,
    pub agent: Box<dyn Agent>,
    pub repeating: bool,
}

impl AgentImpl {
    /// create a new instance of AgentImpl
    pub fn new(agent: Box<dyn Agent>, id: u32) -> AgentImpl {
        AgentImpl {
            id,
            agent,
            repeating: false,
        }
    }
}

impl fmt::Display for AgentImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.repeating)
    }
}

impl PartialEq for AgentImpl {
    fn eq(&self, other: &AgentImpl) -> bool {
        self.id == other.id
    }
}

impl Hash for AgentImpl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for AgentImpl {}
