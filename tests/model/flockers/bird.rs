use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::{toroidal_distance, toroidal_transform, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

use crate::model::flockers::state::Flocker;

static COHESION: f32 = 1.0;
static AVOIDANCE: f32 = 1.0;
static RANDOMNESS: f32 = 1.0;
static CONSISTENCY: f32 = 1.0;
static MOMENTUM: f32 = 1.0;
static JUMP: f32 = 0.7;

#[derive(Clone, Copy)]
pub struct Bird {
    pub id: u32,
    pub pos: Real2D,
    pub last_d: Real2D,
    pub flag: bool,
}

impl Bird {
    pub fn new(id: u32, pos: Real2D, last_d: Real2D) -> Self {
        Bird {
            id,
            pos,
            last_d,
            flag: false,
        }
    }
}

impl Agent for Bird {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any().downcast_ref::<Flocker>().unwrap();
        let vec = state.field1.get_neighbors_within_distance(self.pos, 10.0);

        let width = state.dim.0;
        let height = state.dim.1;

        let mut avoidance = Real2D { x: 0.0, y: 0.0 };

        let mut cohesion = Real2D { x: 0.0, y: 0.0 };
        let mut randomness = Real2D { x: 0.0, y: 0.0 };
        let mut consistency = Real2D { x: 0.0, y: 0.0 };

        if !vec.is_empty() {
            //avoidance
            let mut x_avoid = 0.0;
            let mut y_avoid = 0.0;
            let mut x_cohe = 0.0;
            let mut y_cohe = 0.0;
            let mut x_cons = 0.0;
            let mut y_cons = 0.0;
            let mut count = 0;

            for elem in &vec {
                if self.id != elem.id {
                    let dx = toroidal_distance(self.pos.x, elem.pos.x, width);
                    let dy = toroidal_distance(self.pos.y, elem.pos.y, height);
                    count += 1;

                    //avoidance calculation
                    let square = dx * dx + dy * dy;
                    x_avoid += dx / (square * square + 1.0);
                    y_avoid += dy / (square * square + 1.0);

                    //cohesion calculation
                    x_cohe += dx;
                    y_cohe += dy;

                    //consistency calculation
                    x_cons += elem.last_d.x;
                    y_cons += elem.last_d.y;
                }
            }

            if count > 0 {
                x_avoid /= count as f32;
                y_avoid /= count as f32;
                x_cohe /= count as f32;
                y_cohe /= count as f32;
                x_cons /= count as f32;
                y_cons /= count as f32;

                consistency = Real2D {
                    x: x_cons / count as f32,
                    y: y_cons / count as f32,
                };
            } else {
                consistency = Real2D {
                    x: x_cons,
                    y: y_cons,
                };
            }

            avoidance = Real2D {
                x: 400.0 * x_avoid,
                y: 400.0 * y_avoid,
            };

            cohesion = Real2D {
                x: -x_cohe / 10.0,
                y: -y_cohe / 10.0,
            };

            //randomness
            let mut rng = rand::thread_rng();
            let r1: f32 = rng.gen();
            let x_rand = r1 * 2.0 - 1.0;
            let r2: f32 = rng.gen();
            let y_rand = r2 * 2.0 - 1.0;

            let square = (x_rand * x_rand + y_rand * y_rand).sqrt();
            randomness = Real2D {
                x: 0.05 * x_rand / square,
                y: 0.05 * y_rand / square,
            };
        }

        let mom = self.last_d;

        let mut dx = COHESION * cohesion.x
            + AVOIDANCE * avoidance.x
            + CONSISTENCY * consistency.x
            + RANDOMNESS * randomness.x
            + MOMENTUM * mom.x;
        let mut dy = COHESION * cohesion.y
            + AVOIDANCE * avoidance.y
            + CONSISTENCY * consistency.y
            + RANDOMNESS * randomness.y
            + MOMENTUM * mom.y;

        let dis = (dx * dx + dy * dy).sqrt();
        if dis > 0.0 {
            dx = dx / dis * JUMP;
            dy = dy / dis * JUMP;
        }

        self.last_d = Real2D { x: dx, y: dy };
        let loc_x = toroidal_transform(self.pos.x + dx, width);
        let loc_y = toroidal_transform(self.pos.y + dy, width);

        self.pos = Real2D { x: loc_x, y: loc_y };
        drop(vec);
        state
            .field1
            .set_object_location(*self, Real2D { x: loc_x, y: loc_y });
    }
}

impl Hash for Bird {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
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
        write!(f, "{} pos {}", self.id, self.pos)
    }
}
