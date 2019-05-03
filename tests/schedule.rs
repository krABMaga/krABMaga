extern crate priority_queue;

//use abm::location::Location2D;
use abm::location::Real2D;
use abm::field2D::Field2D;
use std::fmt;
use abm::agent::Agent;
use abm::priority::Priority;
use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::location::Location2D;
//use abm::field::Field;
//use std::default::Default;
//use priority_queue::PriorityQueue;
//use abm::field2D::Field2D;

//static mut COUNT: u32 = 0;

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

    let data = State::new();
    let mut schedule: Schedule<Bird> = Schedule::new();
    assert!(schedule.events.is_empty());

    let bird1 = Bird {x: 1, pos: Real2D{x: 1.0, y: 1.0}, state: &data};
    let bird2 = Bird {x: 2, pos: Real2D{x: 2.0, y: 2.0}, state: &data};
    let bird3 = Bird {x: 3, pos: Real2D{x: 3.0, y: 3.0}, state: &data};

    schedule.schedule_repeating(bird2.clone(), 8.0, 100);
    schedule.schedule_repeating(bird1.clone(), 5.0, 100);
    schedule.schedule_repeating(bird3.clone(), 10.0, 100);

    let mut ag1 = AgentImpl::new(bird1.clone());
    //se no istanzia un agente impl diverso
    ag1.id = 2;
    ag1.repeating = true;
    let pr1 = Priority {time: 5.0, ordering: 100};
    let x1 = (ag1, pr1);

    assert_eq!(Some(x1), schedule.events.pop());

    let mut ag2 = AgentImpl::new(bird2.clone());
    //se no istanzia un agente impl diverso
    ag2.id = 1;
    ag2.repeating = true;
    let pr2 = Priority {time: 8.0, ordering: 100};
    let x2 = (ag2, pr2);

    assert_eq!(Some(x2), schedule.events.pop());

    let mut ag3 = AgentImpl::new(bird1.clone());
    //se no istanzia un agente impl diverso
    ag3.id = 3;
    ag3.repeating = true;
    let pr3 = Priority {time: 10.0, ordering: 100};
    let x3 = (ag3, pr3);

    assert_eq!(Some(x3), schedule.events.pop());
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
