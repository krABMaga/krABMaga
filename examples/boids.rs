extern crate abm;
extern crate priority_queue;


use std::fmt;
use abm::agent::Agent;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::simulstate::SimState;
use std::time::{Instant};
use abm::location::Real2D;
use abm::location::Location2D;
use abm::field2D::Field2D;

static mut _COUNT: u128 = 0;
static STEP: u128 = 10;
static NUM_AGENT: u128 = 10;


fn main() {
    let field = Field2D::new();
    // let field2 = Field2D::new();
    // let field3 = Field2D::new();
    //let data = MyData::new(field,field2,field3);
    let data = State::new();
    let mut simstate: SimState = SimState::new();
    let mut schedule: Schedule<Bird> = Schedule::new();//data
    assert!(schedule.events.is_empty());

    for bird_id in 1..NUM_AGENT{

        let bird = Bird::new(bird_id, Real2D{x: 1.0, y: 1.0}, &data);
        let bird_clone = bird.clone();
        simstate.field.set_object_location(bird);
        let pa = AgentImpl::new(bird_clone);
        schedule.schedule_repeating(pa, 5.0, 100);
    }
    assert!(!schedule.events.is_empty());

    let start = Instant::now();
    for _step in 1..STEP{
        //println!("step {}", step);
        schedule.step();
    }
    let duration = start.elapsed();
    println!("Time elapsed in testing schedule is: {:?}", duration);
    println!("Step for seconds: {:?}", duration.as_millis()/STEP)

}



pub struct State{
    pub field1: Field2D,
    pub field2: Field2D,
    pub field3: Field2D,
}

impl State{
    pub fn new() -> State {
        State {
            field1: Field2D::new(),
            field2: Field2D::new(),
            field3: Field2D::new(),
        }
    }
}


#[derive(Clone)]
pub struct Bird<'a> {
    pub x: u128,
    pub pos: Real2D,
    pub state: &'a State,
}

impl<'a> Bird<'a> {
    pub fn new(x: u128, pos: Real2D, state: &'a State) -> Self {
        Bird {
            x,
            pos,
            state,
        }
    }
}

impl<'a> Agent for Bird<'a> {
    fn step(self) {
        //fn step(self, data: &MyData) {
        let _vec = self.state.field1.get_neighbors_within_distance(self);
        // unsafe {
        //     COUNT += self.x;
        // }
    }
}

impl<'a> Location2D for Bird<'a> {
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
