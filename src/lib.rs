pub mod agent;
pub mod agentimpl;
pub mod location;
pub mod priority;
pub mod simple_grid_2d;
pub mod field;
pub mod state;
use cfg_if::cfg_if;

cfg_if!{
    if #[cfg(feature ="parallel")]{
        mod field_2d_double_buffer_dashmap;
        mod par_schedule;
        pub use field_2d_double_buffer_dashmap::Field2D;
        pub use field_2d_double_buffer_dashmap::toroidal_distance;
        pub use field_2d_double_buffer_dashmap::toroidal_transform;
        pub use par_schedule::Schedule;
    }
    else{
        mod field_2d;
        mod schedule;
        pub use field_2d::Field2D;
        pub use field_2d::toroidal_distance;
        pub use field_2d::toroidal_transform;
        pub use schedule::Schedule;
    }
}