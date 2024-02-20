use std::hash::Hash;
use std::marker::PhantomData;
use std::marker::Sync;

use bevy::prelude::{Commands, Component, Handle, Image, Query, Res, Transform};

use crate::bevy::math::Quat;
use crate::engine::{
    fields::{dense_object_grid_2d::DenseGrid2D, sparse_object_grid_2d::SparseGrid2D},
    location::Int2D,
    state::State,
};
use crate::visualization::{
    asset_handle_factory::AssetHandleFactoryResource, wrappers::ActiveState,
};

// Allows rendering field structs as a single texture, to improve performance by sending the whole struct to the GPU in a single batch.
// Use the trait by declaring a wrapper struct over a field, for example over a ObjectGrid2D<f32>, and implementing this trait on said wrapper.
pub trait RenderObjectGrid2D<S: State, O: 'static + Sync + Send + Hash + Copy + Eq + Component> {
    // Handles telling bevy how to draw the texture.
    fn init_graphics_grid(
        sprite_render_factory: &mut AssetHandleFactoryResource,
        commands: &mut Commands,
        state_wrapper: &S,
    ) where
        Self: 'static + Sized + Sync + Send,
    {
        let state = &state_wrapper;

        let sparse_grid: Option<&SparseGrid2D<O>> = Self::fetch_sparse_grid(state);
        let dense_grid: Option<&DenseGrid2D<O>> = Self::fetch_dense_grid(state);

        if sparse_grid.is_some() {
            for obj in sparse_grid
                .expect("error on unwrapping sparse grid")
                .obj2loc
                .keys()
            {
                let emoji = Self::fetch_emoji(state, obj);
                let mut sprite_bundle = sprite_render_factory.get_emoji_loader(emoji);
                let loc = Self::fetch_loc(state, obj).expect("error on fetch_loc");
                let rotation = Quat::from_rotation_z(Self::fetch_rotation(state, obj));
                sprite_bundle.transform = Transform::from_xyz(loc.x as f32, loc.y as f32 - 0.5, 0.);
                sprite_bundle.transform.rotation = rotation;
                let scale = Self::scale(obj);
                sprite_bundle.transform.scale.x = scale.0;
                sprite_bundle.transform.scale.y = scale.1;

                commands
                    .spawn(sprite_bundle)
                    .insert(Marker::<Self> {
                        marker: PhantomData,
                    })
                    .insert(obj.clone());
            }
        } else if dense_grid.is_some() {
            for obj in dense_grid
                .expect("error on unwrapping dense_grid")
                .obj2loc
                .keys()
            {
                let emoji = Self::fetch_emoji(state, obj);
                let mut sprite_bundle = sprite_render_factory.get_emoji_loader(emoji);
                let loc = Self::fetch_loc(state, obj).expect("error on fetch_lock");
                let rotation = Quat::from_rotation_z(Self::fetch_rotation(state, obj));
                sprite_bundle.transform = Transform::from_xyz(loc.x as f32, loc.y as f32 - 0.5, 0.);
                // sprite_bundle.transform = Transform::from_xyz(loc.x as f32, 0.5, 0.);
                sprite_bundle.transform.rotation = rotation;
                let scale = Self::scale(obj);
                sprite_bundle.transform.scale.x = scale.0;
                sprite_bundle.transform.scale.y = scale.1;

                commands
                    .spawn(sprite_bundle)
                    .insert(Marker::<Self> {
                        marker: PhantomData,
                    })
                    .insert(obj.clone());
            }
        }
    }

    // fn get_grid(state: &S) -> Option<&SparseGrid2D<O>>;
    fn fetch_sparse_grid(state: &S) -> Option<&SparseGrid2D<O>>;
    fn fetch_dense_grid(state: &S) -> Option<&DenseGrid2D<O>>;
    fn fetch_emoji(state: &S, obj: &O) -> String;
    fn fetch_loc(state: &S, obj: &O) -> Option<Int2D>;
    fn fetch_rotation(state: &S, obj: &O) -> f32;
    fn scale(obj: &O) -> (f32, f32);

    // The system that will handle batch rendering self. You must insert this system in the [AppBuilder].
    fn render(
        mut query: Query<(&Marker<Self>, &O, &mut Transform, &mut Handle<Image>)>,
        mut sprite_render_factory: AssetHandleFactoryResource,
        state_wrapper: Res<ActiveState<S>>,
    ) where
        Self: 'static + Sized + Sync + Send,
    {
        let state = &*state_wrapper.0.lock().expect("error on lock");
        for (_marker, obj, mut transform, mut material) in query.iter_mut() {
            //update location
            let loc = match Self::fetch_loc(state, obj) {
                Some(x) => x,
                None => {
                    //TODO sometimes it panics when pressing stop
                    // panic!("Error the RenderObjectGrid2D must implement fetch_loc.
                    //     Where each object in the Grid2D must have a location!")},
                    continue;
                }
            };

            let new_material =
                sprite_render_factory.get_material_handle(Self::fetch_emoji(state, obj));

            *material = new_material;

            transform.translation.x = loc.x as f32;
            transform.translation.y = loc.y as f32 + 0.5;
            let rotation = Quat::from_rotation_z(Self::fetch_rotation(state, obj));
            transform.rotation = rotation;
        }
    }
}

// Marker required to mark the batch render entity, to be able to query for it.
#[derive(Component)]
pub struct Marker<T> {
    marker: PhantomData<T>,
}
