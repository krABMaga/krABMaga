extern crate priority_queue;


//use abm::location::Location2D;
use std::fmt;
use abm::agent::Agent;
use abm::priority::Priority;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::simulstate::SimState;
//use abm::field::Field;
//use std::default::Default;
//use priority_queue::PriorityQueue;
use abm::field2D::Field2D;

static mut COUNT: u32 = 0;

// #[test]
// fn schedule_test_1() {
//
//     let field = Field2D::new();
//     let mut simstate: SimState = SimState::new();
//     let mut schedule: Schedule<Bird, Field2D> = Schedule::new(field);
//     assert!(schedule.events.is_empty());
//
//     let mut priority_queue = PriorityQueue::new();
//     let mut counter = 0;
//
//     for bird_id in 1..4{
//         counter += bird_id;
//         let bird = Bird::new(bird_id);
//         let pa = AgentImpl::new(bird);
//         let mut pa_clone = pa.clone();
//         pa_clone.repeating = true;
//         priority_queue.push(pa_clone, Priority{time: 5.0, ordering: 100});
//         schedule.schedule_repeating(pa, 5.0, 100);
//     }
//
//     assert!(!schedule.events.is_empty());
//     //assert_eq!(schedule.events, priority_queue);
//
//     // let simstate = SimState {
//     // };
//     //
//     // schedule.step(&simstate);
//     // unsafe {
//     //     assert_eq!(counter, COUNT);
//     // }
// }

#[test]
fn schedule_test_2() {

    let field = Field2D::new();
    let mut _simstate: SimState = SimState::new();
    let mut schedule: Schedule<Bird, Field2D> = Schedule::new(field);

    let bird1 = Bird {x: 1};
    let bird2 = Bird {x: 2};
    let bird3 = Bird {x: 3};
    let pa1 = AgentImpl::new(bird1);
    let pa2 = AgentImpl::new(bird2);
    let pa3 = AgentImpl::new(bird3);

    let mut pa1_clone = pa1.clone();
    pa1_clone.repeating = true;
    let mut pa2_clone = pa2.clone();
    pa2_clone.repeating = true;
    let mut pa3_clone = pa3.clone();
    pa3_clone.repeating = true;

    schedule.schedule_repeating(pa2, 8.0, 100);
    schedule.schedule_repeating(pa1, 5.0, 100);
    schedule.schedule_repeating(pa3, 10.0, 100);

    let pr1 = Priority {time: 5.0, ordering: 100};
    let x1 = (pa1_clone, pr1);
    assert_eq!(Some(x1), schedule.events.pop());
    let pr2 = Priority {time: 8.0, ordering: 100};
    let x2 = (pa2_clone, pr2);
    assert_eq!(Some(x2), schedule.events.pop());
    let pr3 = Priority {time: 10.0, ordering: 100};
    let x3 = (pa3_clone, pr3);
    assert_eq!(Some(x3), schedule.events.pop());

    // let simstate = SimState {};
    //
    // // schedule.step(&simstate);
    //
    // schedule.step(&simstate);
}
//
// #[test]
// fn field_test_1() {
//     let bird = Bird{x: 1};
//     let pa = AgentImpl::new(bird);
//     let pa_clone = pa.clone();
//     let mut field : Field<Bird> = Default::default();
//     field.hash_map.insert(1, pa);
//     assert_eq!(Some(&pa_clone), field.hash_map.get(&1));
// }

#[derive(Clone, Default, Debug)]
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
