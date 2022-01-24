// use std::f32::consts::PI;
// use std::fmt::Display;
// use std::hash::Hash;

// use crate::bevy::prelude::{Res, ResMut, Vec2};

// use crate::visualization::{utils::arrow::Arrow, wrappers::ActiveState};

// use crate::engine::{
//     fields::network::{Edge, Network},
//     location::Real2D,
//     state::State,
// };

// pub use bevy::prelude::Color;
// pub use bevy_canvas::{Canvas, DrawMode};

// pub extern crate bevy_canvas;

// // Allows customization of the arrow geometry used to render edges.
// pub struct ArrowOptions {
//     pub length: f32,
//     pub angle: f32,
// }

// impl Default for ArrowOptions {
//     fn default() -> Self {
//         ArrowOptions {
//             length: 10.,
//             angle: PI / 12.,
//         }
//     }
// }

// // Specifies the type of geometry to use to represent the edge, along with drawing directives if needed.
// pub enum LineType {
//     Line,
//     // An arrow will be drawn with the arrow head placed on the second point. You can specify the length
//     // of the vectors representing the arrow head segments, and the angle in radians.
//     Arrow(ArrowOptions),
// }

// // All the data we need to properly visualize an edge.
// pub struct EdgeRenderInfo {
//     pub line_color: Color,
//     pub draw_mode: DrawMode,
//     pub source_loc: Real2D,
//     pub target_loc: Real2D,
//     pub line_type: LineType,
// }

// /// Allows rendering the edges of a graph as customizable lines through the Bevy Canvas plugin.
// pub trait NetworkRender<O: Hash + Eq + Clone + Display, L: Clone + Hash + Display, S: State> {
//     // Specify how to fetch a reference to the network from the state.
//     fn get_network(state: &S) -> &Network<O, L>;

//     // Called for each edge to let the user specify how it should be rendered
//     fn get_edge_info(edge: &Edge<L>, network: &Network<O, L>) -> EdgeRenderInfo;

//     fn render(state_wrapper: Res<ActiveState<S>>, mut canvas: ResMut<Canvas>) {
//         if cfg!(target_arch = "wasm32") {
//             panic!("Currently network visualization does not support WebGL shaders. https://github.com/Nilirad/bevy_canvas/blob/main/src/render/mod.rs#L257");
//         }

//         let state = state_wrapper.0.lock().unwrap();
//         let network = Self::get_network(&*state);
//         // let network = Self::get_network(&((*state_wrapper.lock().unwrap()).0));
//         for node_edges in network.edges.values() {
//             for edge in node_edges {
//                 let EdgeRenderInfo {
//                     line_color,
//                     draw_mode,
//                     source_loc,
//                     target_loc,
//                     line_type,
//                 } = Self::get_edge_info(edge, network);
//                 // We could just use the correct geometry based on whether the network is directed or not,
//                 // but that way we couldn't allow the user to configure the arrow's options.
//                 match line_type {
//                     LineType::Line => {
//                         let line = bevy_canvas::common_shapes::Line(
//                             Vec2::new(source_loc.x as f32, source_loc.y as f32),
//                             Vec2::new(target_loc.x as f32, target_loc.y as f32),
//                         );
//                         canvas.draw(&line, draw_mode, line_color);
//                     }
//                     LineType::Arrow(ArrowOptions { length, angle }) => {
//                         let arrow = Arrow(
//                             Vec2::new(source_loc.x as f32, source_loc.y as f32),
//                             Vec2::new(target_loc.x as f32, target_loc.y as f32),
//                             length,
//                             angle,
//                         );
//                         canvas.draw(&arrow, draw_mode, line_color);
//                     }
//                 };
//             }
//         }
//     }
// }
