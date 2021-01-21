use crate::model::bird::Bird;
use crate::model::boids_state::BoidsState;
use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::core::Transform;
use rust_ab::engine::location::Real2D;
use rust_ab::visualization::renderable::{Render, SpriteType};
use std::f64::consts::PI;

impl Render for Bird {
    fn sprite(&self) -> SpriteType {
        /* Example of randomly choosing an emoji sprite for each agent
        let possible_sprites = vec![
            "8ball", "airplane", "anger", "apple", "angry", "banana", "beer",
        ];
        let choice = possible_sprites.choose(&mut rand::thread_rng()).unwrap();
        SpriteType::Emoji(String::from(*choice))*/
        SpriteType::Emoji(String::from("bird"))
    }

    fn position(&self, state: &BoidsState) -> (f32, f32, f32) {
        let loc = state.field1.get_object_location(*self);
        match loc {
            Some(pos) => (pos.x as f32, pos.y as f32, 0.),
            None => (self.pos.x as f32, self.pos.y as f32, 0.),
        }
    }

    fn scale(&self) -> (f32, f32) {
        (0.1, 0.1)
    }

    /// The bird emoji points to left by default, so we calculate the rotation
    /// and offset it by pi radians
    fn rotation(&self) -> f32 {
        let rotation = if self.last_d.x == 0. || self.last_d.y == 0. {
            0.
        } else {
            self.last_d.y.atan2(self.last_d.x)
        };
        (rotation - PI) as f32
    }

    fn update(&mut self, transform: &mut Transform, state: &BoidsState) {
        let (pos_x, pos_y, z) = self.position(state);
        let model_pos = Real2D {
            x: pos_x as f64,
            y: pos_y as f64,
        };

        // Update our local bird copy positions to properly calculate rotation
        let scheduled_bird = state.field1.get_objects_at_location(model_pos);
        let scheduled_bird = scheduled_bird.first();
        if let Some(state_bird) = scheduled_bird {
            self.pos = state_bird.pos;
            self.last_d = state_bird.last_d;
        }

        let rotation = self.rotation();
        transform.set_translation_xyz(pos_x, pos_y, z);
        transform.set_rotation_2d(rotation);
    }
}

impl Component for Bird {
    type Storage = DenseVecStorage<Self>;
}
