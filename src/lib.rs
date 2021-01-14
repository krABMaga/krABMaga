pub mod engine;
pub mod utils;

use cfg_if::cfg_if;


cfg_if!{
    if #[cfg(feature ="parallel")]{
        pub use engine::par_schedule::Schedule;
    }
    else{
        pub use engine::schedule::Schedule;
    }
}
