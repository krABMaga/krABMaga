use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    math::{Vec2, Vec3},
    prelude::{default, SpatialBundle},
    render::{
        camera::OrthographicProjection,
        color::Color,
        mesh::{Mesh, Meshable},
    },
    sprite::ColorMaterial,
    transform::components::Transform,
};
use bevy_asset::Assets;
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    geometry::*,
};
use geo::{Centroid, CoordsIter, GeometryCollection};
use geo_types::{Geometry, Point};
use geojson::{quick_collection, FeatureCollection, GeoJson};
use std::{fs, str::FromStr};

#[derive(Component, Clone)]
pub struct EntityFile {
    pub name: String,
    pub path: String,
    pub layers: AllLayers,
    pub entities: Vec<bevy::ecs::entity::Entity>,
}

#[derive(crate::bevy::ecs::system::Resource)]
pub struct PickedFile {
    pub picked: bool,
}

pub enum MeshType {
    Point,
    Polygon,
    LineString,
}

#[derive(
    Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, bevy::ecs::component::Component,
)]
pub struct LayerId(i32);

impl Default for LayerId {
    fn default() -> Self {
        Self::new(-1)
    }
}

impl LayerId {
    pub fn new(last: i32) -> Self {
        LayerId(new_id(last))
    }

    pub fn get_id(&self) -> i32 {
        self.0
    }
}

pub fn new_id(last: i32) -> i32 {
    last + 1
}

#[derive(Clone)]
pub struct Layer {
    pub id: LayerId,
    pub file_name: String,
    pub geom_type: geo_types::Geometry,
    pub visible: bool,
}

#[derive(Clone)]
pub struct AllLayers {
    pub layers: Vec<Layer>,
    pub selected_layer_id: i32,
}

impl AllLayers {
    pub fn new() -> AllLayers {
        AllLayers {
            layers: vec![],
            selected_layer_id: 0,
        }
    }

    fn next_layer_id(&self) -> LayerId {
        LayerId::new(self.last_layer_id())
    }

    pub fn add(&mut self, geometry: geo_types::Geometry, file_name: String) {
        let id = self.next_layer_id();
        let layer = Layer {
            id,
            file_name,
            visible: false,
            geom_type: geometry,
        };

        self.layers.push(layer);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Layer> {
        self.layers.iter()
    }

    pub fn last_layer_id(&self) -> i32 {
        if self.layers.len() == 0 {
            return 0;
        }

        self.layers.last().unwrap().id.get_id()
    }
}

pub fn read_geojson(path: String) -> GeoJson {
    let geojson_str = fs::read_to_string(path).unwrap();
    GeoJson::from_str(&geojson_str).unwrap()
}

pub fn get_feature_collection(geojson: GeoJson) -> FeatureCollection {
    let collection: GeometryCollection = quick_collection(&geojson).unwrap();
    FeatureCollection::from(&collection)
}

pub fn calculate_z(layer_index: i32, mesh_type: MeshType) -> f32 {
    layer_index as f32 * 3.
        + match mesh_type {
            MeshType::Point => 1.,
            MeshType::Polygon => 2.,
            MeshType::LineString => 3.,
        }
}

pub fn medium_centroid(centroids: Vec<Point>) -> Point {
    let mut somma_x = 0.0;
    let mut somma_y = 0.0;

    for centroid in centroids.clone() {
        somma_x += centroid.0.x;
        somma_y += centroid.0.y;
    }

    Point::new(
        somma_x / centroids.len() as f64,
        somma_y / centroids.len() as f64,
    )
}

pub fn build_polygon(
    polygon: geo_types::geometry::Polygon,
    id: i32,
) -> (GeometryBuilder, SpatialBundle) {
    let mut coords: Vec<Vec2> = Vec::new();

    for coord in polygon.coords_iter() {
        coords.push(Vec2 {
            x: coord.x as f32,
            y: coord.y as f32,
        });
    }

    let shape = bevy_prototype_lyon::shapes::Polygon {
        points: coords,
        closed: true,
    };
    let z = calculate_z(id, MeshType::Polygon);
    let translation = Vec3 { x: 0., y: 0., z };
    let transform = Transform::from_translation(translation);
    let spatial = SpatialBundle::from_transform(transform);

    (GeometryBuilder::new().add(&shape), spatial)
}

pub fn build_linestring(
    line_string: geo_types::geometry::LineString,
    id: i32,
) -> (GeometryBuilder, SpatialBundle) {
    let mut coords: Vec<Point> = Vec::new();

    for coord in line_string.0 {
        coords.push(Point::new(coord.x as f64, coord.y as f64));
    }

    let start = coords.get(0).unwrap();
    let last = coords.last().unwrap();
    let shape = bevy_prototype_lyon::shapes::Line(
        Vec2 {
            x: start.0.x as f32,
            y: start.0.y as f32,
        },
        Vec2 {
            x: last.0.x as f32,
            y: last.0.y as f32,
        },
    );
    let translation = Vec3 {
        x: 0.,
        y: 0.,
        z: calculate_z(id, MeshType::LineString),
    };
    let transform = Transform::from_translation(translation);
    let spatial = SpatialBundle::from_transform(transform);
    let builder = GeometryBuilder::new().add(&shape);

    (builder, spatial)
}

pub fn center_camera(
    commands: &mut Commands,
    camera: Entity,
    entity_file: Vec<EntityFile>,
    x: f32,
) {
    let mut points: Vec<geo_types::Point<f64>> = Vec::new();
    let mut new_camera = bevy::core_pipeline::core_2d::Camera2dBundle::default();

    commands.entity(camera).despawn();

    for file in entity_file {
        let layers = file.layers;
        for layer in layers.iter() {
            let geom = &layer.geom_type;
            let centroid = geom.centroid().unwrap();

            points.push(centroid);
        }
    }

    let center = medium_centroid(points);
    let projection = OrthographicProjection {
        scaling_mode: bevy::render::camera::ScalingMode::WindowSize(0.1),
        scale: 0.01,
        ..default()
    };

    new_camera.transform =
        Transform::from_xyz((center.0.x as f32 - x) / 1.5, center.0.y as f32, 999.9);
    new_camera.projection = projection;

    commands
        .spawn(new_camera)
        .insert(bevy_pancam::PanCam::default());
}

pub fn build_meshes(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    commands: &mut Commands,
    path: String,
    name: String,
) -> (AllLayers, Vec<Entity>, Vec<geo::Geometry<f64>>, i32, i32) {
    let geojson = read_geojson(path);
    let feature_collection = get_feature_collection(geojson);
    let mut layers: AllLayers = AllLayers::new();
    let mut shapes: Vec<geo::Geometry<f64>> = vec![];
    let mut min_y = f64::INFINITY;
    let mut min_x = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut entities_id: Vec<Entity> = Vec::new();

    for feature in feature_collection.clone().into_iter() {
        let geometry = feature.geometry.unwrap();
        let geom: geo::Geometry = geometry.try_into().unwrap();
        let geo_t: geo_types::Geometry = geom.try_into().unwrap();

        shapes.push(geo_t.clone());

        let (tmp_min_x, tmp_min_y, tmp_max_x, tmp_max_y) = max_min_coords(&geo_t);

        if tmp_min_x < min_x {
            min_x = tmp_min_x;
        }

        if tmp_min_y < min_y {
            min_y = tmp_min_y;
        }

        if tmp_max_x > max_x {
            max_x = tmp_max_x;
        }

        if tmp_max_y > max_y {
            max_y = tmp_min_y;
        }

        match geo_t {
            Geometry::Polygon(polygon) => {
                layers.add(geo::Geometry::Polygon(polygon.clone()), name.clone());

                let (builder, spatial) = build_polygon(polygon, layers.last_layer_id());

                let id = commands
                    .spawn((
                        ShapeBundle {
                            path: builder.build(),
                            spatial,
                            ..default()
                        },
                        Fill::color(Color::WHITE),
                        Stroke::new(Color::BLUE, 0.1),
                    ))
                    .id();

                entities_id.push(id);
            }
            Geometry::LineString(linestring) => {
                layers.add(geo::Geometry::LineString(linestring.clone()), name.clone());

                let (builder, spatial) = build_linestring(linestring, layers.last_layer_id());

                let id = commands
                    .spawn((
                        ShapeBundle {
                            path: builder.build(),
                            spatial,
                            ..default()
                        },
                        Fill::color(Color::RED),
                        Stroke::new(Color::YELLOW_GREEN, 0.1),
                    ))
                    .id();

                entities_id.push(id);
            }
            Geometry::Point(point) => {
                let center = point.centroid();
                layers.add(geo_t.clone(), name.clone());
                let z = calculate_z(layers.last_layer_id(), MeshType::Point);

                let id = commands
                    .spawn(bevy::sprite::MaterialMesh2dBundle {
                        mesh: bevy::sprite::Mesh2dHandle(
                            meshes.add(bevy_math::primitives::Circle::new(1.).mesh()),
                        ),
                        material: materials.add(Color::PINK),
                        transform: Transform::from_translation(Vec3::new(
                            center.0.x as f32,
                            center.0.y as f32,
                            z,
                        )),
                        ..Default::default()
                    })
                    .id();

                entities_id.push(id);
            }
            Geometry::MultiPolygon(multi_polygon) => {
                layers.add(
                    geo::Geometry::MultiPolygon(multi_polygon.clone()),
                    name.clone(),
                );

                for polygon in multi_polygon.0.iter() {
                    let (builder, spatial) = build_polygon(polygon.clone(), layers.last_layer_id());

                    let id = commands
                        .spawn((
                            ShapeBundle {
                                path: builder.build(),
                                spatial,
                                ..default()
                            },
                            Fill::color(Color::WHITE),
                            Stroke::new(Color::BLUE, 0.1),
                        ))
                        .id();
                    entities_id.push(id);
                }
            }
            Geometry::MultiLineString(multi_line_string) => {
                layers.add(
                    geo::Geometry::MultiLineString(multi_line_string.clone()),
                    name.clone(),
                );

                for line in multi_line_string.iter() {
                    let (builder, spatial) = build_linestring(line.clone(), layers.last_layer_id());

                    let id = commands
                        .spawn((
                            ShapeBundle {
                                path: builder.build(),
                                spatial,
                                ..default()
                            },
                            Fill::color(Color::WHITE),
                            Stroke::new(Color::YELLOW_GREEN, 0.1),
                        ))
                        .id();

                    entities_id.push(id);
                }
            }
            Geometry::MultiPoint(multi_point) => {
                for point in multi_point {
                    layers.add(geo::Geometry::Point(point), name.clone());
                    let z = calculate_z(layers.last_layer_id(), MeshType::Point);
                    let id = commands
                        .spawn(bevy::sprite::MaterialMesh2dBundle {
                            mesh: bevy::sprite::Mesh2dHandle(
                                meshes.add(bevy_math::primitives::Circle::new(1.).mesh()),
                            ),
                            material: materials.add(Color::PINK),
                            transform: Transform::from_translation(Vec3::new(
                                point.0.x as f32,
                                point.0.y as f32,
                                z,
                            )),
                            ..Default::default()
                        })
                        .id();

                    entities_id.push(id);
                }
            }
            _ => continue,
        }
    }

    (
        layers,
        entities_id,
        shapes,
        (max_x - min_x) as i32,
        (max_y - min_y) as i32,
    )
}

pub fn max_min_coords(geom: &Geometry) -> (f64, f64, f64, f64) {
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;

    for coord in geom.exterior_coords_iter() {
        if coord.x > max_x {
            max_x = coord.x;
        }
        if coord.x < min_x {
            min_x = coord.x;
        }
        if coord.y > max_y {
            max_y = coord.y;
        }
        if coord.y < min_y {
            min_y = coord.y
        }
    }

    return (min_x, min_y, max_x, max_y);
}
