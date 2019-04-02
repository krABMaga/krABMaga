extern crate priority_queue;
//use priority_queue::PriorityQueue;
//use abm::agent::Agent;
//use abm::priority::Priority;
use std::fmt;
use abm::agent::Agent;
use abm::simstate::SimState;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::priority::Priority;
//use priority::Priority;
//use agent::Agent;

fn main() {
    let piccione = Bird {
        id: String::from("piccione"),
    };
    let quaglia = Bird {
        id: String::from("quaglia"),
    };
    assert_eq!{piccione, piccione.clone()};


    let pa = AgentImpl::new(piccione);
    let pp = Priority {
        time: 10.0,
        ordering: 100
    };
    let qa = AgentImpl::new(quaglia);
    let qp = Priority {
        time: 5.0,
        ordering: 200
    };

    let mut schedule: Schedule<Bird> = Schedule::new();

    schedule.events.push(pa, pp);
    schedule.events.push(qa, qp);

    // for (item, _) in schedule.events.into_sorted_iter() {
    //         println!("1 {}", item);
    //     }

    let simstate = SimState {
        schedule: schedule.clone(),
    };

    schedule.step(&simstate);
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
}

impl Eq for Bird{}

impl PartialEq for Bird {
    fn eq(&self, other: &Bird) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
