extern crate priority_queue;
#[macro_use]
extern crate criterion;
use abm::location::Real2D;
use abm::location::Location2D;
use criterion::Criterion;

use std::fmt;
use abm::agent::Agent;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::simulstate::SimState;
use std::default::Default;


static mut COUNT: u32 = 0;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Scheduling", |b| b.iter(|| schedule_test()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


fn schedule_test() {

    let mut schedule: Schedule<Bird> = Schedule::new();
    assert!(schedule.events.is_empty());

    for bird_id in 1..1000{
        let bird = Bird::new(bird_id, Real2D{x: 1.0, y: 1.0});
        let pa = AgentImpl::new(bird);
        schedule.schedule_repeating(pa, 5.0, 100);
    }

    let simstate: SimState<Bird> = SimState::new();

    for _step in 1..100{
        //println!("step {}", step);
        schedule.step(&simstate);
    }


}

#[derive(Clone)]
pub struct Bird {
    x: u32,
    pos: Real2D,
}

impl Bird {
    pub fn new(x: u32, pos: Real2D) -> Self {
        Bird {
            x,
            pos
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

impl Agent for Bird {
    fn step<P: Location2D>(self, _simstate: &SimState<P>) {
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
