extern crate priority_queue;

use std::hash::Hasher;
use std::hash::Hash;
use abm::location::Real2D;
use abm::field2D::Field2D;
use std::fmt;
use abm::agent::Agent;
//use abm::schedule::Schedule;
use abm::location::Location2D;

//static STEP: u128 = 10;
// static NUM_AGENT: u128 = 2;
// static WIDTH: f64 = 10.0;
// static HEIGTH: f64 = 10.0;
// static DISCRETIZATION: f64 = 2.0;
// static TOROIDAL: bool = true;

//static mut COUNT: u32 = 0;

#[test]
fn field_2d_test_2_1() {

    let width = 10.0;
    let heigth = 10.0;
    let discretization = 0.5;
    let toroidal = true;

    let mut data = State::new(width, heigth, discretization, toroidal);

    let data_ref = &data as *const State;

    unsafe {
        //TODO bound circle
        let bird1 = Bird::new(1, Real2D{x: 5.0, y: 5.0}, &*data_ref);
        let mut bird2 = Bird::new(2, Real2D{x: 5.0, y: 6.0}, &*data_ref);
        let bird3 = Bird::new(3, Real2D{x: 5.0, y: 6.99}, &*data_ref);
        let bird4 = Bird::new(4, Real2D{x: 6.0, y: 6.0}, &*data_ref);
        let bird5 = Bird::new(5, Real2D{x: 7.0, y: 7.0}, &*data_ref);
        let mut bird6 = Bird::new(6, Real2D{x: 9.0, y: 9.0}, &*data_ref);

        data.field1.set_object_location(bird1, bird1.pos);
        data.field1.set_object_location(bird2, bird2.pos);
        data.field1.set_object_location(bird3, bird3.pos);
        data.field1.set_object_location(bird4, bird4.pos);
        data.field1.set_object_location(bird5, bird5.pos);
        data.field1.set_object_location(bird6, bird6.pos);

        assert_eq!(6, data.field1.num_objects());

        let vec = data.field1.get_neighbors_within_distance(Real2D{x: 5.0, y: 5.0}, 5.0);
        assert_eq!(5, vec.len());
        assert!(vec.contains(&bird1));
        assert!(vec.contains(&bird2));
        assert!(vec.contains(&bird3));
        assert!(vec.contains(&bird4));
        assert!(vec.contains(&bird5));

        let vec = data.field1.get_neighbors_within_distance(Real2D{x: 5.0, y: 5.0}, 2.0);

        assert_eq!(4, vec.len());
        assert!(vec.contains(&bird1));
        assert!(vec.contains(&bird2));
        assert!(vec.contains(&bird3));
        assert!(vec.contains(&bird4));

        let vec = data.field1.get_neighbors_within_distance(Real2D{x: 9.0, y: 9.0}, 1.0);

        assert_eq!(1, vec.len());
        assert!(vec.contains(&bird6));

        let vec = data.field1.get_neighbors_within_distance(Real2D{x: 9.0, y: 9.0}, 5.0);

        assert_eq!(5, vec.len());
        assert!(vec.contains(&bird5));
        assert!(vec.contains(&bird2));
        assert!(vec.contains(&bird3));
        assert!(vec.contains(&bird4));
        assert!(vec.contains(&bird6));

        // let vec = data.field1.get_neighbors_within_distance(Real2D{x: 1.0, y: 1.0}, 5.0);
        //
        // assert_eq!(1, vec.len());
        // let pos_b2 = match data.field1.get_object_location(bird2) {
        //     Some(i) => i,
        //     None => panic!("non trovato"),
        // };
        // println!("pos b2 {}", pos_b2);

        bird2.pos = Real2D {x:0.5, y:0.5};
        bird6.pos = Real2D {x:7.5, y:7.5};
        data.field1.set_object_location(bird2, bird2.pos);
        data.field1.set_object_location(bird6, bird6.pos);
        assert_eq!(6, data.field1.num_objects());
        // let pos_b2 = match data.field1.get_object_location(bird2) {
        //     Some(i) => i,
        //     None => panic!("non trovato"),
        // };
        // println!("pos post aggiornamento {}", pos_b2);
        let vec = data.field1.get_neighbors_within_distance(Real2D{x: 5.0, y: 5.0}, 4.0);

        for elem in vec.clone() {
            println!("{}", elem);
        }

        //assert_eq!(5, vec.len());
        assert!(vec.contains(&bird1));
        assert!(vec.contains(&bird6));
        assert!(vec.contains(&bird3));
        assert!(vec.contains(&bird4));
        assert!(vec.contains(&bird5));
    }
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
        let pos = Real2D {
            x: 1.0,
            y: 2.0,
        };

        let vec = self.state.field1.get_neighbors_within_distance(pos, 5.0);
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

//
// fn distance(pos1: &Real2D, pos2: &Real2D, dim1: f64, dim2: f64, tor: bool) -> f64{
//
//     let dx;
//     let dy;
//
//     if tor {
//         dx = toroidal_distance(pos1.x, pos2.x, dim1);
//         dy = toroidal_distance(pos1.y, pos2.y, dim2);
//     } else {
//         dx = pos1.x - pos2.x;
//         dy = pos1.y - pos2.y;
//     }
//     (dx*dx + dy*dy).sqrt()
// }
//
// fn toroidal_distance(val1: f64, val2: f64, dim: f64) -> f64{
//
//     if (val1 - val2).abs() <= dim/2.0 {
//         return val1 - val2;
//     }
//
//     let d = toroidal_transform(val1, dim) - toroidal_transform(val2, dim);
//
//     if d*2.0 > dim {
//         d - dim
//     } else if d*2.0 < dim {
//         d + dim
//     } else {
//         d
//     }
// }
//
// fn toroidal_transform(val: f64, dim: f64) -> f64 {
//
//     if val >= 0.0 && val < dim {
//         val
//     } else {
//         let val = val%dim;
//         if val < 0.0 {
//             let _val = val + dim;
//         }
//         val
//     }
// }
