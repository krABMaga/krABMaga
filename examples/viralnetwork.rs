extern crate rust_ab;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::field::field_2d::toroidal_transform;
use rust_ab::engine::field::field_2d::toroidal_distance;
use rust_ab::engine::field::field_2d::Field2D;
use rust_ab::engine::field::network::Network;
use rust_ab::engine::location::Location2D;
use rust_ab::engine::location::Real2D;
use rust_ab::engine::state::State;
use rand::Rng;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::Instant;
use rust_ab::engine::field::field::Field;
use rust_ab::engine::schedule::Schedule;

static mut _COUNT: u128 = 0;
static STEP: u128 = 50;
static NUM_AGENT: u128 = 8390;
static WIDTH: f64 = 208.0;
static HEIGTH: f64 = 208.0;
static DISCRETIZATION: f64 = 10.0 / 1.5;
static TOROIDAL: bool = true;
static COHESION: f64 = 1.0;
static AVOIDANCE: f64 = 1.0;
static RANDOMNESS: f64 = 1.0;
static CONSISTENCY: f64 = 1.0;
static MOMENTUM: f64 = 1.0;
static JUMP: f64 = 0.7;


fn main() {
    let mut rng = rand::thread_rng();
    let mut schedule: Schedule<SusceptibleIndividual> = Schedule::new();
    // assert!(schedule.events.is_empty());

    let mut state = EpidemicNetworkState::new(WIDTH, HEIGTH, DISCRETIZATION, TOROIDAL);
    for User_id in 0..NUM_AGENT {
        
        let r1: f64 = rng.gen();
        let r2: f64 = rng.gen();
        let last_d = Real2D { x: 0.0, y: 0.0 };
        let user = SusceptibleIndividual::new(
            User_id,
            Real2D {
                x: WIDTH * r1,
                y: HEIGTH * r2,
            },
            last_d,
        );
        state
            .field1
            .set_object_location(user, user.pos);
    
        schedule.schedule_repeating(user, 0.0, 0);
    }

    // assert!(!schedule.events.is_empty());

    let start = Instant::now();

    for _ in 0..STEP {
        schedule.step(&mut state);
    }

    let run_duration = start.elapsed();

    println!("Time elapsed in testing schedule is: {:?}", run_duration);
    println!("(boids)Total Step:{}\nStep for seconds: {:?}",
        schedule.step_count(),
        schedule.step_count() as f64 /(run_duration.as_nanos() as f64 * 1e-9)
    );
}

pub struct EpidemicNetworkState {
    pub field1: Field2D<SusceptibleIndividual>,
    pub network: Network<SusceptibleIndividual, String>,
}

impl EpidemicNetworkState {
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> EpidemicNetworkState {
        EpidemicNetworkState {
            field1: Field2D::new(w, h, d, t),
            network: Network::new(false),
        }
    }
}

impl State for EpidemicNetworkState{
    fn update(&self){
        self.field1.lazy_update();
    }
}


#[derive(Clone, Copy)]
pub struct SusceptibleIndividual {
    pub id: u128,
    pub pos: Real2D,
    pub last_d: Real2D,
}

impl SusceptibleIndividual {
    pub fn new(id: u128, pos: Real2D, last_d: Real2D) -> Self {
        SusceptibleIndividual { id, pos, last_d }
    }

    pub fn avoidance(self, vec: &Vec<&SusceptibleIndividual>) -> Real2D {
        if vec.is_empty() {
            let real = Real2D { x: 0.0, y: 0.0 };
            return real;
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            if self != *vec[i] {
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

    pub fn cohesion(self, vec: &Vec<&SusceptibleIndividual>) -> Real2D {
        if vec.is_empty() {
            let real = Real2D { x: 0.0, y: 0.0 };
            return real;
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            if self != *vec[i] {
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

    pub fn consistency(self, vec: &Vec<&SusceptibleIndividual>) -> Real2D {
        if vec.is_empty() {
            let real = Real2D { x: 0.0, y: 0.0 };
            return real;
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            if self != *vec[i] {
                let _dx = toroidal_distance(self.pos.x, vec[i].pos.x, WIDTH);
                let _dy = toroidal_distance(self.pos.y, vec[i].pos.y, HEIGTH);
                count += 1;
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

impl Agent for SusceptibleIndividual {
    type SimState = EpidemicNetworkState;

    fn step(&mut self, state:&EpidemicNetworkState) {
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

        self.last_d = Real2D { x: dx, y: dy };
        let loc_x = toroidal_transform(self.pos.x + dx, WIDTH);
        let loc_y = toroidal_transform(self.pos.y + dy, WIDTH);
        
        self.pos = Real2D { x: loc_x, y: loc_y };
        drop(vec);
        state
            .field1
            .set_object_location(*self, Real2D { x: loc_x, y: loc_y });
    }

    fn id(&self)->u128{
        self.id
    }
}

impl Hash for SusceptibleIndividual {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
        //    state.write_u128(self.id);
        //    state.finish();
    }
}

impl Eq for SusceptibleIndividual {}

impl PartialEq for SusceptibleIndividual {
    fn eq(&self, other: &SusceptibleIndividual) -> bool {
        self.id == other.id
    }
}

impl Location2D<Real2D> for SusceptibleIndividual {
    fn get_location(self) -> Real2D {
        self.pos
    }

    fn set_location(&mut self, loc: Real2D) {
        self.pos = loc;
    }
}

impl fmt::Display for SusceptibleIndividual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
