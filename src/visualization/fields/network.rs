pub extern crate bevy_prototype_lyon;

use std::f32::consts::PI;
use std::fmt::Display;
use std::hash::Hash;

pub use bevy::prelude::Color;
use bevy::prelude::{Commands, Component, Query, Transform};
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_prototype_lyon::path::ShapePath;
use bevy_prototype_lyon::prelude::{GeometryBuilder, Path};
use bevy_prototype_lyon::shapes::Line;

use crate::bevy::prelude::{Res, Vec2};
use crate::engine::{
    fields::network::{Edge, Network},
    location::Real2D,
    state::State,
};
use crate::visualization::wrappers::ActiveState;

// Allows customization of the arrow geometry used to render edges.
pub struct ArrowOptions {
    pub length: f32,
    pub angle: f32,
}

impl Default for ArrowOptions {
    fn default() -> Self {
        ArrowOptions {
            length: 10.,
            angle: PI / 12.,
        }
    }
}

// Specifies the type of geometry to use to represent the edge, along with drawing directives if needed.
pub enum LineType {
    Line,
    // An arrow will be drawn with the arrow head placed on the second point. You can specify the length
    // of the vectors representing the arrow head segments, and the angle in radians.
    Arrow(ArrowOptions),
}

// All the data we need to properly visualize an edge.
#[derive(Component)]
pub struct EdgeRenderInfo {
    pub line_color: Color,
    pub line_width: f32,
    pub source_loc: Real2D,
    pub target_loc: Real2D,
    pub is_static: bool, // If true, render() won't loop on this edge
}

#[derive(Component)]
pub struct EdgeRender(u32, u32, Real2D, Real2D);

/// Allows rendering the edges of a graph as customizable lines through the Bevy Canvas plugin.
pub trait NetworkRender<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display, S: State> {
    // Specify how to fetch a reference to the network from the state.
    fn get_network(state: &S) -> &Network<O, L>;

    // Called for each edge to let the user specify how it should be rendered
    fn get_edge_info(edge: &Edge<L>, network: &Network<O, L>) -> EdgeRenderInfo;

    fn get_loc(network: &Network<O, L>, node: u32) -> Real2D;

    fn init_network_graphics(state: &S, commands: &mut Commands) {
        let network = Self::get_network(&*state);
        for node_edges in network.edges.values() {
            for edge in node_edges {
                let EdgeRenderInfo {
                    source_loc,
                    target_loc,
                    line_color,
                    line_width,
                    is_static,
                } = Self::get_edge_info(edge, network);

                let mut spawn_command = commands.spawn((
                    GeometryBuilder::build_as(&Line(
                        Vec2::new(source_loc.x, source_loc.y),
                        Vec2::new(target_loc.x, target_loc.y),
                    )),
                    Fill::color(Color::BLACK),
                    Stroke::new(line_color, line_width),
                    Transform::default(),
                ));
                if !is_static {
                    spawn_command.insert(EdgeRender(edge.u, edge.v, source_loc, target_loc));
                }
            }
        }
    }

    // If the nodes connected by the edge have moved, we regenerate the path mesh related to the edge.
    fn render(state_wrapper: Res<ActiveState<S>>, mut query: Query<(&mut Path, &EdgeRender)>) {
        let state = state_wrapper.0.lock().expect("error on lock");
        let network = Self::get_network(&*state);
        for (mut path, edge_render) in query.iter_mut() {
            let source_loc = Self::get_loc(network, edge_render.0);
            let target_loc = Self::get_loc(network, edge_render.1);
            if source_loc != edge_render.2 || target_loc != edge_render.3 {
                *path = ShapePath::build_as(&Line(
                    Vec2::new(source_loc.x, source_loc.y),
                    Vec2::new(target_loc.x, target_loc.y),
                ));
            }
        }
    }
}
