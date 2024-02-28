mod id;
mod layers;
mod lib;

use ::bevy::prelude::*;

use bevy_pancam::*;

use bevy::render::camera::ScalingMode;
use bevy_egui::{
    egui::{self, Color32, RichText},
    EguiContexts,
};

use self::lib::EntityFile;

pub struct GisPlugin;

impl Plugin for GisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pick_file);
    }
}

//fn startup(mut commands: Commands) {
//    let far = 1000.;
//    // Offset the whole simulation to the left to take the width of the UI panel into account.
//    let ui_offset = -700.;
//    // Scale the simulation so it fills the portion of the screen not covered by the UI panel.
//    let scale_x = 700. / (700. + ui_offset);
//    // The translation x must depend on the scale_x to keep the left offset constant between window resizes.
//    let mut initial_transform = Transform::from_xyz(ui_offset * scale_x, 0., far - 0.1);
//    initial_transform.scale.x = scale_x;
//    initial_transform.scale.y = 700. / 700.;
//
//    commands
//        .spawn(Camera2dBundle {
//            projection: OrthographicProjection {
//                far,
//                scaling_mode: ScalingMode::WindowSize(1.),
//                viewport_origin: Vec2::new(0., 0.),
//                ..default()
//            }
//            .into(),
//            transform: initial_transform,
//            ..default()
//        })
//        .insert(PanCam::default());
//}

fn pick_file(
    mut egui_context: EguiContexts,
    mut commands: Commands,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut files_query: Query<&EntityFile>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    file_entity_query: Query<Entity, With<EntityFile>>,
    all_entities_query: Query<Entity, Without<Camera>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    if let Some(camera) = camera_query.get_single().ok() {
        egui::SidePanel::left("new")
            // .resizable(true)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("GIS").color(Color32::RED).strong());
                    ui.separator();
                    let select_btn =
                        egui::Button::new(RichText::new("▶ Select File").color(Color32::GREEN));
                    let clear_btn =
                        egui::Button::new(RichText::new("▶ Clear").color(Color32::YELLOW));
                    let exit_btn = egui::Button::new(RichText::new("▶ Exit").color(Color32::RED));

                    if ui.add(select_btn).clicked() {
                        if let Some(path_buf) = rfd::FileDialog::new().pick_file() {
                            let extension = path_buf.extension().unwrap();
                            if extension.eq("json") || extension.eq("geojson") {
                                let path = Some(path_buf.display().to_string()).unwrap();
                                let name = path_buf.file_name().unwrap().to_str().unwrap();
                                let (layers, entities) = lib::build_meshes(
                                    &mut *meshes,
                                    &mut *materials,
                                    &mut commands,
                                    path.to_owned(),
                                    name.to_owned(),
                                );
                                let entity_file = EntityFile {
                                    name: name.to_owned(),
                                    path: path.to_owned(),
                                    layers: layers,
                                    entities: entities,
                                };
                                let mut vec_entity_file: Vec<EntityFile> = Vec::new();

                                vec_entity_file.push(entity_file.clone());

                                commands.spawn(entity_file);

                                for file in files_query.iter() {
                                    vec_entity_file.push(file.clone());
                                }

                                lib::center_camera(&mut commands, camera, vec_entity_file);
                            }
                        }
                    }

                    if ui.add(clear_btn).clicked() {
                        for entity in all_entities_query.iter() {
                            commands.entity(entity).despawn();
                        }
                    }

                    if ui.add(exit_btn).clicked() {
                        app_exit_events.send(bevy::app::AppExit);
                    }

                    ui.separator();

                    for file in &mut files_query.iter_mut() {
                        let name = &file.name;
                        let remove_file_btn =
                            egui::Button::new(RichText::new("Remove").color(Color32::WHITE));
                        let label_text = name.to_owned();

                        ui.label(
                            RichText::new(label_text)
                                .strong()
                                .color(Color32::DEBUG_COLOR),
                        );

                        if ui.add(remove_file_btn).clicked() {
                            for entity_file in file_entity_query.iter() {
                                commands.entity(entity_file).despawn();
                            }

                            for entity in file.entities.iter() {
                                commands.entity(*entity).despawn();
                            }
                        }

                        ui.separator();
                    }
                })
            });
    }
}
