use crate::engine::{
    fields::{dense_object_grid_2d::DenseGrid2D, sparse_object_grid_2d::SparseGrid2D},
    location::Int2D,
    state::State,
};

use crate::visualization::{
    // agent_render::SpriteType,
    asset_handle_factory::AssetHandleFactoryResource,
    wrappers::ActiveState,
};

use crate::bevy::math::Quat;
// use bevy::prelude::{
//     Assets, ColorMaterial, Commands, Handle, Query, Res, ResMut, SpriteBundle, Texture, Transform,
// };

use bevy::prelude::{ColorMaterial, Commands, Handle, Query, Res, Transform};

use std::hash::Hash;
use std::marker::PhantomData;
use std::marker::Sync;

// Allows rendering field structs as a single texture, to improve performance by sending the whole struct to the GPU in a single batch.
// Use the trait by declaring a wrapper struct over a field, for example over a ObjectGrid2D<f32>, and implementing this trait on said wrapper.
pub trait RenderObjectGrid2D<S: State, O: 'static + Sync + Send + Hash + Copy + Eq> {
    // Handles telling bevy how to draw the texture.
    fn init_graphics_grid(
        sprite_render_factory: &mut AssetHandleFactoryResource,
        commands: &mut Commands,
        state_wrapper: &S,
    ) where
        Self: 'static + Sized + Sync + Send,
    {
        let state = &state_wrapper;

        let sparse_grid: Option<&SparseGrid2D<O>> = Self::get_sparse_grid(state);
        let dense_grid: Option<&DenseGrid2D<O>> = Self::get_dense_grid(state);

        if sparse_grid.is_some() {
            for obj in sparse_grid.unwrap().obj2loc.keys() {
                let emoji = Self::get_emoji_obj(state, obj);
                let mut sprite_bundle = sprite_render_factory.get_emoji_loader(emoji);
                let pos = Self::get_pos_obj(state, obj).unwrap();
                let rotation = Quat::from_rotation_z(Self::get_rotation_obj(state, obj));
                sprite_bundle.transform = Transform::from_xyz(pos.x as f32, pos.y as f32, 0.);
                sprite_bundle.transform.rotation = rotation;
                let scale = Self::scale(obj);
                sprite_bundle.transform.scale.x = scale.0;
                sprite_bundle.transform.scale.y = scale.1;

                commands
                    .spawn()
                    .insert(Marker::<Self> {
                        marker: PhantomData,
                    })
                    .insert(obj.clone())
                    .insert_bundle(sprite_bundle);
            }
        } else if dense_grid.is_some() {
            for obj in dense_grid.unwrap().obj2loc.keys() {
                let emoji = Self::get_emoji_obj(state, obj);
                let mut sprite_bundle = sprite_render_factory.get_emoji_loader(emoji);
                let pos = Self::get_pos_obj(state, obj).unwrap();
                let rotation = Quat::from_rotation_z(Self::get_rotation_obj(state, obj));
                sprite_bundle.transform = Transform::from_xyz(pos.x as f32, pos.y as f32, 0.);
                sprite_bundle.transform.rotation = rotation;
                let scale = Self::scale(obj);
                sprite_bundle.transform.scale.x = scale.0;
                sprite_bundle.transform.scale.y = scale.1;

                commands
                    .spawn()
                    .insert(Marker::<Self> {
                        marker: PhantomData,
                    })
                    .insert(obj.clone())
                    .insert_bundle(sprite_bundle);
            }
        }
    }

    // fn get_grid(state: &S) -> Option<&SparseGrid2D<O>>;
    fn get_sparse_grid(state: &S) -> Option<&SparseGrid2D<O>>;
    fn get_dense_grid(state: &S) -> Option<&DenseGrid2D<O>>;
    fn get_emoji_obj(state: &S, obj: &O) -> String;
    fn get_pos_obj(state: &S, obj: &O) -> Option<Int2D>;
    fn get_rotation_obj(state: &S, obj: &O) -> f32;
    fn scale(obj: &O) -> (f32, f32);

    // The system that will handle batch rendering self. You must insert this system in the [AppBuilder].
    fn render(
        mut query: Query<(
            &Marker<Self>,
            &O,
            &mut Transform,
            &mut Handle<ColorMaterial>,
        )>,
        mut sprite_render_factory: AssetHandleFactoryResource,
        state_wrapper: Res<ActiveState<S>>,
    ) where
        Self: 'static + Sized + Sync + Send,
    {
        let state = &*state_wrapper.0.lock().unwrap();
        for (_marker, obj, mut transform, mut material) in query.iter_mut() {
            //update position
            let pos = match Self::get_pos_obj(state, obj) {
                Some(x) => x,
                None => {
                    //TODO sometimes it panics when pressing stop
                    // panic!("Error the RenderObjectGrid2D must implement the get_pos_obj.
                    //     Where each object in the Grid2D must have a position!")},
                    continue;
                }
            };

            let new_material =
                sprite_render_factory.get_material_handle(Self::get_emoji_obj(state, obj));

            *material = new_material;

            transform.translation.x = pos.x as f32;
            transform.translation.y = pos.y as f32;
            let rotation = Quat::from_rotation_z(Self::get_rotation_obj(state, obj));
            transform.rotation = rotation;
        }
    }
}

// Marker required to mark the batch render entity, to be able to query for it.
pub struct Marker<T> {
    marker: PhantomData<T>,
}
