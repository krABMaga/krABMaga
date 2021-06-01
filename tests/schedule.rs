extern crate priority_queue;

use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

use rust_ab::engine::agent::Agent;
use rust_ab::engine::field::field::Field;
use rust_ab::engine::field::field_2d::Field2D;
use rust_ab::engine::location::{Location2D, Real2D};
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;

static STEP: u128 = 10;
// static NUM_AGENT: u128 = 2;
// static WIDTH: f64 = 10.0;
// static HEIGTH: f64 = 10.0;
// static DISCRETIZATION: f64 = 2.0;
// static TOROIDAL: bool = true;

//static mut COUNT: u32 = 0;

#[test]
fn field_2d_test_1() {
    let width = 100.0;
    let heigth = 100.0;
    let discretization = 0.7;
    let toroidal = true;

    let mut data = BoidsState::new(width, heigth, discretization, toroidal);
    let mut schedule = Schedule::new();
    //assert!(schedule.events.is_empty());

    let bird1 = Bird::new(1, Real2D { x: 10.5, y: 10.0 });
    let bird2 = Bird::new(2, Real2D { x: 10.0, y: 10.0 });

    // Let's try to write only bird1 into the field
    data.field1.set_object_location(bird1, bird1.pos);

    data.update(schedule.step);

    let pos = match data.field1.get_object_location(bird1) {
        Some(i) => i,
        None => panic!("no location"),
    };

    // Was bird1 position written correctly?
    let real_pos = Real2D { x: 10.5, y: 10.0 };
    assert_eq!(real_pos, *pos);

    // Now let's try writing only bird2
    data.field1
        .set_object_location(bird1, Real2D { x: 10.0, y: 10.0 });

    data.update(schedule.step);

    let pos = match data.field1.get_object_location(bird1) {
        Some(i) => i,
        None => panic!("no location"),
    };

    // Was bird2 position written correctly?
    let real_pos = Real2D { x: 10.0, y: 10.0 };
    assert_eq!(real_pos, *pos);

    // Now let's write two birds in the same position
    data.field1.set_object_location(bird1, real_pos);
    data.field1.set_object_location(bird2, real_pos);

    data.update(schedule.step);

    // Is the field correctly telling us there are two objects at the position used previously?
    let num = data.field1.num_objects_at_location(real_pos);
    assert_eq!(2, num);

    schedule.schedule_repeating(bird1, 5.0, 100);
    schedule.schedule_repeating(bird2, 5.0, 100);

    //assert!(!schedule.events.is_empty());

    for _ in 1..STEP {
        schedule.step(&mut data);
        data.field1.set_object_location(bird1, real_pos);
        data.field1.set_object_location(bird2, real_pos);
    }
}

#[test]
fn field_2d_test_2() {
    let width = 10.0;
    let heigth = 10.0;
    let discretization = 0.5;
    let toroidal = false;

    let mut data = BoidsState::new(width, heigth, discretization, toroidal);
    //let mut schedule: Schedule<Bird> = Schedule::new();
    //assert!(schedule.events.is_empty());

    let bird1 = Bird::new(1, Real2D { x: 5.5, y: 5.5 });
    let bird2 = Bird::new(2, Real2D { x: 4.0, y: 4.0 });
    let bird3 = Bird::new(3, Real2D { x: 5.2, y: 5.2 });
    let bird4 = Bird::new(4, Real2D { x: 5.2, y: 2.2 });
    let bird5 = Bird::new(5, Real2D { x: 5.2, y: 2.1 });

    data.field1
        .set_object_location(bird1.clone(), bird1.pos.clone());
    data.field1
        .set_object_location(bird2.clone(), bird2.pos.clone());
    data.field1
        .set_object_location(bird3.clone(), bird3.pos.clone());
    data.field1
        .set_object_location(bird4.clone(), bird4.pos.clone());
    data.field1
        .set_object_location(bird5.clone(), bird5.pos.clone());

    data.update(0);

    let vec = data
        .field1
        .get_neighbors_within_distance(Real2D { x: 5.2, y: 5.2 }, 3.0);
    assert_eq!(4, vec.len());

    // bird2.pos = Real2D {x: 5.3, y:5.3};
    //
    // data.field1.set_object_location(bird2.clone(), bird2.pos.clone());
    // let vec = data.field1.get_neighbors_within_distance(Real2D{x: 5.2, y:5.2}, 3.0);
    // assert_eq!(5, vec.len());

    let bird6 = Bird::new(6, Real2D { x: 0.1, y: 0.1 });
    data.field1
        .set_object_location(bird1.clone(), bird1.pos.clone());
    data.field1
        .set_object_location(bird2.clone(), bird2.pos.clone());
    data.field1
        .set_object_location(bird3.clone(), bird3.pos.clone());
    data.field1
        .set_object_location(bird4.clone(), bird4.pos.clone());
    data.field1
        .set_object_location(bird5.clone(), bird5.pos.clone());
    data.field1
        .set_object_location(bird6.clone(), bird6.pos.clone());

    data.update(0);

    let vec = data
        .field1
        .get_neighbors_within_distance(Real2D { x: 5.2, y: 5.2 }, 5.0);
    assert_eq!(5, vec.len());

    let num = data
        .field1
        .num_objects_at_location(Real2D { x: 5.0, y: 2.0 });
    assert_eq!(2, num);

    //schedule.schedule_repeating(bird1, 5.0, 100);
    //schedule.schedule_repeating(bird2, 5.0, 100);

    // assert!(!schedule.events.is_empty());
    //
    // for _ in 1..STEP{
    //     schedule.step();
    // }
}

#[test]
fn field_2d_test_3() {
    let width = 10.0;
    let heigth = 10.0;
    let discretization = 0.5;
    let toroidal = false;

    let mut data = BoidsState::new(width, heigth, discretization, toroidal);
    let schedule: Schedule<Bird> = Schedule::new();
    //assert!(schedule.events.is_empty());
    let mut bird1;
    let bird2;
    bird1 = Bird::new(1, Real2D { x: 5.5, y: 5.5 });
    bird2 = Bird::new(2, Real2D { x: 4.0, y: 4.0 });

    data.field1
        .set_object_location(bird1.clone(), bird1.pos.clone());
    data.field1
        .set_object_location(bird2.clone(), bird2.pos.clone());

    data.update(schedule.step);

    let pos_b1 = match data.field1.get_object_location(bird1.clone()) {
        Some(i) => i,
        None => panic!("non trovato"),
    };
    assert_eq!(bird1.pos, *pos_b1);

    let new_pos = Real2D { x: 7.0, y: 9.2 };
    bird1.pos = new_pos.clone();
    data.field1.set_object_location(bird1.clone(), new_pos);

    data.update(schedule.step);

    let pos_b1 = match data.field1.get_object_location(bird1.clone()) {
        Some(i) => i,
        None => panic!("non trovato"),
    };
    assert_eq!(bird1.pos, *pos_b1);
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

pub struct BoidsState {
    pub field1: Field2D<Bird>,
}

impl BoidsState {
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> BoidsState {
        BoidsState {
            field1: Field2D::new(w, h, d, t),
        }
    }
}

impl rust_ab::engine::state::State for BoidsState {
    fn update(&mut self, _step: usize) {
        self.field1.update();
    }
}

#[derive(Clone, Copy)]
pub struct Bird {
    pub id: u128,
    pub pos: Real2D,
}

impl Hash for Bird {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u128(self.id);
        state.finish();
    }
}

impl Bird {
    pub fn new(id: u128, pos: Real2D) -> Self {
        Bird { id, pos }
    }
}

impl Eq for Bird {}

impl PartialEq for Bird {
    fn eq(&self, other: &Bird) -> bool {
        self.id == other.id && self.pos == other.pos
    }
}

impl Agent for Bird {
    type SimState = BoidsState;

    fn step(&mut self, state: &BoidsState) {
        let pos = Real2D { x: 1.0, y: 2.0 };
        state.field1.set_object_location(*self, pos);
    }
}

impl Location2D<Real2D> for Bird {
    fn get_location(self) -> Real2D {
        self.pos
    }

    fn set_location(&mut self, loc: Real2D) {
        self.pos = loc;
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
