use crate::model::bird::Bird;
use crate::model::boids_state::{BoidsState, DISCRETIZATION, HEIGHT, TOROIDAL, WIDTH};
use crate::NUM_AGENT;
use abm::location::Real2D;
use abm::visualization::on_state_init::OnStateInit;
use abm::visualization::renderable::{Render, SpriteType};
use abm::visualization::sprite_render_factory::SpriteRenderFactory;
use abm::Schedule;
use amethyst::assets::Loader;
use amethyst::core::ecs::shred::FetchMut;
use amethyst::prelude::{World, WorldExt};
use amethyst::renderer::SpriteRender;
use rand::Rng;

pub struct VisState;

impl OnStateInit for VisState {
    fn on_init(&self, world: &mut World, sprite_render_factory: &mut SpriteRenderFactory) {
        let state = BoidsState::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL);
        let mut schedule: Schedule<Bird> = Schedule::new();
        let mut rng = rand::thread_rng();
        world.register::<Bird>();
        let mut locked_state = state.field1.lock().unwrap();
        for bird_id in 0..NUM_AGENT {
            // TODO NUM_AGENT
            let r1: f64 = rng.gen();
            let r2: f64 = rng.gen();
            let last_d = Real2D { x: 0., y: 0. };
            let bird = Bird::new(
                bird_id,
                Real2D {
                    x: WIDTH * r1,
                    y: HEIGHT * r2,
                },
                last_d,
            );
            locked_state.set_object_location(bird, bird.pos);

            schedule.schedule_repeating(bird, 0., 0);
            // TODO move all to setup graphics
            let SpriteType::Emoji(emoji_code) = bird.sprite();
            // TODO: get_emoji_loader deadlocks
            let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code, world);
            bird.setup_graphics(sprite_render, world);
        }
        drop(locked_state);
        world.insert(state);
        world.insert(schedule);
    }
}
