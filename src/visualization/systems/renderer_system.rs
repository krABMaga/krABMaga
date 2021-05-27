use bevy::prelude::{ColorMaterial, Handle, Query, Res, Transform, Visible};

use crate::bevy::prelude::Commands;
use crate::engine::schedule::Schedule;
use crate::visualization::renderable::{Render, SpriteType};
use crate::visualization::sprite_render_factory::SpriteFactoryResource;

/// The system that updates the visual representation of each agent of our simulation.
pub fn renderer_system<A: Render + Clone>(
    mut query: Query<(
        &mut A,
        &mut Transform,
        &mut Visible,
        &mut Handle<ColorMaterial>,
    )>,
    state: Res<A::SimState>,
    schedule: Res<Schedule<A>>,
    mut sprite_factory: SpriteFactoryResource,
    mut commands: Commands,
) {
    for (mut render, mut transform, mut visible, mut material) in query.iter_mut() {
        render.update(&mut *transform, &state, &mut *visible);
        let SpriteType::Emoji(emoji_code) = render.sprite();
        let new_material = sprite_factory.get_material_handle(emoji_code);
        if *material != new_material {
            *material = new_material;
        }
    }
    for new_agent in &schedule.newly_scheduled {
        let SpriteType::Emoji(emoji_code) = new_agent.sprite();
        let sprite_render = sprite_factory.get_emoji_loader(emoji_code);
        new_agent
            .clone()
            .setup_graphics(sprite_render, &mut commands, &state);
    }
}
