use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "visualization", feature = "visualization_wasm"))] {

        use bevy::prelude::{Commands, Camera2dBundle, Res, ResMut};
        use bevy::window::Windows;
        use bevy::render::camera::WindowOrigin;
        use crate::bevy::prelude::Transform;
        use crate::bevy::utils::default;
        use crate::bevy::render::camera::ScalingMode;
        use crate::engine::state::State;
        use crate::bevy::render::camera::OrthographicProjection;

        use crate::visualization::{
            asset_handle_factory::AssetHandleFactoryResource,
            simulation_descriptor::SimulationDescriptor,
            visualization_state::VisualizationState,
            wrappers::{ActiveSchedule, ActiveState},
        };

        /// The main startup system which bootstraps a simple orthographic camera, centers it to aim at the simulation,
        /// then calls the user provided init callback.
        pub fn init_system<I: VisualizationState<S> + 'static + bevy::prelude::Resource, S: State>(
            on_init: Res<I>,
            mut sprite_factory: AssetHandleFactoryResource,
            mut commands: Commands,
            state_resource: ResMut<ActiveState<S>>,
            schedule_resource: ResMut<ActiveSchedule>,
            windows: Res<Windows>,
            mut sim: ResMut<SimulationDescriptor>,
        ) {
            if let Some(window) = windows.get_primary() {

            // Right handed coordinate system, equal to how it is implemented in [`OrthographicProjection::new_2d()`].
            let far = 1000.;
            // Offset the whole simulation to the left to take the width of the UI panel into account.
            let ui_offset = -sim.ui_width;
            // Scale the simulation so it fills the portion of the screen not covered by the UI panel.
            let scale_x = sim.width / (window.width() + ui_offset);
            // The translation x must depend on the scale_x to keep the left offset constant between window resizes.
            let mut initial_transform = Transform::from_xyz(ui_offset * scale_x, 0., far - 0.1);
            initial_transform.scale.x = scale_x;
            initial_transform.scale.y = sim.height / window.height();

            commands.spawn(Camera2dBundle {
                projection: OrthographicProjection {
                    far,
                    scaling_mode: ScalingMode::WindowSize,
                    window_origin: WindowOrigin::BottomLeft,
            ..default()
                }
                .into(),
                transform: initial_transform,
                ..default()
            });

            on_init.on_init(
                &mut commands,
                &mut sprite_factory,
                &mut state_resource.0.lock().expect("error on lock"),
                &mut schedule_resource.0.lock().expect("error on lock"),
                &mut *sim,
            );
            on_init.setup_graphics(
                &mut schedule_resource.0.lock().expect("error on lock"),
                &mut commands,
                &mut state_resource.0.lock().expect("error on lock"),
                sprite_factory,
            )
        }

    }
    }
}
