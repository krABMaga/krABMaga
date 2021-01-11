pub mod agent;
pub mod agentimpl;
pub mod location;
pub mod priority;
pub mod simple_grid_2d;
pub mod field;
pub mod state;
pub mod field_2d;
use cfg_if::cfg_if;


cfg_if!{
    if #[cfg(feature ="parallel")]{
        mod par_schedule;
        pub use par_schedule::Schedule;
    }
    else{
        mod schedule;
        pub use schedule::Schedule;
    }
}
