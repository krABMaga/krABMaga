use crate::model::state::State;
use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::core::Transform;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::location::Int2D;
use rust_ab::visualization::renderable::{Render, SpriteType};

/// Objects that do not move, such as obstacles or sites.
pub struct StaticObject {
    pub loc: Int2D,
    pub emoji_code: String,
}

impl Agent for StaticObject {
    type SimState = State;

    fn step(&mut self, _state: &State) {}
}

impl Render for StaticObject {
    /// The emoji_code varies on the object type and it will be passed during the creation of the struct
    fn sprite(&self) -> SpriteType {
        SpriteType::Emoji(self.emoji_code.clone())
    }

    fn position(&self, _state: &State) -> (f32, f32, f32) {
        (self.loc.x as f32, self.loc.y as f32, 0.)
    }

    /// Required because all objects act based on proximity with ants. If they're too big, ants can be
    /// seen to step over such objects without actually avoiding obstacles, or collecting food, because
    /// the simulation objects themself are 1x1.
    fn scale(&self) -> (f32, f32) {
        (0.05, 0.05)
    }

    fn rotation(&self) -> f32 {
        0.
    }

    // No update required, static objects do not move
    fn update(&mut self, _transform: &mut Transform, _state: &Self::SimState) {}
}

impl Component for StaticObject {
    type Storage = DenseVecStorage<Self>;
}
