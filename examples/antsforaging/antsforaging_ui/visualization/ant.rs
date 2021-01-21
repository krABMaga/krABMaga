use crate::model::ant::Ant;
use crate::model::state::State;
use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::core::Transform;
use rust_ab::visualization::renderable::{Render, SpriteType};

impl Render for Ant {
    fn sprite(&self) -> SpriteType {
        SpriteType::Emoji(String::from("ant"))
    }

    /// The position must always be fetched through the state, since that will be the one actually updated
    /// by the RustAB schedule. All objects will be rendered on the 0. z, except pheromones, which will be
    /// put on a lower z-axis.
    fn position(&self, state: &State) -> (f32, f32, f32) {
        let loc = state.get_ant_location(self);
        match loc {
            Some(pos) => (pos.x as f32, pos.y as f32, 0.),
            None => (self.loc.x as f32, self.loc.y as f32, 0.),
        }
    }

    /// Emojis are 64x64, way too big for our simulation
    fn scale(&self) -> (f32, f32) {
        (0.1, 0.1)
    }

    /// No rotation is needed for ants
    fn rotation(&self) -> f32 {
        0.
    }

    /// Simply update the transform based on the position chosen
    fn update(&mut self, transform: &mut Transform, state: &Self::SimState) {
        let (pos_x, pos_y, z) = self.position(state);
        transform.set_translation_xyz(pos_x, pos_y, z);
    }
}

impl Component for Ant {
    type Storage = DenseVecStorage<Self>;
}
