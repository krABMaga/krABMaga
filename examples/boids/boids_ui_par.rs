extern crate abm;
extern crate piston_window;
extern crate priority_queue;

use piston_window::*;

#[macro_use]
extern crate lazy_static;

use abm::agent::Agent;
use abm::field_2d_double_buffer::toroidal_distance;
use abm::field_2d_double_buffer::toroidal_transform;
use abm::field_2d_double_buffer::Field2D;
use abm::location::Location2D;
use abm::location::Real2D;
use abm::par_schedule::Schedule;
use rand::Rng;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::{thread, time};
use abm::state::State;
use abm::field::DoubleBufferedField;

static mut _COUNT: u128 = 0;

static NUM_AGENT: u128 = 100;
static WIDTH: f64 = 150.0;
static HEIGTH: f64 = 150.0;
static DISCRETIZATION: f64 = 10.0 / 1.5;
static TOROIDAL: bool = true;
static COHESION: f64 = 1.0;
static AVOIDANCE: f64 = 1.0;
static RANDOMNESS: f64 = 1.0;
static CONSISTENCY: f64 = 1.0;
static MOMENTUM: f64 = 1.0;
static JUMP: f64 = 0.7;

lazy_static! {
    static ref GLOBAL_STATE: BoidsState = BoidsState::new(WIDTH, HEIGTH, DISCRETIZATION, TOROIDAL);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut schedule: Schedule<Bird> = Schedule::with_threads(4);
    // assert!(schedule.events.is_empty());

    for bird_id in 0..NUM_AGENT {
        let r1: f64 = rng.gen();
        let r2: f64 = rng.gen();

        let last_d = Real2D { x: 0.0, y: 0.0 };

        let bird = Bird::new(
            bird_id,
            Real2D {
                x: WIDTH * r1,
                y: HEIGTH * r2,
            },
            last_d,
        );

        GLOBAL_STATE
            .field1
            .set_object_location(bird, bird.pos);
        schedule.schedule_repeating(bird, 0.0, 0);
    }
    // assert!(!schedule.events.is_empty());
   

    // let start = Instant::now();

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
    while let Some(e) = window.next() {
        schedule.step(&*GLOBAL_STATE);
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            for (_key, value) in GLOBAL_STATE.field1.r_fpos.read().unwrap().iter() {
                let shape = polygon::Polygon::new([1.0, 0.0, 0.0, 1.0]);

                let rotation;
                let size = 5.0;
                let radius = size / 2.0;

                if (value.y.sin() == 0.0) && (value.x.cos() == 0.0) {
                    rotation = 0.0;
                } else {
                    rotation = value.y.atan2(value.x);
                }

                let transform = c
                    .transform
                    .trans(value.x * 5.0, value.y * 5.0)
                    .rot_rad(rotation)
                    .trans(-radius, -radius);

                let point = [[0.0, radius], [size, size], [size, 0.0]];

                shape.draw(&point, &c.draw_state, transform, g);
                
            }
        });
        let ten_millis = time::Duration::from_millis(50);
        thread::sleep(ten_millis);
    }

    // let duration = start.elapsed();

    //    println!("Time elapsed in testing schedule is: {:?}", duration);
    //    println!("Step for seconds: {:?}", STEP as u64/duration.as_secs());
}

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

impl State for BoidsState{
    fn update(&self){
        self.field1.update();
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
                let square = dx * dx + dy * dy;
                count += 1;
                x += dx / (square * square + 1.0);
                y += dy / (square * square + 1.0);
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
            if self != vec[i] {
                let xx = vec[i].last_d.x;
                let yy = vec[i].last_d.y;
                count += 1;
                x += xx;
                y += yy;
            }
        }
        if count > 0 {
            x = x / count as f64;
            y = y / count as f64;
            let real = Real2D {
                x: x / count as f64,
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
    type SimState = BoidsState;

    fn step(&mut self, state: &BoidsState) {
        let vec = state
            .field1
            .get_neighbors_within_distance(self.pos, 10.0);

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
        let loc_y = toroidal_transform(self.pos.y + dy, HEIGTH);
        self.last_d = _lastd;
        self.pos = Real2D {
            x: loc_x + dx,
            y: loc_y + dy,
        };

        state
            .field1
            .set_object_location(*self, self.pos);
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
