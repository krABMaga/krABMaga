use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::prelude::{World, WorldExt};
use amethyst::renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};
use hashbrown::HashMap;

pub struct SpriteRenderFactory {
    emoji_loaders: HashMap<String, Handle<SpriteSheet>>,
}

impl SpriteRenderFactory {
    pub fn new() -> SpriteRenderFactory {
        SpriteRenderFactory {
            emoji_loaders: HashMap::new(),
        }
    }

    pub fn get_emoji_loader(&mut self, emoji_code: String, world: &mut World) -> SpriteRender {
        let emoji_filename = format!("{}.png", emoji_code);
        let sprite_render = match self.emoji_loaders.get(&emoji_code) {
            Some(sprite_sheet_handle) => SpriteRender::new((*sprite_sheet_handle).clone(), 0),
            None => {
                let sprite_sheet_handle = self.load_emoji_sprite_render(world, emoji_filename);
                let sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 0);
                self.emoji_loaders.insert(emoji_code, sprite_sheet_handle);
                sprite_render
            }
        };
        sprite_render
    }

    fn load_emoji_sprite_render(
        &mut self,
        world: &mut World,
        emoji_filename: String,
    ) -> Handle<SpriteSheet> {
        let loader = world.read_resource::<Loader>();
        let texture_handle = {
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load_from(
                emoji_filename,
                ImageFormat::default(),
                "visualization_framework",
                (),
                &texture_storage,
            )
        };

        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load_from(
            "emoji.ron",
            SpriteSheetFormat(texture_handle),
            "visualization_framework",
            (),
            &sprite_sheet_store,
        )
    }
}
