mod lib;

use crate::engine::state::State;
use crate::visualization::wrappers::ActiveSchedule;
use crate::visualization::{simulation_descriptor::SimulationDescriptor, wrappers::ActiveState};
use ::bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

use self::lib::{EntityFile, PickedFile};

#[derive(Event)]
pub struct OpenDialog(pub bool);

pub struct GisPlugin<S: State> {
    pub phantom_data: std::marker::PhantomData<S>,
}

impl<S> Plugin for GisPlugin<S>
where
    S: State,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(PickedFile { picked: false });
        app.add_event::<OpenDialog>();
        app.add_plugins(PanCamPlugin);
        app.add_systems(Update, pick_file::<S>);
    }
}

fn pick_file<S: State>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut picked: ResMut<PickedFile>,
    mut event_dialog: EventReader<OpenDialog>,
    state: Res<ActiveState<S>>,
    schedule_resource: ResMut<ActiveSchedule>,
    sim_descriptor: ResMut<SimulationDescriptor>,
    camera_query: Query<Entity, With<Camera>>,
    files_query: Query<&EntityFile>,
) {
    let mut state = state.0.lock().expect("error on lock");
    if let Some(camera) = camera_query.get_single().ok() {
        for event in event_dialog.read().into_iter() {
            if event.0.eq(&true) {
                if let Some(path_buf) = rfd::FileDialog::new().pick_file() {
                    let extension = path_buf.extension().unwrap();
                    if extension.eq("json") || extension.eq("geojson") {
                        let path = Some(path_buf.display().to_string()).unwrap();
                        let name = path_buf.file_name().unwrap().to_str().unwrap();
                        let (layers, entities, shapes, _width, _height) = lib::build_meshes(
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

                        lib::center_camera(&mut commands, camera, vec_entity_file.clone(), x / 2.);

                        let mut r = geo_rasterize::LabelBuilder::background(0)
                            .width(30)
                            .height(30)
                            .build()
                            .unwrap();

                        let mut pixels: Vec<i32> = Vec::new();

                        for shape in shapes {
                            r.rasterize(&shape, 1).unwrap();
                        }

                        for i in r.finish().mapv(|x| x as i32) {
                            pixels.push(i);
                        }
                        (*state).set_gis(
                            pixels,
                            &mut schedule_resource.0.lock().expect("error on lock"),
                        );
                    }
                }
                picked.picked = true;
            }
        }
    }
}
