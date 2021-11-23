use std::path::Path;

use bevy::ecs::system::SystemParam;

use bevy::prelude::{
    AssetServer, Assets, ColorMaterial, Handle, Res, ResMut, SpriteBundle, Texture,
};

use hashbrown::HashMap;

// A simple lazy loader of sprites, mainly for use with the Emoji sprite feature offered by the framework.
// This allows loading sprites only once, storing a handle pointing to the sprite resource itself and returning clones
// of the handle, for optimization purposes.
pub struct AssetHandleFactory {
    emoji_loaders: HashMap<String, Handle<ColorMaterial>>,
}

impl AssetHandleFactory {
    pub fn new() -> AssetHandleFactory {
        AssetHandleFactory {
            emoji_loaders: HashMap::new(),
        }
    }

    // Get the sprite_render associated to the emoji code lazily.
    pub fn get_emoji_loader(
        &mut self,
        emoji_code: String,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> SpriteBundle {
        SpriteBundle {
            material: self.get_material_handle(emoji_code, materials, asset_server),
            ..Default::default()
        }
    }

    // Actually fetch the sprite resource from the filesystem, from the framework asset folder.
    fn load_emoji_sprite(
        &mut self,
        asset_server: &Res<AssetServer>,
        emoji_filename: String,
    ) -> Handle<Texture> {
        asset_server.load(Path::new("emojis").join(emoji_filename))
    }

    // The core of this factory, stores a reference of the materials handle so that it doesn't get
    // garbage collected and returns its clone.
    pub fn get_material_handle(
        &mut self,
        emoji_code: String,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Handle<ColorMaterial> {
        let emoji_filename = format!("{}.png", emoji_code);
        let material = match self.emoji_loaders.get(&emoji_code) {
            Some(handle) => (*handle).clone(),
            None => {
                let handle = self.load_emoji_sprite(asset_server, emoji_filename);
                let material = materials.add(handle.into());
                self.emoji_loaders.insert(emoji_code, material.clone());
                material
            }
        };
        material
    }
}

// A bundle of resources related to sprite assets, commonly used to edit the graphical representation of an agent.
#[derive(SystemParam)]
pub struct AssetHandleFactoryResource<'a> {
    pub sprite_factory: ResMut<'a, AssetHandleFactory>,
    pub asset_server: Res<'a, AssetServer>,
    pub materials: ResMut<'a, Assets<ColorMaterial>>,
    pub assets: ResMut<'a, Assets<Texture>>,
}

impl<'a> AssetHandleFactoryResource<'a> {
    // A proxy method that exposes [AssetHandleFactory get_emoji_loader](AssetHandleFactory#get_emoji_loader)
    pub fn get_emoji_loader(&mut self, emoji_code: String) -> SpriteBundle {
        self.sprite_factory.get_emoji_loader(
            emoji_code,
            &mut self.materials,
            &mut self.asset_server,
        )
    }

    // A proxy method that exposes [AssetHandleFactory get_material_handle](AssetHandleFactory#get_material_handle)
    pub fn get_material_handle(&mut self, emoji_code: String) -> Handle<ColorMaterial> {
        self.sprite_factory.get_material_handle(
            emoji_code,
            &mut self.materials,
            &mut self.asset_server,
        )
    }
}
