use crate::engine::agent::Agent;
use amethyst::core::ecs::storage::DistinctStorage;
use amethyst::core::ecs::{Component, EntityBuilder};
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::SpriteRender;

/// A trait that specifies a struct can be rendered somehow.
/// It requires a Storage associated type, one can just use DenseVecStorage<Self> unless futher optimization is required.
///
/// SAFETY:
/// Currently only Storages which implement DistinctStorage are allowed, to allow for the parallelization
/// of the renderer system in future, when a solution will be found for the current problems regarding par_iter() and Transforms.
pub trait Render: Agent + Component + Send + Sync
where
    <Self as Component>::Storage: DistinctStorage,
{
    /// Specifies the asset to use when drawing the struct in an Amethyst window.
    /// This should be overwritten to return a string which can point to two things:
    /// 1) An emoji code, a list of compatible ones can be found here: https://www.webfx.com/tools/emoji-cheat-sheet/
    /// 2) (NOT YET SUPPORTED) A filename pointing to a file within the project's asset folder, which should be located in the project root.
    fn sprite(&self) -> SpriteType;

    /// Specifies the position of the sprite in the window.
    /// This is separate from Location2D because we require f32s, and the user may want to separate window
    /// position from the actual model's.
    ///
    /// IMPORTANT:
    /// Do NOT rely on local fields of the struct implementing Render to save the location of the agent:
    /// they will NOT be automatically updated when the RustAB scheduler steps. Instead, use the state
    /// passed as argument to fetch the agent position and act on that.
    fn position(&self, state: &Self::SimState) -> (f32, f32, f32);

    /// Specifies the scale of the sprite in the window.
    fn scale(&self) -> (f32, f32);

    /// Rotation of the sprite in radians.
    fn rotation(&self) -> f32;

    /// Generate an Amethyst entity and automatically insert it into the world, with a Transform with
    /// the given position, scale and rotation defined by the three previous methods.
    fn setup_graphics(
        self,
        sprite_render: SpriteRender,
        world: &mut World,
        state: &Self::SimState,
    ) {
        self.prepare_graphics_builder(sprite_render, world, state)
            .build();
    }

    /// Prepare an initial EntityBuilder for the entity, made of the absolutely necessary components:
    /// 1) The SpriteRender;
    /// 2) The Transform;
    /// 3) self;
    /// Returns the EntityBuilder for futher component insertions. Call build() on it to finalize the entity.
    fn prepare_graphics_builder<'a>(
        self,
        sprite_render: SpriteRender,
        world: &'a mut World,
        state: &Self::SimState,
    ) -> EntityBuilder<'a> {
        let mut transform = Transform::default();
        let (x, y, z) = self.position(state);
        let (scale_x, scale_y) = self.scale();
        let rotation = self.rotation();
        transform.set_translation_xyz(x, y, z);
        transform.set_scale(Vector3::new(scale_x, scale_y, 1.));
        transform.set_rotation_2d(rotation);
        world
            .create_entity()
            .with(sprite_render)
            .with(transform)
            .with(self)
    }

    /// Update the graphical variables based on the information coming from the model through the state.
    fn update(&mut self, transform: &mut Transform, state: &Self::SimState);
}

pub enum SpriteType {
    Emoji(String),
    // File(String), TODO
}
