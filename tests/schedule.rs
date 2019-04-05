extern crate priority_queue;

use std::fmt;
use abm::agent::Agent;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::simulstate::SimState;
use std::default::Default;

static mut COUNT: u32 = 0;

#[test]
fn schedule_test() {

    let mut schedule: Schedule<Bird> = Default::default();
    assert!(schedule.events.is_empty());

    for bird_id in 1..3{
        let bird = Bird::new(bird_id);
        let pa = AgentImpl::new(bird);
        schedule.schedule_repeating(pa, 5.0, 100);
    }

    let simstate = SimState {
        //schedule: schedule.clone(),
    };

    for step in 1..10{
        println!("step {}", step);
        schedule.step(&simstate);
    }

    
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    fn step(self, _simstate: &SimState) {
        println!("{:?} ha fatto lo step", self.x);
        unsafe {
            COUNT += self.x;
        }
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}
