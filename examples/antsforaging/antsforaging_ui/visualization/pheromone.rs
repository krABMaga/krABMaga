use crate::model::state::State;
use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::core::Transform;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::location::Int2D;
use rust_ab::visualization::renderable::{Render, SpriteType};

/// A graphical representation of a pheromone, contained within a single cell of our grid.
pub struct Pheromone {
    pub loc: Int2D,
}

impl Agent for Pheromone {
    type SimState = State;

    fn step(&mut self, _state: &State) {}
}

impl Render for Pheromone {
    /// The white square will be used as a canvas which will be colored through the Tint component of Amethyst
    fn sprite(&self) -> SpriteType {
        SpriteType::Emoji(String::from("white_square"))
    }

    /// Position never changes, in particular render all pheromones behind ants, nests and obstacles
    fn position(&self, _state: &State) -> (f32, f32, f32) {
        (self.loc.x as f32, self.loc.y as f32, -0.1)
    }

    fn scale(&self) -> (f32, f32) {
        (0.1, 0.1)
    }

    fn rotation(&self) -> f32 {
        0.
    }

    /// Pheromones do not move, we don't need to update anything
    fn update(&mut self, _transform: &mut Transform, _state: &Self::SimState) {}
}

impl Component for Pheromone {
    type Storage = DenseVecStorage<Self>;
}
