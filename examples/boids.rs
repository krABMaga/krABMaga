extern crate abm;
extern crate priority_queue;

use std::fmt;
use abm::agent::Agent;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::simulstate::SimState;
use std::time::{Instant};
use abm::field2D::Field2D;
use abm::location::Real2D;
use abm::location::Location2D;

static mut COUNT: u128 = 0;
static STEP: u128 = 10;
static NUM_AGENT: u128 = 100;


fn main() {
    let data = MyData::new();
    let mut simstate: SimState<Bird> = SimState::new();
    let mut schedule: Schedule<Bird, MyData<Location2D>>= Schedule::new();
    assert!(schedule.events.is_empty());

    for bird_id in 1..NUM_AGENT{

        let bird = Bird::new(bird_id, Real2D{x: 1.0, y: 1.0});
        let bird_clone = bird.clone();
        simstate.field.set_object_location(bird);
        let pa = AgentImpl::new(bird_clone);
        schedule.schedule_repeating(pa, 5.0, 100);
    }

    let start = Instant::now();
    for _step in 1..STEP{
        //println!("step {}", step);
        schedule.step(&simstate);
    }
    let duration = start.elapsed();
    println!("Time elapsed in testing schedule is: {:?}", duration);
    println!("Step for seconds: {:?}", duration.as_millis()/STEP)

}

#[derive(Clone)]
pub struct MyData<P: Location2D> {
    field: Field2D<P>,
}

impl <P: Location2D> MyData<P> {
    pub fn new() -> MyData<P> {
        MyData {
            field: Field2D::new(),
        }
    }
}
#[derive(Clone)]
pub struct Bird {
    x: u128,
    pos: Real2D,
}

impl Bird {
    pub fn new(x: u128, pos: Real2D) -> Self {
        Bird {
            x,
            pos
        }
    }
}

impl Agent for Bird {
    fn step<P: Location2D>(self, simstate: &SimState<P>) {
        //let vec = simstate.field.get_neighbors_within_distance(self);
        unsafe {
            COUNT += self.x;
        }
    }
}

impl Location2D for Bird {
    fn get_location(self) -> Real2D {
        self.pos
    }

    fn set_location(&mut self, loc: Real2D) {
        self.pos = loc;
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}
