pub mod engine;
pub mod utils;
pub use rand; // Re-export rand to let users use the correct version, compatible with wasm

#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub mod visualization;

#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub use bevy;
#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub use bevy_canvas;

///To do "Space exploration" each sim parameters need a triple: start, end and step
pub enum Parameters {
    IntParam(i32, i32, i32),
    FloatParam(f32, f32, f32),
}

#[macro_export]
///step = simulation step number
///schedule
///agents
///states
///other parametes
macro_rules!  simulate{
    ($step:expr, $sch:expr, $ty:ty, $s:expr $(,$opt:expr)*) => {

    let n_step:u128 = $step;
    let mut schedule:Schedule<$ty> = $sch;
    

    $(
        println!("Option received. {}", $opt);
    )*

    let mut fetch_time = std::time::Duration::from_secs_f64(0.);
    let mut step_time = std::time::Duration::from_secs_f64(0.);
    let mut update_time = std::time::Duration::from_secs_f64(0.);
    let start = std::time::Instant::now();
    for _ in 0..n_step{
        let (p_fetch,p_step,p_update) = schedule.step(&mut $s);
        fetch_time += p_fetch;
        step_time += p_step;
        update_time += p_update;
        $s.step +=1;
    }

    let run_duration = start.elapsed();

    println!("Thread_Num\tTotal_Time\tFetch_Time\tStep_Time\tUpdate_Time\tStep_Number\tStep/Seconds");
    println!("{}\t\t{:?}\t{:?}\t{:?}\t{:?}\t{}\t{}",
    schedule.thread_num,
    run_duration,
    fetch_time/schedule.step as u32,
    step_time/schedule.step as u32,
    update_time/schedule.step as u32,
    schedule.step,
    schedule.step as f64 /(run_duration.as_secs_f64())

        );
    };
}

///WORK IN PROGRESS, DONT USE IT
#[macro_export]
///step = simulation step number
///schedule
///agents
///states
///sim param
macro_rules! explore {
    ($step:expr, $sch:expr, $ty:ty, $s:expr, $sim_param:expr) => {
        let n_step: u128 = $step;
        let mut schedule: Schedule<$ty> = $sch;

        let (start, end, range) = match $sim_param {
            Parameters::FloatParam(a, b, c) => (a, b, c),
            Parameters::IntParam(a, b, c) => (a as f32, b as f32, c as f32),
        };

        println!(
            "received param {}: {} {} {}",
            stringify!($sim_param),
            start,
            end,
            range
        );
    };
}
