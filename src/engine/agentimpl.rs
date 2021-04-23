use crate::engine::agent::Agent;
use std::clone::Clone;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;



///Wrapper for the Agent struct, providing an integer id and a boolean field for repeated scheduling.
#[derive(Clone, Debug)]
pub struct AgentImpl<A: Agent + Clone + Hash + Eq> {
    pub agent: A,
    pub repeating: bool,
}

impl<A: Agent + Clone + Hash + Eq> AgentImpl<A> {
    ///Instantiates a new AgentImpl object, wrapping the_agent.
    pub fn new(the_agent: A) -> AgentImpl<A> {
    
            AgentImpl {
                agent: the_agent,
                repeating: false,
            }
       
    }

    // pub fn step(&mut self) {
    //     self.agent.step();
    // }
    
 
}

impl<A: Agent + Clone + Hash + Eq + fmt::Display> fmt::Display for AgentImpl<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.agent, self.repeating)
    }
}

impl<A: Agent + Clone + Hash + Eq> PartialEq for AgentImpl<A> {
    fn eq(&self, other: &AgentImpl<A>) -> bool {
        self.agent == other.agent
    }
}

impl<A: Agent + Clone + Hash + Eq> Hash for AgentImpl<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.agent.hash(state);
    }
}

impl<A: Agent + Clone + Hash + Eq> Eq for AgentImpl<A> {}
