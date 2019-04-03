extern crate priority_queue;

use std::fmt;
use abm::agent::Agent;
use abm::simstate::SimState;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;

fn main() {

    let mut schedule: Schedule<Bird> = Schedule::new();
    assert!(schedule.events.is_empty());

    for bird_id in 1..10000 {
        let bird = Bird::new(bird_id);
        let pa = AgentImpl::new(bird);
        schedule.schedule_repeating(pa, 5.0, 100);
    }

    let simstate = SimState {
        schedule: schedule.clone(),
    };

    for step in 1..100{
        println!("step {}", step);
        schedule.step(&simstate);
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Bird {
    x: u32,
}

impl Bird {
    pub fn new(x: u32) -> Self {
        Bird {
            x
        }
    }
}

impl Agent for Bird {
    fn step(self, simstate: &SimState<A>) {
        println!("{:?} ha fatto lo step", self.x);
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}
