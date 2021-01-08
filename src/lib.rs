pub mod agent;
pub mod agentimpl;
pub mod bag_ref;
pub mod field;
pub mod field_2d;
pub mod field_2d_double_buffer_dashmap;
pub mod location;
pub mod par_schedule;
pub mod priority;
pub mod schedule;
pub mod simple_grid_2d;
pub mod state;

// TODO: lock over feature
pub mod visualization;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature ="parallel")]{

        pub use field_2d_double_buffer_dashmap::Field2D;
        pub use field_2d_double_buffer_dashmap::toroidal_distance;
        pub use field_2d_double_buffer_dashmap::toroidal_transform;
        pub use par_schedule::Schedule;
    }
    else{

        pub use field_2d::Field2D;
        pub use field_2d::toroidal_distance;
        pub use field_2d::toroidal_transform;
        pub use schedule::Schedule;
    }
}
