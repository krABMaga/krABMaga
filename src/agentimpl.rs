use crate::agent::Agent;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::fmt;
use crate::simstate::SimState;

pub struct AgentImpl<A: Agent>{
    pub id: String,
    pub agent: A,
    pub repeating: bool,
}

impl<A: Agent> AgentImpl<A> {
    pub fn new(agent: A) -> AgentImpl<A>
        where A: Agent
    {
        AgentImpl {
            id: String::new(),
            agent: agent,
            repeating: false,
        }
    }

    pub fn step(self, simstate: &SimState<A>) {
        self.agent.step(simstate);
    }
}

impl<A: Agent> Eq for AgentImpl<A>{}

impl<A: Agent> PartialEq for AgentImpl<A> {
    fn eq(&self, other: &AgentImpl<A>) -> bool {
        self.id == other.id
    }
}

impl<A: Agent> Hash for AgentImpl<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<A: Agent> fmt::Display for AgentImpl<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
