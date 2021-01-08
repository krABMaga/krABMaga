use crate::model::bird::Bird;
use abm::location::Location2D;
use abm::visualization::renderable::{Render, SpriteType};
use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::core::Transform;
use rand::seq::SliceRandom;

impl Render for Bird {
    fn sprite(&self) -> SpriteType {
        /*
        let possible_sprites = vec![
            "8ball", "airplane", "anger", "apple", "angry", "banana", "beer",
        ];
        let choice = possible_sprites.choose(&mut rand::thread_rng()).unwrap();
        SpriteType::Emoji(String::from(*choice))*/
        SpriteType::Emoji(String::from("airplane"))
    }

    fn position(&self) -> (f32, f32) {
        let loc = self.get_location();
        println!("LOC {:?}", loc);
        (loc.x as f32, loc.y as f32)
    }

    fn scale(&self) -> (f32, f32) {
        (0.1, 0.1)
    }

    fn rotation(&self) -> f32 {
        0.
    }
}

impl Component for Bird {
    type Storage = DenseVecStorage<Self>;
}
