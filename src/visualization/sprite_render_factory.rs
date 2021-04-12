use std::path::Path;

use bevy::ecs::system::SystemParam;
use bevy::prelude::{
    AssetServer, Assets, ColorMaterial, Handle, Res, ResMut, SpriteBundle, Texture,
};
use hashbrown::HashMap;

/// A simple lazy loader of sprites, mainly for use with the Emoji sprite feature offered by the framework.
/// This allows loading sprites only once, storing a handle pointing to the sprite resource itself and returning clones
/// of the handle, for optimization purposes.
pub struct SpriteRenderFactory {
    emoji_loaders: HashMap<String, Handle<Texture>>,
}

impl SpriteRenderFactory {
    pub fn new() -> SpriteRenderFactory {
        SpriteRenderFactory {
            emoji_loaders: HashMap::new(),
        }
    }

    /// Get the sprite_render associated to the emoji code lazily.
    pub fn get_emoji_loader(
        &mut self,
        emoji_code: String,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> SpriteBundle {
        let emoji_filename = format!("{}.png", emoji_code);
        let sprite_handle = match self.emoji_loaders.get(&emoji_code) {
            Some(handle) => handle.clone(),
            None => {
                let handle = self.load_emoji_sprite(asset_server, emoji_filename);
                self.emoji_loaders.insert(emoji_code, handle.clone());
                handle
            }
        };
        SpriteBundle {
            material: materials.add(sprite_handle.into()),
            ..Default::default()
        }
    }

    /// Actually fetch the sprite resource from the filesystem, from the framework asset folder.
    fn load_emoji_sprite(
        &mut self,
        asset_server: &Res<AssetServer>,
        emoji_filename: String,
    ) -> Handle<Texture> {
        asset_server.load(Path::new("emojis").join(emoji_filename))
    }
}

/// A bundle of resources related to sprite assets, commonly used to edit the graphical representation of an agent.
#[derive(SystemParam)]
pub struct SpriteFactoryResource<'a> {
    pub sprite_factory: ResMut<'a, SpriteRenderFactory>,
    pub asset_server: Res<'a, AssetServer>,
    pub materials: ResMut<'a, Assets<ColorMaterial>>,
    pub assets: ResMut<'a, Assets<Texture>>,
}

impl<'a> SpriteFactoryResource<'a> {
    /// A proxy method that exposes [SpriteRenderFactory get_emoji_loader](SpriteRenderFactory#get_emoji_loader)
    pub fn get_emoji_loader(&mut self, emoji_code: String) -> SpriteBundle {
        self.sprite_factory.get_emoji_loader(
            emoji_code,
            &mut self.materials,
            &mut self.asset_server,
        )
    }
}
