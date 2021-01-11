use crate::visualization::sprite_render_factory::SpriteRenderFactory;
use amethyst::prelude::World;

pub trait OnStateInit {
    fn on_init(&self, world: &mut World, sprite_render_factory: &mut SpriteRenderFactory);
}
