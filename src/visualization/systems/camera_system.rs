use bevy::{
    prelude::{Query, Transform},
    window::WindowResized,
};
use bevy::prelude::EventReader;

use crate::bevy::prelude::Res;
use crate::bevy::render::camera::Camera;
use crate::visualization::simulation_descriptor::SimulationDescriptor;

pub fn camera_system(
    mut resize_event: EventReader<WindowResized>,
    sim: Res<SimulationDescriptor>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for e in resize_event.read() {
        let (_camera, mut transform) = query.single_mut();
        // Offset the whole simulation to the left to take the width of the UI panel into account.
        let ui_offset = -sim.ui_width;
        // Scale the simulation so it fills the portion of the screen not covered by the UI panel.
        let scale_x = sim.width / (e.width + ui_offset);

        // The translation x must depend on the scale_x to keep the left offset constant between window resizes.
        transform.translation.x = ui_offset * scale_x;
        transform.scale.x = scale_x;
        // Scale up the simulation enough to fill the window height
        transform.scale.y = sim.height / e.height;
    }
}
