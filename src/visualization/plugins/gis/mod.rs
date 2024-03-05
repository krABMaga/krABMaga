mod lib;

use ::bevy::prelude::*;

//use bevy_pancam::*;

//use bevy::render::camera::ScalingMode;
use bevy_egui::{
    egui::{self, Color32, RichText},
    EguiContexts,
};
use bevy_pancam::PanCamPlugin;

use self::lib::{EntityFile, PickedFile};

pub struct GisPlugin;

impl Plugin for GisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PickedFile { picked: false });
        app.add_plugins(PanCamPlugin);
        app.add_systems(Update, pick_file);
    }
}

fn pick_file(
    mut egui_context: EguiContexts,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut picked: ResMut<PickedFile>,
    camera_query: Query<Entity, With<Camera>>,
    files_query: Query<&EntityFile>,
) {
    if let Some(camera) = camera_query.get_single().ok() {
        if picked.picked.eq(&false) {
            egui::Window::new("GIS")
                .open(&mut true)
                .movable(false)
                .show(egui_context.ctx_mut(), |ui| {
                    let select_btn =
                        egui::Button::new(RichText::new("▶ Select File").color(Color32::GREEN));

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
                                    layers,
                                    entities,
                                };
                                let mut vec_entity_file: Vec<EntityFile> = Vec::new();

                                vec_entity_file.push(entity_file.clone());

                                commands.spawn(entity_file);

                                for file in files_query.iter() {
                                    vec_entity_file.push(file.clone());
                                }

                                lib::center_camera(&mut commands, camera, vec_entity_file.clone());
                            }
                        }
                        picked.picked = true;
                    }
                });
        }
    }
}
