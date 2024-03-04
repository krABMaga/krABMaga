use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::{Entity, Query, Without};
use bevy::time::{Fixed, Time};
use bevy::window::Window;
use bevy_egui::egui;
use bevy_egui::egui::{Color32, RichText};
use bevy_egui::EguiContexts;

use crate::bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use crate::bevy::prelude::{Commands, Res, ResMut};
use crate::bevy::render::camera::Camera;
use crate::engine::{schedule::Schedule, state::State};
use crate::visualization::{
    asset_handle_factory::AssetHandleFactoryResource,
    simulation_descriptor::SimulationDescriptor,
    visualization_state::VisualizationState,
    wrappers::{ActiveSchedule, ActiveState},
};

pub fn ui_system<I: VisualizationState<S> + Clone + 'static + bevy::prelude::Resource, S: State>(
    mut egui_context: EguiContexts,
    mut sim_data: ResMut<SimulationDescriptor>,
    active_schedule_wrapper: ResMut<ActiveSchedule>,
    active_state_wrapper: ResMut<ActiveState<S>>,
    on_init: Res<I>,
    mut sprite_factory: AssetHandleFactoryResource,
    query: Query<Entity, (Without<Camera>, Without<Window>)>,
    diagnostics: Res<DiagnosticsStore>,
    mut commands: Commands,
    mut time: ResMut<Time<Fixed>>,
) {
    egui::SidePanel::left("main").show(egui_context.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading(sim_data.title.clone());
            ui.separator();
            ui.label("Press start to let the simulation begin!");
            ui.label(format!(
                "Step: {}",
                active_schedule_wrapper
                    .0
                    .lock()
                    .expect("error on lock")
                    .step
            ));
            ui.label(format!("Number of entities: {}", query.iter().count()));

            let fps = match diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
                Some(fps_measurement) => fps_measurement.value,
                None => 0.,
            };
            ui.label(format!("FPS: {:.0}", fps));

            // A slider that allows the user to set the speed of the simulation.
            let mut value = 1. / time.timestep().as_secs_f64();
            ui.add(
                egui::Slider::new(&mut value, 0.1..=250.0)
                    .text("Steps per second")
                    .clamp_to_range(true),
            );
            time.set_timestep_seconds(1. / value);

            ui.horizontal_wrapped(|ui| {
                ui.centered_and_justified(|ui| {
                    let start_button =
                        egui::Button::new(RichText::new("▶ Start").color(Color32::GREEN));
                    if ui.add(start_button).clicked() {
                        sim_data.paused = false;
                    }

                    let stop_button =
                        egui::Button::new(RichText::new("⏹ Stop").color(Color32::RED));
                    if ui.add(stop_button).clicked() {
                        sim_data.paused = true;

                        // Despawn all existing entities (agents)
                        for entity in query.iter() {
                            commands.entity(entity).despawn();
                        }
                        // Reset schedule and state and call the initializer method
                        let mut new_schedule = Schedule::new();
                        active_state_wrapper
                            .0
                            .lock()
                            .expect("error on lock")
                            .reset();
                        active_state_wrapper
                            .0
                            .lock()
                            .expect("error on lock")
                            .init(&mut new_schedule);
                        on_init.on_init(
                            &mut commands,
                            &mut sprite_factory,
                            &mut active_state_wrapper.0.lock().expect("error on lock"),
                            &mut new_schedule,
                            &mut *sim_data,
                        );
                        on_init.setup_graphics(
                            &mut new_schedule,
                            &mut commands,
                            &mut active_state_wrapper.0.lock().expect("error on lock"),
                            sprite_factory,
                        );
                        *(*active_schedule_wrapper).0.lock().expect("error on lock") = new_schedule;
                        //(*active_state_wrapper).0 = new_state;
                    }

                    if ui.button("⏸ Pause").clicked() {
                        sim_data.paused = true;
                    }
                });
            });
        });
    });
}
