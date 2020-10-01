use std::f64::consts::PI;

use abm::{field2D::Field2D, field2D::toroidal_distance, field2D::toroidal_transform, location::Real2D};
use amethyst::{core::Transform, ecs::{Join, System, WriteExpect, WriteStorage}};
use rand::Rng;

use crate::{agent_adapter::AgentAdapter, environment::HEIGHT, environment::WIDTH};

pub const COHESION : f64 = 0.1;
pub const AVOIDANCE : f64 = 1.0;
pub const RANDOMNESS : f64 = 1.0;
pub const CONSISTENCY : f64 = 1.0;
pub const MOMENTUM : f64 = 1.0;
pub const JUMP : f64 = 0.7;
pub const NEIGHBOR_DISTANCE : f64 = 12.5;

pub struct FlockerSystem;

// Vector calculation helper methods.
impl FlockerSystem {
    
    fn avoidance (agent: &AgentAdapter, vec: &Vec<AgentAdapter>) -> Real2D {
        if vec.is_empty() {
            let real = Real2D {x: 0.0, y: 0.0};
            return real;
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            if *agent != vec[i] {
                let dx = toroidal_distance(agent.pos.x, vec[i].pos.x, WIDTH.into());
                let dy = toroidal_distance(agent.pos.y, vec[i].pos.y, HEIGHT.into());
                let square = dx*dx + dy*dy;
                count += 1;
                x += dx/(square*square + 1.0);
                y += dy/(square*square + 1.0);
            }
        }
        if count > 0 {
            x = x/count as f64;
            y = y/count as f64;
        }

        Real2D {x: 400.0*x, y: 400.0*y}
    }

    fn cohesion (agent: &AgentAdapter, vec: &Vec<AgentAdapter>) -> Real2D {
        if vec.is_empty() {
            return Real2D {x: 0.0, y: 0.0};
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            if *agent != vec[i] {
                let dx = toroidal_distance(agent.pos.x, vec[i].pos.x, WIDTH.into());
                let dy = toroidal_distance(agent.pos.y, vec[i].pos.y, HEIGHT.into());
                count += 1;
                x += dx;
                y += dy;
            }
        }
        if count > 0 {
            x = x/count as f64;
            y = y/count as f64;
        }

        Real2D {x: -x/10.0, y: -y/10.0}
    }

    fn randomness() -> Real2D {
        let mut rng = rand::thread_rng();
        let r1: f64 = rng.gen();
        let x = r1*2.0 -1.0;
        let r2: f64 = rng.gen();
        let y = r2*2.0 -1.0;

        let square = (x*x + y*y).sqrt();
        
        Real2D {
            x: 0.05*x/square,
            y: 0.05*y/square,
        }
    }

    fn consistency (agent: &AgentAdapter, vec: &Vec<AgentAdapter>) -> Real2D {
        if vec.is_empty() {
            return Real2D {x: 0.0, y: 0.0};
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for i in 0..vec.len() {
            if *agent != vec[i] {

                let xx = vec[i].last_d.x;
                let yy = vec[i].last_d.y;
                count += 1;
                x += xx;
                y += yy;
            }
        }
        if count > 0 {
            x = x/count as f64;
            y = y/count as f64;
        }
        
        Real2D {x, y}
    }
}

// Transform our struct in an actual Amethyst System.
impl<'s> System<'s> for FlockerSystem {
    // Specify what data we are going to operate on and in which way. Amethyst will give it to us and it will build
    // an optimized execution schedule to parallelize systems as much as possible.
	type SystemData = (
		WriteStorage<'s, Transform>,
        WriteStorage<'s, AgentAdapter>,
        WriteExpect<'s, Field2D<AgentAdapter>>,
	);

	fn run(&mut self, (mut transforms, mut agent_adapters, mut field): Self::SystemData) {
        // We specify which groups of components we're gonna operate on, by fetching them from their respective
        // storages, similar to a SQL query. The components returned per cycle are owned by the same entity.
		for(agent_adapter, transform) in (&mut agent_adapters, &mut transforms).join() {
            let vec = field.get_neighbors_within_distance(agent_adapter.pos, NEIGHBOR_DISTANCE);
            let avoidance = FlockerSystem::avoidance(agent_adapter, &vec);
            let cohesion = FlockerSystem::cohesion(agent_adapter, &vec);
            let randomness = FlockerSystem::randomness();
            let consistency = FlockerSystem::consistency(agent_adapter, &vec);
            let momentum = agent_adapter.last_d;

            let mut dx = COHESION*cohesion.x + AVOIDANCE*avoidance.x + CONSISTENCY*consistency.x + RANDOMNESS*randomness.x + MOMENTUM*momentum.x;
            let mut dy = COHESION*cohesion.y + AVOIDANCE*avoidance.y + CONSISTENCY*consistency.y + RANDOMNESS*randomness.y + MOMENTUM*momentum.y;

            let dis = (dx*dx + dy*dy).sqrt();
            if dis > 0.0 {
                dx = dx/dis*JUMP;
                dy = dy/dis*JUMP;
            }

            let _last_pos = Real2D {x: dx, y: dy};
            let loc_x = toroidal_transform(agent_adapter.pos.x + dx, WIDTH.into());
            let loc_y = toroidal_transform(agent_adapter.pos.y + dx, HEIGHT.into());
            agent_adapter.last_d = _last_pos;
            let new_x = loc_x + dx;
            let new_y = loc_y + dy;
            let old_x: f64 = agent_adapter.pos.x;
            let old_y: f64 = agent_adapter.pos.y;
            
            agent_adapter.pos = Real2D { x: new_x, y: new_y};
            field.set_object_location(*agent_adapter, agent_adapter.pos);
            let diff_y = new_y - old_y;
            let diff_x = new_x - old_x;
            let rotation = if diff_y.sin() == 0.0 && diff_x.cos() == 0.0 {
                0.
            } else {
                diff_y.atan2(diff_x)
            };
            let rotation = (rotation - (PI * 0.50)) as f32;
            //let cur_rot = transform.rotation().angle();

            // Offset the result of atan2 to properly align the rotation to the sprite initial direction (up)
            transform.set_rotation_2d(rotation);
            //transform.rotate_2d(rotation-cur_rot);
            
            // Actually set the new transform translation so that the sprite will render in the new position.
            transform.set_translation_xyz(new_x as f32, new_y as f32, 0.0);
        }
	}

}