use amethyst::core::ecs::Component;
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::SpriteRender;

/// A trait that specifies a struct can be rendered somehow.
/// It requires a Storage associated type, one can just use DenseVecStorage<Self> unless futher optimization is required.
pub trait Render: Component + Send + Sync {
    /// Specifies the asset to use when drawing the struct in an Amethyst window.
    /// This should be overwritten to return a string which can point to two things:
    /// 1) An emoji code, a list can be found here: https://www.webfx.com/tools/emoji-cheat-sheet/
    /// 2) A filename pointing to a file within the project's asset folder, which should be located in the project root.
    fn sprite(&self) -> SpriteType;

    /// Specifies the position of the sprite in the window.
    /// This is separate from Location2D because we require f32s, and the user may want to separate window
    /// position from the actual model's.
    fn position(&self) -> (f32, f32);

    /// Specifies the scale of the sprite in the window.
    fn scale(&self) -> (f32, f32);

    /// Rotation of the sprite in radians
    fn rotation(&self) -> f32;

    fn setup_graphics(self, sprite_render: SpriteRender, world: &mut World) {
        let mut transform = Transform::default();
        let (x, y) = self.position();
        let (scale_x, scale_y) = self.scale();
        transform.set_translation_xyz(x, y, 0.);
        transform.set_scale(Vector3::new(scale_x, scale_y, 1.));
        world
            .create_entity()
            .with(sprite_render)
            .with(transform)
            .with(self)
            .build();
    }

    /// Update the graphical position and orientation based on the info coming from the model
    fn update(&mut self, transform: &mut Transform) {
        let (pos_x, pos_y) = self.position();
        println!("Agent: {:?} {:?}", self.position(), transform);
        println!("Setting transform with {:?} {:?}!", pos_x, pos_y);
        transform.set_translation_xyz(pos_x, pos_y, 0.);
    }
}

pub enum SpriteType {
    Emoji(String),
    // File(String), TODO
}
