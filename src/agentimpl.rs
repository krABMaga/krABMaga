use crate::agent::Agent;
use std::clone::Clone;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

static mut COUNTER: u32 = 0;

#[derive(Clone, Debug)]
pub struct AgentImpl<A: Agent + Clone> {
    pub id: u32,
    pub agent: A,
    pub repeating: bool,
}

impl<A: Agent + Clone> AgentImpl<A> {
    pub fn new(the_agent: A) -> AgentImpl<A> {
        unsafe {
            COUNTER += 1;

            AgentImpl {
                id: COUNTER,
                agent: the_agent,
                repeating: false,
            }
        }
    }

    // pub fn step(&mut self) {
    //     self.agent.step();
    // }

    pub fn id(self) -> u32 {
        self.id
    }
}

impl<A: Agent + Clone> fmt::Display for AgentImpl<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.repeating)
    }
}

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

impl<A: Agent + Clone> Eq for AgentImpl<A> {}
