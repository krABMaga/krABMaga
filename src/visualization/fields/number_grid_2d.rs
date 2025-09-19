use std::default::Default;
use std::marker::PhantomData;

use bevy::prelude::{
    Assets, Commands, Component, Handle, Image, Query, Res, ResMut, SpriteBundle, Transform,
};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use image::imageops::{flip_horizontal, rotate180};
use image::ImageBuffer;

use crate::engine::{location::Int2D, state::State};
use crate::visualization::{
    asset_handle_factory::AssetHandleFactoryResource, simulation_descriptor::SimulationDescriptor,
    wrappers::ActiveState,
};

// Allows rendering field structs as a single texture, to improve performance by sending the whole struct to the GPU in a single batch.
// Use the trait by declaring a wrapper struct over a field, for example over a NumberGrid2D<f32>, and implementing this trait on said wrapper.
pub trait BatchRender<S: State> {
    // Specifies the conversion from a 2d point in space in a pixel is done. The format of the return value
    // is [Rgba8UnormSrgb]
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4];

    // Specifies how big the texture should be. For example, for a grid the dimensions would be its width and height.
    fn get_dimensions(&self) -> (u32, u32);

    // Useful to specify the z-index where the texture will be drawn. A good default is 0.
    fn get_layer(&self) -> f32;

    // Converts self to a texture.
    fn texture(&self) -> Image {
        let (width, height) = self.get_dimensions();
        let image_buffer = ImageBuffer::from_fn(width, height, |x, y| {
            let loc = Int2D {
                x: x as i32,
                y: y as i32,
            };
            let pixel = self.get_pixel(&loc);
            image::Rgba(pixel)
        });

        // For some reasons, the image buffer created previously is mirrored, so we fix it up.
        let image_buffer = flip_horizontal(&image_buffer);
        let image_buffer = rotate180(&image_buffer);

        Image::new(
            Extent3d {
                width,
                height,
                ..Default::default()
            },
            TextureDimension::D2,
            image_buffer.into_raw(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        )
    }

    // Handles telling bevy how to draw the texture.
    fn render(
        &self,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        commands: &mut Commands,
        sim: &mut SimulationDescriptor,
    ) where
        Self: 'static + Sized + Sync + Send,
    {
        let texture = self.texture();
        let handle = sprite_render_factory.assets.add(texture);
        let transform = Transform::from_xyz(sim.center_x - 0.5, sim.center_y, self.get_layer());
        let sprite_bundle = SpriteBundle {
            texture: handle,
            transform,
            ..Default::default()
        };
        commands.spawn(sprite_bundle).insert(Marker::<Self> {
            marker: PhantomData,
        });
    }

    // Must override to specify how to fetch the texture of self from the state. Your state struct
    // should have self as one of its field, just return the result of texture() applied on it.
    fn get_texture_from_state(state: &S) -> Image;

    // The system that will handle batch rendering self. You must insert this system in the [AppBuilder].
    fn batch_render(
        mut assets: ResMut<Assets<Image>>,
        mut query: Query<(&Marker<Self>, &mut Handle<Image>)>,
        state_wrapper: Res<ActiveState<S>>,
    ) where
        Self: 'static + Sized + Sync + Send,
    {
        let (_marker, mut image) = query.single_mut();
        let new_image =
            Self::get_texture_from_state(&(*state_wrapper).0.lock().expect("error on lock"));

        let new_asset = assets.add(new_image);
        *image = new_asset;
    }
}

// Marker required to mark the batch render entity, to be able to query for it.
#[derive(Component)]
pub struct Marker<T> {
    marker: PhantomData<T>,
}
