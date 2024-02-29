use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    math::{Vec2, Vec3},
    prelude::default,
    render::{
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

use super::layers::AllLayers;

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

pub fn read_geojson(path: String) -> GeoJson {
    let geojson_str = fs::read_to_string(path).unwrap();
    GeoJson::from_str(&geojson_str).unwrap()
}

//read geometry with a feature collection
pub fn read_geojson_feature_collection(geojson: GeoJson) -> FeatureCollection {
    let collection: GeometryCollection = quick_collection(&geojson).unwrap();
    FeatureCollection::from(&collection)
}

pub fn calculate_z(layer_index: i32, mesh_type: MeshType) -> f32 {
    return layer_index as f32 * 3.
        + match mesh_type {
            MeshType::Point => 1.,
            MeshType::Polygon => 2.,
            MeshType::LineString => 3.,
        };
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
) -> (GeometryBuilder, Transform) {
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

    (GeometryBuilder::new().add(&shape), transform)
}

pub fn build_linestring(
    line_string: geo_types::geometry::LineString,
    id: i32,
) -> (GeometryBuilder, Transform) {
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

    let builder = GeometryBuilder::new().add(&shape);
    let translation = Vec3 {
        x: 0.,
        y: 0.,
        z: calculate_z(id, MeshType::LineString),
    };
    let transform = Transform::from_translation(translation);

    (builder, transform)
}

pub fn center_camera(commands: &mut Commands, camera: Entity, entity_file: Vec<EntityFile>) {
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

    new_camera.transform = Transform::from_xyz(center.0.x as f32, center.0.y as f32, 999.9);

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
) -> (AllLayers, Vec<Entity>) {
    let geojson = read_geojson(path);
    let feature_collection = read_geojson_feature_collection(geojson);
    let mut layers: AllLayers = AllLayers::new();
    let mut entities_id: Vec<Entity> = Vec::new();
    for feature in feature_collection {
        let geometry = feature.geometry.unwrap();
        let geom: geo::Geometry = geometry.try_into().unwrap();
        match geom {
            Geometry::Polygon(polygon) => {
                layers.add(geo::Geometry::Polygon(polygon.clone()), name.clone());

                let (builder, transform) = build_polygon(polygon, layers.last_layer_id());

                let id = commands
                    .spawn((
                        ShapeBundle {
                            path: builder.build(),
                            ..default()
                        },
                        Fill::color(Color::WHITE),
                        Stroke::new(Color::BLUE, 0.1),
                        transform,
                    ))
                    .id();

                entities_id.push(id);
            }
            Geometry::LineString(linestring) => {
                layers.add(geo::Geometry::LineString(linestring.clone()), name.clone());

                let (builder, transform) = build_linestring(linestring, layers.last_layer_id());

                let id = commands
                    .spawn((
                        ShapeBundle {
                            path: builder.build(),
                            ..default()
                        },
                        Fill::color(Color::RED),
                        Stroke::new(Color::YELLOW_GREEN, 0.1),
                        transform,
                    ))
                    .id();

                entities_id.push(id);
            }
            Geometry::Point(point) => {
                let center = point.centroid();
                layers.add(geom.clone(), name.clone());
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
                    let (builder, transform) =
                        build_polygon(polygon.clone(), layers.last_layer_id());

                    let id = commands
                        .spawn((
                            ShapeBundle {
                                path: builder.build(),
                                ..default()
                            },
                            Fill::color(Color::WHITE),
                            Stroke::new(Color::BLUE, 0.1),
                            transform,
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
                    let (builder, transform) =
                        build_linestring(line.clone(), layers.last_layer_id());

                    let id = commands
                        .spawn((
                            ShapeBundle {
                                path: builder.build(),
                                ..default()
                            },
                            Fill::color(Color::WHITE),
                            Stroke::new(Color::YELLOW_GREEN, 0.1),
                            transform,
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

    (layers, entities_id)
}
