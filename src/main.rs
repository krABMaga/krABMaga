extern crate priority_queue;
//use priority_queue::PriorityQueue;
//use abm::agent::Agent;
//use abm::priority::Priority;
use std::fmt;
use abm::agent::Agent;
use abm::simstate::SimState;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
//use priority::Priority;
//use agent::Agent;

fn main() {

    let mut schedule: Schedule<Bird> = Schedule::new();

    for bird_id in 1..10 {
        let bird = Bird::new(String::from(bird_id.to_string()));
        println!("{}", bird_id);
        let pa = AgentImpl::new(bird);
        schedule.schedule_repeating(pa, 1.0, 10);
        println!("{}", schedule.events.len());
    }


    let simstate = SimState {
        schedule: schedule.clone(),
    };


    for step in 1..3 {
        println!("step {}", step);
        schedule.step(&simstate);
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Bird {
    id: String,
}

impl Bird {
    pub fn new(id: String) -> Bird {
        Bird {
            id,
        }
    }
}

impl Agent for Bird {
    fn step<A: Agent + Clone>(self, _simstate: &SimState<A>) {
        println!("{:?} ha fatto lo step", self.id);
    }

    fn id<A: Agent + Clone>(self) -> String {
        self.id
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
