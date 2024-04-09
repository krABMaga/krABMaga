mod lib;

use ::bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

use crate::visualization::simulation_descriptor::SimulationDescriptor;

use self::lib::{EntityFile, PickedFile};

extern crate tinyfiledialogs as tfd;

#[derive(Event)]
pub struct OpenDialog(pub bool);

pub struct GisPlugin;

impl Plugin for GisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PickedFile { picked: false });
        app.add_event::<OpenDialog>();
        app.add_plugins(PanCamPlugin);
        app.add_systems(Update, pick_file);
    }
}

fn pick_file(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut picked: ResMut<PickedFile>,
    mut event_dialog: EventReader<OpenDialog>,
    sim_descriptor: ResMut<SimulationDescriptor>,
    camera_query: Query<Entity, With<Camera>>,
    files_query: Query<&EntityFile>,
) {
    if let Some(camera) = camera_query.get_single().ok() {
        for event in event_dialog.read().into_iter() {
            if event.0.eq(&true) {
                if let Some(path_buf) = rfd::FileDialog::new().pick_file() {
                    let extension = path_buf.extension().unwrap();
                    if extension.eq("json") || extension.eq("geojson") {
                        let path = Some(path_buf.display().to_string()).unwrap();
                        let name = path_buf.file_name().unwrap().to_str().unwrap();
                        let (layers, entities, shapes, width, height) = lib::build_meshes(
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

                        let x = sim_descriptor.width - sim_descriptor.ui_width;

                        println!("{:?} {:?}", width as usize, height as usize);

                        let mut r = geo_rasterize::LabelBuilder::background(0)
                            .width(width as usize)
                            .height(height as usize)
                            .build()
                            .unwrap();

                        lib::center_camera(&mut commands, camera, vec_entity_file.clone(), x / 2.);

                        for shape in shapes {
                            let _ = r.rasterize(&shape, 1).unwrap();
                        }

                        println!("{:?}", r.finish());
                    }
                }
                picked.picked = true;
            }
        }
    }
}
