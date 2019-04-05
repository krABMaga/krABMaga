extern crate abm;

extern crate priority_queue;


use std::fmt;
use abm::agent::Agent;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::simulstate::SimState;
use std::default::Default;
use std::time::{Instant};

static mut COUNT: u128 = 0;
static STEP: u128 = 1000;
static NUM_AGENT: u128 = 1000000;

fn main() {

    let mut schedule: Schedule<Bird> = Default::default();
    assert!(schedule.events.is_empty());

    for bird_id in 1..NUM_AGENT{
        let bird = Bird::new(bird_id);
        let pa = AgentImpl::new(bird);
        schedule.schedule_repeating(pa, 5.0, 100);
    }

    let simstate = SimState {
        //schedule: schedule.clone(),
    };
    let start = Instant::now();
    for _step in 1..STEP{
        //println!("step {}", step);
        schedule.step(&simstate);
    }
    let duration = start.elapsed();
    println!("Time elapsed in testing schedule is: {:?}", duration);
    println!("Step for seconds: {:?}", duration.as_millis()/STEP)

}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bird {
    x: u128,
}

impl Bird {
    pub fn new(x: u128) -> Self {
        Bird {
            x
        }
    }
}

impl Agent for Bird {
    fn step(self, _simstate: &SimState) {
        //println!("{:?} ha fatto lo step", self.x);
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
