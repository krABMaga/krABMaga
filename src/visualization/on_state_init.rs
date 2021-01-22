use crate::visualization::sprite_render_factory::SpriteRenderFactory;
use amethyst::prelude::World;

/// A simple trait which lets the developer access the on_start Amethyst hook. This allows the developer
/// to instantiate entities for his visualization, similar to a constructor.
pub trait OnStateInit {
    fn on_init(&self, world: &mut World, sprite_render_factory: &mut SpriteRenderFactory);
}
