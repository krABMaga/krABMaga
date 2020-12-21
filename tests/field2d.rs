extern crate abm;
extern crate priority_queue;

#[macro_use]
extern crate lazy_static;

use abm::agent::Agent;
use abm::field_2d::toroidal_distance;
use abm::field_2d::toroidal_transform;
use rand::Rng;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Mutex;
//use abm::schedule::Schedule;
use abm::field_2d::Field2D;
use abm::location::Location2D;
use abm::location::Real2D;

static mut _COUNT: u128 = 0;
static _STEP: u128 = 10;
static _NUM_AGENT: u128 = 1000;
static WIDTH: f64 = 10.0;
static HEIGTH: f64 = 10.0;
static DISCRETIZATION: f64 = 0.5;
static TOROIDAL: bool = true;
static COHESION: f64 = 1.0;
static AVOIDANCE: f64 = 1.0;
static RANDOMNESS: f64 = 1.0;
static CONSISTENCY: f64 = 1.0;
static MOMENTUM: f64 = 1.0;
static JUMP: f64 = 0.7;

lazy_static! {
    static ref GLOBAL_STATE: Mutex<State> =
        Mutex::new(State::new(WIDTH, HEIGTH, DISCRETIZATION, TOROIDAL));
}

lazy_static! {
    static ref GLOBAL_STATE_2: Mutex<State> =
        Mutex::new(State::new(150.0, 150.0, 10.0 / 1.5, TOROIDAL));
}

#[test]
fn field_2d_test_2_1() {
    let last_d = Real2D { x: 0.0, y: 0.0 };

    let bird1 = Bird::new(1, Real2D { x: 5.0, y: 5.0 }, last_d);
    let mut bird2 = Bird::new(2, Real2D { x: 5.0, y: 6.0 }, last_d);
    let bird3 = Bird::new(3, Real2D { x: 5.0, y: 7.0 }, last_d);
    let bird4 = Bird::new(4, Real2D { x: 6.0, y: 6.0 }, last_d);
    let bird5 = Bird::new(5, Real2D { x: 7.0, y: 7.0 }, last_d);
    let mut bird6 = Bird::new(6, Real2D { x: 9.0, y: 9.0 }, last_d);

    GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird1, bird1.pos);
    GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird2, bird2.pos);
    GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird3, bird3.pos);
    GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird4, bird4.pos);
    GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird5, bird5.pos);
    GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird6, bird6.pos);

    assert_eq!(6, GLOBAL_STATE.lock().unwrap().field1.num_objects());

    let vec = GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .get_neighbors_within_distance(Real2D { x: 5.0, y: 5.0 }, 5.0);
    assert_eq!(5, vec.len());
    assert!(vec.contains(&bird1));
    assert!(vec.contains(&bird2));
    assert!(vec.contains(&bird3));
    assert!(vec.contains(&bird4));
    assert!(vec.contains(&bird5));
    assert!(!vec.contains(&bird6));

    let vec = GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .get_neighbors_within_distance(Real2D { x: 5.0, y: 5.0 }, 2.0);

    assert_eq!(4, vec.len());
    assert!(vec.contains(&bird1));
    assert!(vec.contains(&bird2));
    assert!(vec.contains(&bird3));
    assert!(vec.contains(&bird4));

    let vec = GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .get_neighbors_within_distance(Real2D { x: 9.0, y: 9.0 }, 1.0);

    assert_eq!(1, vec.len());
    assert!(vec.contains(&bird6));

    let vec = GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .get_neighbors_within_distance(Real2D { x: 9.0, y: 9.0 }, 5.0);

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

    bird2.pos = Real2D { x: 0.5, y: 0.5 };
    bird6.pos = Real2D { x: 7.5, y: 7.5 };
    GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird2, bird2.pos);
    GLOBAL_STATE
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird6, bird6.pos);
    assert_eq!(6, GLOBAL_STATE.lock().unwrap().field1.num_objects());
    // let pos_b2 = match data.field1.get_object_location(bird2) {
    //     Some(i) => i,
    //     None => panic!("non trovato"),
    // };
    // println!("pos post aggiornamento {}", pos_b2);

    // for (key, val) in data.field1.fpos.iter() {
    //     println!("key: {} val: {}", key, val);
    // }
}

#[test]
fn field_2d_test_vicinato() {
    let last_d = Real2D { x: 0.0, y: 0.0 };

    let mut bird1 = Bird::new(1, Real2D { x: 0.0, y: 0.0 }, last_d);
    let mut bird2 = Bird::new(2, Real2D { x: 0.0, y: 0.0 }, last_d);

    GLOBAL_STATE_2
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird1, bird1.pos);
    GLOBAL_STATE_2
        .lock()
        .unwrap()
        .field1
        .set_object_location(bird2, bird2.pos);

    assert_eq!(2, GLOBAL_STATE_2.lock().unwrap().field1.num_objects());

    for i in 0..10 {
        println!("step {}", i);
        bird1.pos = Real2D {
            x: bird1.pos.x + 1.0,
            y: bird1.pos.y + 1.0,
        };
        bird2.pos = Real2D {
            x: bird2.pos.x + 1.0,
            y: bird2.pos.y + 1.0,
        };

        GLOBAL_STATE_2
            .lock()
            .unwrap()
            .field1
            .set_object_location(bird1, bird1.pos);
        GLOBAL_STATE_2
            .lock()
            .unwrap()
            .field1
            .set_object_location(bird2, bird2.pos);

        let vec = GLOBAL_STATE_2
            .lock()
            .unwrap()
            .field1
            .get_neighbors_within_distance(bird1.pos, 10.0);
        assert_eq!(2, vec.len());
        assert!(vec.contains(&bird1));
        assert!(vec.contains(&bird2));
        let vec = GLOBAL_STATE_2
            .lock()
            .unwrap()
            .field1
            .get_neighbors_within_distance(bird2.pos, 10.0);
        assert_eq!(2, vec.len());
        assert!(vec.contains(&bird1));
        assert!(vec.contains(&bird2));
    }
}
//
// #[test]
// fn field_2d_test_2_2() {
//     let width = 10.0;
//     let heigth = 10.0;
//     let discretization = 0.5;
//     let toroidal = true;
//
//     let mut data = State::new(width, heigth, discretization, toroidal);
//
//     let data_ref = &data as *const State;
//
//     unsafe {
//         //TODO bound circle
//         let bird1 = Bird::new(1, Real2D{x: 5.0, y: 5.0}, &*data_ref);
//         let mut bird2 = Bird::new(2, Real2D{x: 5.0, y: 6.0}, &*data_ref);
//         let bird3 = Bird::new(3, Real2D{x: 5.0, y: 7.0}, &*data_ref);
//         let bird4 = Bird::new(4, Real2D{x: 6.0, y: 6.0}, &*data_ref);
//         let bird5 = Bird::new(5, Real2D{x: 7.0, y: 7.0}, &*data_ref);
//         let mut bird6 = Bird::new(6, Real2D{x: 9.0, y: 9.0}, &*data_ref);
//
//         data.field1.set_object_location(bird1, bird1.pos);
//         data.field1.set_object_location(bird2, bird2.pos);
//         data.field1.set_object_location(bird3, bird3.pos);
//         data.field1.set_object_location(bird4, bird4.pos);
//         data.field1.set_object_location(bird5, bird5.pos);
//         data.field1.set_object_location(bird6, bird6.pos);
//
//         bird2.pos = Real2D {x:0.5, y:0.5};
//         bird6.pos = Real2D {x:7.5, y:7.5};
//         data.field1.set_object_location(bird2, bird2.pos);
//         data.field1.set_object_location(bird6, bird6.pos);
//
//         let vec = data.field1.get_neighbors_within_distance(Real2D{x: 5.0, y: 5.0}, 4.0);
//
//         // for elem in vec.clone() {
//         //     println!("{}", elem);
//         // }
//
//         assert_eq!(5, vec.len());
//         assert!(vec.contains(&bird1));
//         assert!(vec.contains(&bird6));
//         assert!(vec.contains(&bird3));
//         assert!(vec.contains(&bird4));
//         assert!(vec.contains(&bird5));
//     }
// }
//
//
// #[test]
// fn field_2d_test_2_3() {
//     let width = 10.0;
//     let heigth = 10.0;
//     let discretization = 0.5;
//     let toroidal = true;
//
//     let mut data = State::new(width, heigth, discretization, toroidal);
//
//     let data_ref = &data as *const State;
//
//     unsafe {
//         let bird1 = Bird::new(1, Real2D{x: 5.0, y: 5.0}, &*data_ref);
//         let mut bird2 = Bird::new(2, Real2D{x: 5.0, y: 6.0}, &*data_ref);
//         let bird3 = Bird::new(3, Real2D{x: 5.0, y: 7.0}, &*data_ref);
//         let bird4 = Bird::new(4, Real2D{x: 6.0, y: 6.0}, &*data_ref);
//         let bird5 = Bird::new(5, Real2D{x: 7.0, y: 7.0}, &*data_ref);
//         let mut bird6 = Bird::new(6, Real2D{x: 9.0, y: 9.0}, &*data_ref);
//
//         data.field1.set_object_location(bird1, bird1.pos);
//         data.field1.set_object_location(bird2, bird2.pos);
//         data.field1.set_object_location(bird3, bird3.pos);
//         data.field1.set_object_location(bird4, bird4.pos);
//         data.field1.set_object_location(bird5, bird5.pos);
//         data.field1.set_object_location(bird6, bird6.pos);
//
//         let vec = data.field1.get_neighbors_within_distance(Real2D{x: 5.0, y: 5.0}, 2.0);
//
//         assert_eq!(4, vec.len());
//         assert!(vec.contains(&bird1));
//         assert!(vec.contains(&bird2));
//         assert!(vec.contains(&bird3));
//         assert!(vec.contains(&bird4));
//
//         let vec = data.field1.get_neighbors_within_distance(Real2D{x: 9.0, y: 9.0}, 1.0);
//
//         assert_eq!(1, vec.len());
//         assert!(vec.contains(&bird6));
//
//         let vec = data.field1.get_neighbors_within_distance(Real2D{x: 9.0, y: 9.0}, 5.0);
//
//         assert_eq!(5, vec.len());
//         assert!(vec.contains(&bird5));
//         assert!(vec.contains(&bird2));
//         assert!(vec.contains(&bird3));
//         assert!(vec.contains(&bird4));
//         assert!(vec.contains(&bird6));
//
//         // let vec = data.field1.get_neighbors_within_distance(Real2D{x: 1.0, y: 1.0}, 5.0);
//         //
//         // assert_eq!(1, vec.len());
//         // let pos_b2 = match data.field1.get_object_location(bird2) {
//         //     Some(i) => i,
//         //     None => panic!("non trovato"),
//         // };
//         // println!("pos b2 {}", pos_b2);
//
//         bird2.pos = Real2D {x:0.5, y:0.5};
//         bird6.pos = Real2D {x:7.5, y:7.5};
//         data.field1.set_object_location(bird2, bird2.pos);
//         data.field1.set_object_location(bird6, bird6.pos);
//         assert_eq!(6, data.field1.num_objects());
//     }
// }

pub struct State {
    pub field1: Field2D<Bird>,
}

impl State {
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> State {
        State {
            field1: Field2D::new(w, h, d, t),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Bird {
    pub id: u128,
    pub pos: Real2D,
    pub last_d: Real2D,
}

impl Bird {
    pub fn new(id: u128, pos: Real2D, last_d: Real2D) -> Self {
        Bird { id, pos, last_d }
    }

    pub fn avoidance(self, vec: &Vec<Bird>) -> Real2D {
        if vec.is_empty() {
            let real = Real2D { x: 0.0, y: 0.0 };
            return real;
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            if self != vec[i] {
                let dx = toroidal_distance(self.pos.x, vec[i].pos.x, WIDTH);
                let dy = toroidal_distance(self.pos.y, vec[i].pos.y, HEIGTH);
                let square = (dx * dx + dy * dy).sqrt();
                count += 1;
                x += dx / (square * square) + 1.0;
                y += dy / (square * square) + 1.0;
            }
        }
        if count > 0 {
            x = x / count as f64;
            y = y / count as f64;
            let real = Real2D {
                x: 400.0 * x,
                y: 400.0 * y,
            };
            return real;
        } else {
            let real = Real2D {
                x: 400.0 * x,
                y: 400.0 * y,
            };
            return real;
        }
    }

    pub fn cohesion(self, vec: &Vec<Bird>) -> Real2D {
        if vec.is_empty() {
            let real = Real2D { x: 0.0, y: 0.0 };
            return real;
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            //CONDIZIONE?
            if self != vec[i] {
                let dx = toroidal_distance(self.pos.x, vec[i].pos.x, WIDTH);
                let dy = toroidal_distance(self.pos.y, vec[i].pos.y, HEIGTH);
                count += 1;
                x += dx;
                y += dy;
            }
        }
        if count > 0 {
            x = x / count as f64;
            y = y / count as f64;
            let real = Real2D {
                x: -x / 10.0,
                y: -y / 10.0,
            };
            return real;
        } else {
            let real = Real2D {
                x: -x / 10.0,
                y: -y / 10.0,
            };
            return real;
        }
    }

    pub fn randomness(self) -> Real2D {
        let mut rng = rand::thread_rng();
        let r1: f64 = rng.gen();
        let x = r1 * 2.0 - 1.0;
        let r2: f64 = rng.gen();
        let y = r2 * 2.0 - 1.0;

        let square = (x * x + y * y).sqrt();
        let real = Real2D {
            x: 0.05 * x / square,
            y: 0.05 * y / square,
        };
        return real;
    }

    pub fn consistency(self, vec: &Vec<Bird>) -> Real2D {
        if vec.is_empty() {
            let real = Real2D { x: 0.0, y: 0.0 };
            return real;
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            //CONDIZIONE?
            if self != vec[i] {
                let _dx = toroidal_distance(self.pos.x, vec[i].pos.x, WIDTH);
                let _dy = toroidal_distance(self.pos.y, vec[i].pos.y, HEIGTH);
                count += 1;
                //momentum
                x += self.pos.x;
                y += self.pos.y;
            }
        }
        if count > 0 {
            x = x / count as f64;
            y = y / count as f64;
            let real = Real2D {
                x: -x / count as f64,
                y: y / count as f64,
            };
            return real;
        } else {
            let real = Real2D { x: x, y: y };
            return real;
        }
    }
}

impl Agent for Bird {
    fn step(&mut self) {
        let vec = GLOBAL_STATE
            .lock()
            .unwrap()
            .field1
            .get_neighbors_within_distance(self.pos, 10.0);
        // {
        //     let fpos = GLOBAL_STATE.lock().unwrap();
        //     let fpos = fpos.field1.get_object_location(*self);
        //     let fpos = fpos.unwrap();
        //     println!("{} {} {} {}", self.id, self.pos,fpos,vec.len());
        //
        // }

        let avoid = self.avoidance(&vec);
        let cohe = self.cohesion(&vec);
        let rand = self.randomness();
        let cons = self.consistency(&vec);
        let mom = self.last_d;

        let mut dx = COHESION * cohe.x
            + AVOIDANCE * avoid.x
            + CONSISTENCY * cons.x
            + RANDOMNESS * rand.x
            + MOMENTUM * mom.x;
        let mut dy = COHESION * cohe.y
            + AVOIDANCE * avoid.y
            + CONSISTENCY * cons.y
            + RANDOMNESS * rand.y
            + MOMENTUM * mom.y;

        let dis = (dx * dx + dy * dy).sqrt();
        if dis > 0.0 {
            dx = dx / dis * JUMP;
            dy = dy / dis * JUMP;
        }

        let _lastd = Real2D { x: dx, y: dy };
        let loc_x = toroidal_transform(self.pos.x + dx, WIDTH);
        let loc_y = toroidal_transform(self.pos.y + dy, WIDTH);

        self.pos = Real2D { x: loc_x, y: loc_y };

        GLOBAL_STATE
            .lock()
            .unwrap()
            .field1
            .set_object_location(*self, Real2D { x: loc_x, y: loc_y });
    }
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

impl Eq for Bird {}

impl PartialEq for Bird {
    fn eq(&self, other: &Bird) -> bool {
        self.id == other.id
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
