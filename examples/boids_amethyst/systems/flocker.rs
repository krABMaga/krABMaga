use std::f64::consts::PI;

use abm::{field_2d::Field2D, field_2d::toroidal_distance, field_2d::toroidal_transform, location::Real2D};
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

    fn process_neighbors (agent: &AgentAdapter, vec: &Vec<AgentAdapter>) -> (Real2D, Real2D, Real2D, Real2D, Real2D) {
        let count = vec.len();
        let [mut avoidance, mut cohesion, mut consistency] = [Real2D{x: 0.,y: 0.}; 3];
                
        for other in vec.iter() {
            if agent != other {
                let dx = toroidal_distance(agent.pos.x, other.pos.x, WIDTH);
                let dy = toroidal_distance(agent.pos.y, other.pos.y, HEIGHT);
                let square = dx*dx + dy*dy;
                avoidance.x += dx/(square*square + 1.0);
                avoidance.y += dy/(square*square + 1.0);
                if !other.dead {
                    cohesion.x += dx;
                    cohesion.y += dy;
                    consistency.x += other.last_d.x;
                    consistency.y += other.last_d.y;
                }
            }
        }
        if count > 0 {
            avoidance.x = 400.0 *avoidance.x/count as f64;
            avoidance.y = 400.0 * avoidance.y/count as f64;
            cohesion.x = -0.1 * cohesion.x/count as f64;
            cohesion.y = -0.1 *cohesion.y/count as f64;
            consistency.x = consistency.x/count as f64;
            consistency.y = consistency.y/count as f64;
        }
        
        (avoidance, cohesion, FlockerSystem::randomness(), consistency, agent.last_d)
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
        for(agent_adapter, transform) in (&mut agent_adapters, &mut transforms).join() {
            if agent_adapter.dead {
                continue;
            }
            let vec = field.get_neighbors_within_distance(agent_adapter.pos, NEIGHBOR_DISTANCE);
            let (avoidance, cohesion, randomness, consistency, momentum) = if vec.is_empty() {
                (Real2D{x: 0.,y: 0.}, Real2D{x: 0.,y: 0.}, FlockerSystem::randomness(), Real2D{x: 0.,y: 0.}, agent_adapter.last_d)
            } else {
                FlockerSystem::process_neighbors(agent_adapter, &vec)
            };

            let mut dx = COHESION*cohesion.x + AVOIDANCE*avoidance.x + CONSISTENCY*consistency.x + RANDOMNESS*randomness.x + MOMENTUM*momentum.x;
            let mut dy = COHESION*cohesion.y + AVOIDANCE*avoidance.y + CONSISTENCY*consistency.y + RANDOMNESS*randomness.y + MOMENTUM*momentum.y;

            let dis = (dx*dx + dy*dy).sqrt();
            if dis > 0.0 {
                dx = dx/dis*JUMP;
                dy = dy/dis*JUMP;
            }

            let new_x = toroidal_transform(agent_adapter.pos.x + dx, WIDTH.into());
            let new_y = toroidal_transform(agent_adapter.pos.y + dy, HEIGHT.into());
            let last_d = Real2D {x: dx, y: dy};
            agent_adapter.last_d = last_d;
            
            agent_adapter.pos = Real2D { x: new_x, y: new_y};
            field.set_object_location(*agent_adapter, agent_adapter.pos);
            let rotation = if last_d.x == 0.0 || last_d.y == 0.0 {
                0.
            } else {
                last_d.y.atan2(last_d.x)
            };
            let rotation = (rotation - (PI * 0.50)) as f32;
            //let cur_rot = transform.rotation().angle();

            // Offset the result of atan2 to properly align the rotation to the sprite initial direction (up)
            transform.set_rotation_2d(rotation);
            //transform.rotate_2d(rotation-cur_rot);
            
            // Actually set the new transform translation so that the sprite will render in the new position.
            transform.set_translation_xyz(new_x as f32, new_y as f32, 0.0);
            /* DEBUG
            if agent_adapter.id == 0 {
                println!("dx: {}, dy: {}\navoidance: {}\ncohesion: {}\nconsistency: {}\nlast_pos: {}\n", dx, dy, avoidance, cohesion, consistency, old_vel);
                std::thread::sleep(std::time::Duration::from_millis(250));
            }*/
        }
    }

}