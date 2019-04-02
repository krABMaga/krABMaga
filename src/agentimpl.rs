use crate::agent::Agent;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::fmt;
use crate::simstate::SimState;
use std::clone::Clone;
#[derive(Clone)]

pub struct AgentImpl<A: Agent + Clone>{
    pub id: String,
    pub agent: A,
    pub repeating: bool,
}

impl<A: Agent + Clone> AgentImpl<A> {
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

    pub fn id(self) -> String {
        self.id
    }
}

impl<A: Agent + Clone> Eq for AgentImpl<A>{}

impl<A: Agent + Clone> PartialEq for AgentImpl<A> {
    fn eq(&self, other: &AgentImpl<A>) -> bool {
        self.id == other.id
    }
}

impl<A: Agent + Clone> Hash for AgentImpl<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<A: Agent + Clone> fmt::Display for AgentImpl<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

// impl<A: Agent> Clone for AgentImpl<A> {
//     fn clone(&self) -> Self {
//         AgentImpl {
//             id: self.id,
//             agent: self.agent,
//             repeating: self.repeating,
//         }
//     }
// }
