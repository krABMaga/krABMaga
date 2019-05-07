extern crate priority_queue;

//use abm::location::Location2D;
use std::hash::Hasher;
use std::hash::Hash;
use abm::location::Real2D;
use abm::field2D::Field2D;
use std::fmt;
use abm::agent::Agent;
// use abm::priority::Priority;
// use abm::agentimpl::AgentImpl;
use abm::schedule::Schedule;
use abm::location::Location2D;

static STEP: u128 = 10;
// static NUM_AGENT: u128 = 2;
// static WIDTH: f64 = 10.0;
// static HEIGTH: f64 = 10.0;
// static DISCRETIZATION: f64 = 2.0;
// static TOROIDAL: bool = true;

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
fn schedule_test_1() {

    let width = 100.0;
    let heigth = 100.0;
    let discretization = 0.7;
    let toroidal = true;

    let mut data = State::new(width, heigth, discretization, toroidal);
    let mut schedule = Schedule::new();
    assert!(schedule.events.is_empty());

    let data_ref = &data as *const State;
    unsafe {
        let bird1 = Bird::new(1, Real2D{x: 10.5, y: 10.0}, &*data_ref);
        let bird2 = Bird::new(2, Real2D{x: 10.0, y: 10.0}, &*data_ref);

        data.field1.set_object_location(bird1.clone(), bird1.pos.clone());
        let pos = match data.field1.get_object_location(bird1.clone()) {
            Some(i) => i,
            None => panic!("no location"),
        };

        let real_pos = Real2D {x: 10.5, y: 10.0};
        assert_eq!(&real_pos, pos);

        data.field1.set_object_location(bird1.clone(), Real2D{x: 10.0, y: 10.0});
        let pos = match data.field1.get_object_location(bird1.clone()) {
            Some(i) => i,
            None => panic!("no location"),
        };

        let real_pos = Real2D {x: 10.0, y: 10.0};
        assert_eq!(&real_pos, pos);

        data.field1.set_object_location(bird2.clone(), bird2.pos.clone());

        let num = data.field1.num_objects_at_location(real_pos);
        assert_eq!(1, num);

        schedule.schedule_repeating(bird1, 5.0, 100);
        schedule.schedule_repeating(bird2, 5.0, 100);


    }

    assert!(!schedule.events.is_empty());

    for _ in 1..STEP{
        schedule.step();
    }

}

#[test]
fn schedule_test_2() {

    let width = 10.0;
    let heigth = 10.0;
    let discretization = 0.5;
    let toroidal = false;

    let mut data = State::new(width, heigth, discretization, toroidal);
    let mut schedule = Schedule::new();
    assert!(schedule.events.is_empty());

    let data_ref = &data as *const State;
    unsafe {
        let bird1 = Bird::new(1, Real2D{x: 5.5, y: 5.5}, &*data_ref);
        let bird2 = Bird::new(2, Real2D{x: 4.0, y: 4.0}, &*data_ref);
        let bird3 = Bird::new(3, Real2D{x: 5.2, y: 5.2}, &*data_ref);
        let bird4 = Bird::new(4, Real2D{x: 5.2, y: 2.2}, &*data_ref);
        let bird5 = Bird::new(5, Real2D{x: 5.2, y: 2.1}, &*data_ref);

        data.field1.set_object_location(bird1.clone(), bird1.pos.clone());
        data.field1.set_object_location(bird2.clone(), bird2.pos.clone());
        data.field1.set_object_location(bird3.clone(), bird3.pos.clone());
        data.field1.set_object_location(bird4.clone(), bird4.pos.clone());
        data.field1.set_object_location(bird5.clone(), bird5.pos.clone());

        //non funziona con &bird1
        let vec = data.field1.get_neighbors_within_distance(&bird5, Real2D{x: 5.2, y:5.2}, 3.0);
        assert_eq!(4, vec.len());

        let bird6 = Bird::new(6, Real2D{x: 0.1, y: 0.1}, &*data_ref);
        data.field1.set_object_location(bird6.clone(), bird6.pos.clone());

        let vec = data.field1.get_neighbors_within_distance(&bird6, Real2D{x: 5.2, y:5.2}, 5.0);
        assert_eq!(4, vec.len());

        schedule.schedule_repeating(bird1, 5.0, 100);
        schedule.schedule_repeating(bird2, 5.0, 100);


    }

    assert!(!schedule.events.is_empty());

    for _ in 1..STEP{
        schedule.step();
    }

}
//
// fn test_schedule_3() {
//     let mut schedule: Schedule<Bird> = Schedule::new();
//     assert!(schedule.events.is_empty());
//
//     let bird1 = Bird {x: 1, pos: Real2D{x: 1.0, y: 1.0}, state: &data};
//     let bird2 = Bird {x: 2, pos: Real2D{x: 2.0, y: 2.0}, state: &data};
//     let bird3 = Bird {x: 3, pos: Real2D{x: 3.0, y: 3.0}, state: &data};
//
//     schedule.schedule_repeating(bird2.clone(), 8.0, 100);
//     schedule.schedule_repeating(bird1.clone(), 5.0, 100);
//     schedule.schedule_repeating(bird3.clone(), 10.0, 100);
//
//     let mut ag1 = AgentImpl::new(bird1.clone());
//     //se no istanzia un agente impl diverso
//     ag1.id = 2;
//     ag1.repeating = true;
//     let pr1 = Priority {time: 5.0, ordering: 100};
//     let x1 = (ag1, pr1);
//
//     assert_eq!(Some(x1), schedule.events.pop());
//
//     let mut ag2 = AgentImpl::new(bird2.clone());
//     //se no istanzia un agente impl diverso
//     ag2.id = 1;
//     ag2.repeating = true;
//     let pr2 = Priority {time: 8.0, ordering: 100};
//     let x2 = (ag2, pr2);
//
//     assert_eq!(Some(x2), schedule.events.pop());
//
//     let mut ag3 = AgentImpl::new(bird1.clone());
//     //se no istanzia un agente impl diverso
//     ag3.id = 3;
//     ag3.repeating = true;
//     let pr3 = Priority {time: 10.0, ordering: 100};
//     let x3 = (ag3, pr3);
//
//     assert_eq!(Some(x3), schedule.events.pop());
// }

// #[test]
// fn field_test_1() {
//
//     let mut data = State::new();
//     let schedule: Schedule<Bird> = Schedule::new();
//     assert!(schedule.events.is_empty());
//     let bird1;
//     let mut bird_vec: Vec<Bird> = Vec::new();
//
//     unsafe {
//         let data_ref = &data as *const State;
//         bird1 = Bird {x: 1, pos: Real2D{x: 1.0, y: 1.0}, state: &*data_ref};
//         let bird2 = Bird {x: 2, pos: Real2D{x: 2.0, y: 2.0}, state: &*data_ref};
//         let bird3 = Bird {x: 3, pos: Real2D{x: 3.0, y: 3.0}, state: &*data_ref};
//
//         data.field1.set_object_location(bird1.clone());
//         data.field1.set_object_location(bird2.clone());
//         data.field1.set_object_location(bird3.clone());
//
//         bird_vec.push(bird1.clone());
//         bird_vec.push(bird2);
//         bird_vec.push(bird3);
//
//     }
//
//     let x = bird1.state.field1.get_neighbors_within_distance(&bird1);
//     assert_eq!(bird_vec, x);
//
//     // let bird = Bird{x: 1};
//     // let pa = AgentImpl::new(bird);
//     // let pa_clone = pa.clone();
//     // let mut field : Field<Bird> = Default::default();
//     // field.hash_map.insert(1, pa);
//     // assert_eq!(Some(&pa_clone), field.hash_map.get(&1));
// }

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

#[derive(Clone)]
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
        self.id == other.id && self.pos == other.pos
    }
}

impl<'a> Agent for Bird<'a> {
    fn step(&self) {
        let pos = Real2D {
            x: 1.0,
            y: 2.0,
        };

        let vec = self.state.field1.get_neighbors_within_distance(self, pos, 5.0);
        for elem in vec {
            println!("{}", elem.id);
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
