use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::ecs::shred::FetchMut;
use amethyst::prelude::{World, WorldExt};
use amethyst::renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};
use hashbrown::HashMap;

pub struct SpriteRenderFactory {
    emoji_loaders: HashMap<String, SpriteRender>,
    emoji_sprite_sheet: Option<Handle<SpriteSheet>>,
}

impl SpriteRenderFactory {
    pub fn new() -> SpriteRenderFactory {
        SpriteRenderFactory {
            emoji_loaders: HashMap::new(),
            emoji_sprite_sheet: None,
        }
    }

    pub fn get_emoji_loader(&mut self, emoji_code: String, world: &mut World) -> SpriteRender {
        let emoji_filename = format!("{}.png", emoji_code);
        let sprite_render = match self.emoji_loaders.get(&emoji_code) {
            Some(sprite_render) => sprite_render.clone(),
            None => {
                let sprite_render = self.load_emoji_sprite_render(world, emoji_filename);
                self.emoji_loaders.insert(emoji_code, sprite_render.clone());
                sprite_render
            }
        };
        sprite_render
    }

    // TODO: DEADLOCK
    fn load_emoji_sprite_render(
        &mut self,
        world: &mut World,
        emoji_filename: String,
    ) -> SpriteRender {
        println!("Resource not found, loading...");
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

        if self.emoji_sprite_sheet.is_some() {
            SpriteRender::new(self.emoji_sprite_sheet.clone().unwrap(), 0)
        } else {
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            let sprite_sheet = loader.load_from(
                "emoji.ron",
                SpriteSheetFormat(texture_handle),
                "visualization_framework",
                (),
                &sprite_sheet_store,
            );
            self.emoji_sprite_sheet = Some(sprite_sheet);
            SpriteRender::new(self.emoji_sprite_sheet.clone().unwrap(), 0)
        }
    }
}
