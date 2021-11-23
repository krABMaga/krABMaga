use bevy::prelude::{Commands, OrthographicCameraBundle, Res, ResMut, WindowDescriptor};
use bevy::render::camera::WindowOrigin;

use crate::bevy::prelude::Transform;
use crate::bevy::render::camera::{Camera, DepthCalculation, OrthographicProjection};
use crate::bevy::render::render_graph::base::camera::CAMERA_2D;

use crate::engine::state::State;

use crate::visualization::{
    asset_handle_factory::AssetHandleFactoryResource,
    simulation_descriptor::SimulationDescriptor,
    visualization_state::VisualizationState,
    wrappers::{ActiveSchedule, ActiveState},
};

/// The main startup system which boostraps a simple orthographic camera, centers it to aim at the simulation,
/// then calls the user provided init callback.
pub fn init_system<I: VisualizationState<S> + 'static, S: State>(
    on_init: Res<I>,
    mut sprite_factory: AssetHandleFactoryResource,
    mut commands: Commands,
    state_resource: ResMut<ActiveState<S>>,
    schedule_resource: ResMut<ActiveSchedule>,
    window: Res<WindowDescriptor>,
    mut sim: ResMut<SimulationDescriptor>,
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

    let camera_bundle = OrthographicCameraBundle {
        camera: Camera {
            name: Some(CAMERA_2D.to_string()),
            ..Default::default()
        },
        orthographic_projection: OrthographicProjection {
            far,
            depth_calculation: DepthCalculation::ZDifference,
            window_origin: WindowOrigin::BottomLeft, // Main difference with the new_2d constructor: by default, this is Center
            ..Default::default()
        },
        visible_entities: Default::default(),
        transform: initial_transform,
        global_transform: Default::default(),
    };

    commands.spawn_bundle(camera_bundle);
    on_init.on_init(
        &mut commands,
        &mut sprite_factory,
        &mut state_resource.0.lock().unwrap(),
        &mut schedule_resource.0.lock().unwrap(),
        &mut *sim,
    );
    on_init.setup_graphics(
        &mut schedule_resource.0.lock().unwrap(),
        &mut commands,
        &mut state_resource.0.lock().unwrap(),
        sprite_factory,
    )
}
