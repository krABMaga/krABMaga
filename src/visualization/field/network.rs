use std::fmt::Display;
use std::hash::Hash;

use bevy::prelude::Color;

use crate::bevy::prelude::{Res, ResMut, Vec2};
use crate::engine::agent::Agent;
use crate::engine::field::network::{Edge, Network};
use crate::engine::location::Real2D;
use crate::visualization::renderable::Render;
use crate::visualization::utils::arrow::Arrow;
use bevy_canvas::{Canvas, DrawMode};
use std::f32::consts::PI;

/// Allows customization of the arrow geometry used to render edges.
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

/// Specifies the type of geometry to use to represent the edge, along with drawing directives if needed.
pub enum LineType {
    Line,
    /// An arrow will be drawn with the arrow head placed on the second point. You can specify the length
    /// of the vectors representing the arrow head segments, and the angle in radians.
    Arrow(ArrowOptions),
}

/// All the data we need to properly visualize an edge.
pub struct EdgeRenderInfo {
    pub line_color: Color,
    pub draw_mode: DrawMode,
    pub source_loc: Real2D,
    pub target_loc: Real2D,
    pub line_type: LineType,
}

/// Allows rendering the edges of a graph as customizable lines through the Bevy Canvas plugin.
/// As for now (27/05/2021), this does NOT work in a WebGL context due to the bevy_canvas plugin not
/// being compatible with WebGL.
pub trait NetworkRender<
    O: Hash + Eq + Clone + Display,
    L: Clone + Hash + Display,
    A: 'static + Agent + Render + Clone + Send,
>
{
    /// Specify how to fetch a reference to the network from the state.
    fn get_network(state: &A::SimState) -> &Network<O, L>;

    /// Called for each edge to let the user specify how it should be rendered
    fn get_edge_info(edge: &Edge<O, L>) -> EdgeRenderInfo;

    fn render(state: Res<A::SimState>, mut canvas: ResMut<Canvas>) {
        if cfg!(target_arch = "wasm32") {
            panic!("Currently network visualization does not support WebGL shaders. https://github.com/Nilirad/bevy_canvas/blob/main/src/render/mod.rs#L257");
        }

        let network = Self::get_network(&*state);
        for node_edges in network.edges.values() {
            for edge in node_edges {
                let EdgeRenderInfo {
                    line_color,
                    draw_mode,
                    source_loc,
                    target_loc,
                    line_type,
                } = Self::get_edge_info(edge);
                // We could just use the correct geometry based on whether the network is directed or not,
                // but that way we couldn't allow the user to configure the arrow's options.
                match line_type {
                    LineType::Line => {
                        let line = bevy_canvas::common_shapes::Line(
                            Vec2::new(source_loc.x as f32, source_loc.y as f32),
                            Vec2::new(target_loc.x as f32, target_loc.y as f32),
                        );
                        canvas.draw(&line, draw_mode, line_color);
                    }
                    LineType::Arrow(ArrowOptions { length, angle }) => {
                        let arrow = Arrow(
                            Vec2::new(source_loc.x as f32, source_loc.y as f32),
                            Vec2::new(target_loc.x as f32, target_loc.y as f32),
                            length,
                            angle,
                        );
                        canvas.draw(&arrow, draw_mode, line_color);
                    }
                };
            }
        }
    }
}
