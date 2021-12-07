// Network visualization does not work in wasm due to the bevy_canvas dependency not having a WebGL shaders.
#[cfg(not(feature = "visualization_wasm"))]
pub mod network;
pub mod number_grid_2d;
pub mod object_grid_2d;
