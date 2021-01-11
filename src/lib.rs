pub mod agent;
pub mod agentimpl;
pub mod field;
pub mod field_2d;
pub mod location;
pub mod priority;
pub mod simple_grid_2d;
pub mod state;
#[cfg(any(feature = "amethyst_vulkan", feature = "amethyst_metal"))]
pub mod visualization;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature ="parallel")]{
        mod par_schedule;
        pub use par_schedule::Schedule;
    }
    else{
        mod schedule;
        pub use schedule::Schedule;
    }
}
