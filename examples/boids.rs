extern crate abm;
extern crate priority_queue;

use std::fmt;
use abm::agent::Agent;
//use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
//use abm::simulstate::SimState;
use std::time::{Instant};
use abm::location::Real2D;
use abm::location::Location2D;
use abm::field2D::Field2D;

static mut _COUNT: u128 = 0;
static STEP: u128 = 10;
static NUM_AGENT: u128 = 10000;


fn main() {

    let mut data = State::new();
    //let mut simstate: SimState = SimState::new();
    let mut schedule = Schedule::new();//data
    //let mut schedule: Schedule<Bird> = Schedule::new();
    assert!(schedule.events.is_empty());

    unsafe {
        for bird_id in 1..NUM_AGENT{
            let data_ref = &data as *const State;
            let bird = Bird::new(bird_id, Real2D{x: 1.0, y: 1.0}, &*data_ref);
            //let bird_clone = bird.clone();
            data.field1.set_object_location(bird.clone());
            //let pa = AgentImpl::new(bird_clone);
            schedule.schedule_repeating(bird, 5.0, 100);
        }
    }

    assert!(!schedule.events.is_empty());

    let start = Instant::now();
    for _ in 1..STEP{
        //println!("step {}", step);
        schedule.step();
    }
    let duration = start.elapsed();

    println!("Time elapsed in testing schedule is: {:?}", duration);
    println!("Step for seconds: {:?}", duration.as_millis()/STEP)

}

#[derive(Debug)]
pub struct State<'a>{
    pub field1: Field2D<Bird<'a>>,
}

impl<'a> State<'a>{
    pub fn new() -> State<'a> {
        State {
            field1: Field2D::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bird<'a> {
    pub x: u128,
    pub pos: Real2D,
    pub state: &'a State<'a>,
}

impl<'a > Bird<'a> {
    pub fn new(x: u128, pos: Real2D, state: &'a State) -> Self {
        Bird {
            x,
            pos,
            state,
        }
    }
}

impl<'a> Eq for Bird<'a> {}

impl<'a> PartialEq for Bird<'a> {
    fn eq(&self, other: &Bird) -> bool {
        self.x == other.x && self.pos == other.pos
    }
}

impl<'a> Agent for Bird<'a> {
    fn step(&self) {

        let _vec = self.state.field1.get_neighbors_within_distance(self);

    }

}

impl<'a > Location2D for Bird<'a> {
    fn get_location(self) -> Real2D {
        self.pos
    }

    fn set_location(&mut self, loc: Real2D) {
        self.pos = loc;
    }
}

impl<'a> fmt::Display for Bird<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}
