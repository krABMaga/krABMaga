extern crate abm;
extern crate priority_queue;

use rand::Rng;
use std::hash::Hasher;
use std::hash::Hash;
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
static WIDTH: f64 = 150.0;
static HEIGTH: f64 = 150.0;
static DISCRETIZATION: f64 = 10.0/1.5;
static TOROIDAL: bool = true;

fn main() {
    let mut rng = rand::thread_rng();


    let mut data = State::new(WIDTH, HEIGTH, DISCRETIZATION, TOROIDAL);
    let mut schedule: Schedule<Bird> = Schedule::new();
    assert!(schedule.events.is_empty());

    unsafe {
        for bird_id in 0..NUM_AGENT{
            let data_ref = &data as *const State;
            let r1: f64 = rng.gen();
            let r2: f64 = rng.gen();
            let bird = Bird::new(bird_id, Real2D{x: WIDTH*r1, y: HEIGTH*r2}, &*data_ref);
            //let bird_clone = bird.clone();
            data.field1.set_object_location(bird.clone(), bird.pos.clone());
            //let pa = AgentImpl::new(bird_clone);
            schedule.schedule_repeating(bird, 5.0, 100);
        }
    }
    // let data_ref = &data as *const State;
    // unsafe {
    //     let bird1 = Bird::new(1, Real2D{x: 1.0, y: 1.0}, &*data_ref);
    //     let bird2 = Bird::new(2, Real2D{x: 1.0, y: 2.0}, &*data_ref);
    //     let bird3 = Bird::new(3, Real2D{x: 4.0, y: 4.0}, &*data_ref);
    //
    //     data.field1.set_object_location(bird1.clone(), bird1.pos.clone());
    //     data.field1.set_object_location(bird2.clone(), bird2.pos.clone());
    //     data.field1.set_object_location(bird3.clone(), bird3.pos.clone());
    //
    //
    //
    //   schedule.schedule_repeating(bird1, 5.0, 100);
    //    schedule.schedule_repeating(bird2, 5.0, 100);
    //     schedule.schedule_repeating(bird3, 5.0, 100);
    //
    // }
    // assert!(!schedule.events.is_empty());

    let start = Instant::now();
    for _ in 1..STEP{
        //println!("step {}", step);
        schedule.step();
    }
    let duration = start.elapsed();

    println!("Time elapsed in testing schedule is: {:?}", duration);
    println!("Step for seconds: {:?}", STEP as u64/duration.as_secs());

}

pub struct State<'a>{
    pub field1: Field2D<Bird<'a>>,
}

impl<'a> State<'a>{
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> State<'a> {
        State {
            field1: Field2D::new(w, h, d, t),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Bird<'a> {
    pub id: u128,
    pub pos: Real2D,
    pub state: &'a State<'a>,
}

impl<'a> Hash for Bird<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u128(self.id);
        state.finish();
    }
}

impl<'a > Bird<'a> {
    pub fn new(id: u128, pos: Real2D, state: &'a State) -> Self {
        Bird {
            id,
            pos,
            state,
        }
    }
}

impl<'a> Eq for Bird<'a> {}

impl<'a> PartialEq for Bird<'a> {
    fn eq(&self, other: &Bird) -> bool {
        self.id == other.id
    }
}

impl<'a> Agent for Bird<'a> {
    fn step(&self) {

        let vec = self.state.field1.get_neighbors_within_distance(self.pos.clone(), 10.0);
        for _elem in vec {
            //println!("{}", elem.id);
        }
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
        write!(f, "{}", self.id)
    }
}
