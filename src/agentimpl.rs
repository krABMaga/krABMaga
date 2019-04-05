//use uid::Id as IdT;
use crate::agent::Agent;
// use std::cmp::Eq;
//use std::hash::{Hash};
use std::fmt;
use crate::simulstate::SimState;
use std::clone::Clone;

static mut COUNTER: u32 = 0;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct AgentImpl<A: Agent + Clone>{
    pub id: u32,
    pub agent: A,
    pub repeating: bool,
}

impl<A: Agent + Clone> AgentImpl<A> {
    pub fn new(the_agent: A) -> AgentImpl<A>
        where A: Agent
    {
        unsafe {
            COUNTER += 1;

            AgentImpl {
                    id: COUNTER,
                    agent: the_agent,
                    repeating: false,
                }
            }
    }

    pub fn step(self, simstate: &SimState) {
        self.agent.step(simstate);
    }

    pub fn id(self) -> u32 {
        self.id
    }
}

impl<A: Agent + Clone> fmt::Display for AgentImpl<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.repeating)
    }
}
