use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "visualization", feature = "visualization_wasm"))] {

        // use bevy::prelude::{Commands, OrthographicCameraBundle, Res, ResMut, WindowDescriptor};
        use bevy::prelude::{Commands, Camera2dBundle, Res, ResMut, WindowDescriptor};
        use bevy::render::camera::WindowOrigin;
        use crate::bevy::prelude::Transform;
        // use crate::bevy::render::camera::{DepthCalculation};
        // use bevy::render::camera::CameraPlugin;
        use crate::bevy::utils::default;
        // use crate::bevy::ui::entity::UiCameraConfig;
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
            // on_init: I,
            mut sprite_factory: AssetHandleFactoryResource,
            mut commands: Commands,
            state_resource: ResMut<ActiveState<S>>,
            // state_resource: ActiveState<S>,
            schedule_resource: ResMut<ActiveSchedule>,
            // schedule_resource: ActiveSchedule,
            window: WindowDescriptor,
            // window: WindowDescriptor,
            mut sim: ResMut<SimulationDescriptor>,
            // mut sim: SimulationDescriptor,
        ) {
            // Right handed coordinate system, equal to how it is implemented in [`OrthographicProjection::new_2d()`].
            let far = 1000.;
            // Offset the whole simulation to the left to take the width of the UI panel into account.
            let ui_offset = -sim.ui_width;
            // Scale the simulation so it fills the portion of the screen not covered by the UI panel.
            let scale_x = sim.width / (window.width + ui_offset);
            // The translation x must depend on the scale_x to keep the left offset constant between window resizes.
            let mut initial_transform = Transform::from_xyz(ui_offset * scale_x, 0., far - 0.1);
            initial_transform.scale.x = scale_x;
            initial_transform.scale.y = sim.height / window.height;

            // let camera_bundle = OrthographicCameraBundle::new_2d() {
            //     camera: Camera::default(),
            //     orthographic_projection: OrthographicProjection {
            //         far,
            //         depth_calculation: DepthCalculation::ZDifference,
            //         window_origin: WindowOrigin::BottomLeft, // Main difference with the new_2d constructor: by default, this is Center
            //         ..Default::default()
            //     },
            //     visible_entities: Default::default(),
            //     frustum: Default::default(),
            //     transform: initial_transform,
            //     global_transform: Default::default(),
            // };

            // 0.6 to 0.7
            // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
            // 0.7 to 0.8
            commands.spawn(Camera2dBundle {
                projection: OrthographicProjection {
                    far,
                    scaling_mode: ScalingMode::WindowSize,
                    window_origin: WindowOrigin::BottomLeft,
                    // depth_calculation: DepthCalculation::ZDifference,
            ..default()
                }
                .into(),
                transform: initial_transform,
                ..default()
            });
            // commands.spawn_bundle(Camera2dBundle::default());
                // .insert(UiCameraConfig {
                //     show_ui: false,
                //     ..default()
                // });

            // commands.spawn_bundle(camera_bundle);
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
