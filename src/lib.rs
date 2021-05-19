pub mod engine;
pub mod utils;
pub use rand; // Re-export rand to let users use the correct version, compatible with wasm

#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub mod visualization;

#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub use bevy;




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
    println!("Num of steps {}", n_step);

    $(
        println!("Option received. {}", $opt);
    )*


    let start = std::time::Instant::now();
    for _ in 0..n_step{
        schedule.step(&mut $s);
        $s.step +=1;
    }

    let run_duration = start.elapsed();

    println!("Time elapsed in testing schedule is: {:?}", run_duration);
    println!("({:?}) Total Step:{}\nStep for seconds: {:?}",
    stringify!($ty),
    schedule.step,
    schedule.step as f64 /(run_duration.as_nanos() as f64 * 1e-9)

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
        let n_step:u128 = $step;
        let mut schedule:Schedule<$ty> = $sch;
            
        let (start, end, range) = match $sim_param{
            Parameters::FloatParam(a, b, c) => {(a, b, c)},
            Parameters::IntParam(a, b, c) => { (a as f32, b as f32, c as f32) }
           
        };

        println!("received param {}: {} {} {}", stringify!($sim_param), start, end, range);
    };
}