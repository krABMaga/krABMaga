use std::marker::PhantomData;

use crate::engine::{location::Int2D, state::State};
use crate::visualization::{
    asset_handle_factory::AssetHandleFactoryResource, simulation_descriptor::SimulationDescriptor,
    wrappers::ActiveState,
};

use bevy::prelude::{
    Assets, ColorMaterial, Commands, Handle, Query, Res, ResMut, SpriteBundle, Texture, Transform,
};
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};

use image::imageops::{flip_horizontal, rotate180};
use image::ImageBuffer;

// Allows rendering field structs as a single texture, to improve performance by sending the whole struct to the GPU in a single batch.
// Use the trait by declaring a wrapper struct over a field, for example over a NumberGrid2D<f32>, and implementing this trait on said wrapper.
pub trait BatchRender<S: State> {
    // Specifies the conversion from a 2d point in space in a pixel is done. The format of the return value
    // is [Rgba8UnormSrgb]
    fn get_pixel(&self, pos: &Int2D) -> [u8; 4];

    // Specifies how big the texture should be. For example, for a grid the dimensions would be its width and height.
    fn get_dimensions(&self) -> (u32, u32);

    // Useful to specify the z-index where the texture will be drawn. A good default is 0.
    fn get_layer(&self) -> f32;

    // Converts self to a texture.
    fn texture(&self) -> Texture {
        let (width, height) = self.get_dimensions();
        let image_buffer = ImageBuffer::from_fn(width, height, |x, y| {
            let pos = Int2D {
                x: x as i32,
                y: y as i32,
            };
            let pixel = self.get_pixel(&pos);
            image::Rgba(pixel)
        });

        // For some reasons, the image buffer created previously is mirrored, so we fix it up.
        let image_buffer = flip_horizontal(&image_buffer);
        let image_buffer = rotate180(&image_buffer);

        Texture::new(
            Extent3d::new(width, height, 1),
            TextureDimension::D2,
            image_buffer.into_raw(),
            TextureFormat::Rgba8UnormSrgb,
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
        let transform = Transform::from_xyz(sim.center_x, sim.center_y, self.get_layer());
        let sprite_bundle = SpriteBundle {
            material: sprite_render_factory.materials.add(handle.into()),
            transform,
            ..Default::default()
        };
        commands
            .spawn()
            .insert(Marker::<Self> {
                marker: PhantomData,
            })
            .insert_bundle(sprite_bundle);
    }

    // Must override to specify how to fetch the texture of self from the state. Your state struct
    // should have self as one of its field, just return the result of texture() applied on it.
    fn get_texture_from_state(state: &S) -> Texture;

    // The system that will handle batch rendering self. You must insert this system in the [AppBuilder].
    fn batch_render(
        mut assets: ResMut<Assets<Texture>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut query: Query<(&Marker<Self>, &mut Handle<ColorMaterial>)>,
        state_wrapper: Res<ActiveState<S>>,
    ) where
        Self: 'static + Sized + Sync + Send,
    {
        let material = query.single_mut();
        if let Ok(query_result) = material {
            let material = &*query_result.1;
            let new_texture = Self::get_texture_from_state(&(*state_wrapper).0.lock().unwrap());

            let color_material = materials.get_mut(material).unwrap();
            let old_texture_handle = color_material.texture.as_ref().unwrap();
            let new_asset = assets.set(old_texture_handle, new_texture);
            color_material.texture = Some(new_asset);
        }
    }
}

// Marker required to mark the batch render entity, to be able to query for it.
pub struct Marker<T> {
    marker: PhantomData<T>,
}
