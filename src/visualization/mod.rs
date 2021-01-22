cfg_if::cfg_if! {
if #[cfg(any(feature = "amethyst_metal", feature = "amethyst_vulkan"))] {
extern crate amethyst;

pub mod renderable;
pub mod visualization_state;
pub mod visualization;
pub mod on_state_init;
pub mod sprite_render_factory;

mod main_system_bundle;
mod systems;
}
}
