// Drawn polygons do not work in wasm due to the bevy_canvas dependency not having a WebGL shaders.
#[cfg(not(feature = "visualization_wasm"))]
pub mod arrow;
pub mod fixed_timestep;
pub mod updated_time;