pub mod engine;
pub mod explore;
pub mod utils;

pub use core::fmt;
pub use hashbrown;
pub use indicatif::ProgressBar;
pub use rand;
pub use rand_pcg;
pub use rayon;
#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub mod visualization;

#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub use bevy;

pub use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};

pub use ::lazy_static::*;
pub use csv::{Reader, Writer};
pub use rayon::prelude::*;
use std::error::Error;
pub use std::fs::File;
pub use std::fs::OpenOptions;
pub use std::io::prelude::*;
pub use std::io::Write;
pub use std::process::{Command, Stdio};
pub use std::sync::{Arc, Mutex};
pub use std::time::Duration;

#[cfg(feature = "distributed_mpi")]
pub use {
    memoffset::{offset_of, span_of},
    mpi::datatype::DynBufferMut,
    mpi::datatype::PartitionMut,
    mpi::point_to_point as p2p,
    mpi::Count,
    mpi::{
        datatype::{UncommittedUserDatatype, UserDatatype},
        traits::*,
        Address,
    },
};

#[cfg(feature = "distributed_mpi")]
pub extern crate mpi;

#[cfg(any(feature = "bayesian"))]
pub use {argmin, friedrich, statrs};

#[cfg(feature = "aws")]
pub use {
    aws_config,
    aws_sdk_lambda,
    aws_sdk_sqs,
    futures::executor::block_on,
    lambda_runtime,
    serde_json,
    serde_json::{json, Value},
    std::fs,
    std::io::BufReader,
    tokio,
    tokio::runtime::Runtime, // 0.3.5
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Info {
    Verbose,
    Normal,
}

/**
 * 3 mode to generate the data
 * Exaustive: Brute force parameter exploration
 * Matched: explore every input with the same indexes
 * File: Read from file
 */
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExploreMode {
    Exaustive,
    Matched,
}

#[macro_export]
//step = simulation step number
//states
//# of repetitions
//type of info
macro_rules! simulate {
    ($step:expr, $s:expr, $reps:expr, $info:expr) => {{
        let mut s = $s;
        let mut state = s.as_state_mut();
        let n_step: u64 = $step;

        let mut results: Vec<(Duration, f32)> = Vec::new();
        let option = $info;

        match option {
            Info::Verbose => {
                // println!("\u{1F980} Rust-AB v1.0\n");
                // println!(
                //     "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                //     "#Rep", "Steps", "Steps/Seconds", "Time"
                // );
                // println!("--------------------------------------------------");
            }
            Info::Normal => {
                println!("{esc}c", esc = 27 as char);
                println!("\u{1F980} Rust-AB v1.0\n");
                println!(
                    "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                    "#Rep", "Steps", "Avg. Steps/Seconds", "Avg. Time"
                );
                println!("----------------------------------------------------------------");
            }
        }
        // print!("{:width$}|", 1, width = 14 - $reps.to_string().len());
        // println!(
        //     "{:width$}|",
        //     n_step,
        //     width = 15 - n_step.to_string().len() - $reps.to_string().len()
        // );

        match option {
            Info::Verbose => {}
            Info::Normal => {
                println!("{esc}c", esc = 27 as char);
            }
        }

        for r in 0..$reps {
            let mut schedule: Schedule = Schedule::new();
            state.init(&mut schedule);
            let start = std::time::Instant::now();
            //let pb = ProgressBar::new(n_step);
            for i in 0..n_step {
                schedule.step(state);
                if state.end_condition(&mut schedule) {
                    break;
                }
                //pb.inc(1);
            }
            //pb.finish_with_message("\u{1F980}");

            let run_duration = start.elapsed();

            match option {
                Info::Verbose => {}
                Info::Normal => {
                    println!("{esc}c", esc = 27 as char);
                    println!("\u{1F980} Rust-AB v1.0\n");
                    println!(
                        "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                        "#Rep", "Steps", "Avg. Steps/Seconds", "Avg. Time"
                    );
                    println!("----------------------------------------------------------------");
                }
            }

            // let step_seconds =
            //     format!("{:.0}", schedule.step as f32 / (run_duration.as_secs_f32()));
            // let time = format!("{:.4}", run_duration.as_secs_f32());
            // print!("{:width$}|", (r + 1), width = 14 - $reps.to_string().len());
            // print!(
            //     "{:width$}|",
            //     schedule.step,
            //     width = 15 - n_step.to_string().len() - $reps.to_string().len()
            // );
            // print!("{:width$}", "", width = 13 - step_seconds.len());

            results.push((
                run_duration,
                schedule.step as f32 / (run_duration.as_nanos() as f32 * 1e-9),
            ));

            match option {
                Info::Verbose => {
                    // print!("{}|", step_seconds);
                    // print!("{:width$}", "", width = 9 - time.len());
                    // println!("{}s|", time);
                }
                Info::Normal => {
                    let mut avg_time = 0.0;
                    let mut avg_step_seconds = 0.0;
                    for (time, step_seconds) in &results {
                        avg_time += time.as_secs_f32();
                        avg_step_seconds += step_seconds;
                    }
                    avg_time /= results.len() as f32;
                    avg_step_seconds /= results.len() as f32;
                    let avg_step_seconds = format!("{:.2}", avg_step_seconds);
                    let avg_time = format!("{:.4}", avg_time);
                    print!("{}|", avg_step_seconds);
                    print!("{:width$}", "", width = 9 - avg_time.len());
                    println!("{}s|", avg_time);
                }
            }
        }
        results
    }};
}

#[macro_use]
mod no_exported {
    #[macro_export]
    macro_rules! replace_expr {
        ($_t:tt $sub:expr) => {
            $sub
        };
    }

    //Used to count tokens of an expansion
    #[macro_export]
    macro_rules! count_tts {
        ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
    }

    #[macro_export]
    macro_rules! build_configurations{

        ($n_conf: expr, $( $input:ident )*) =>{{
        let mut config_table_index:Vec<Vec<usize>> = Vec::new();
        let mut input_size:usize = 0;
        let mut rep = $n_conf;
        {
            $(
                let mut row:Vec<usize> = Vec::with_capacity($n_conf);
                input_size = $input.len();
                rep /= input_size;
                let mut i = 0;
                for _ in 0..$n_conf{
                    for _ in 0..rep{
                            row.push(i);
                    }
                    i = (i + 1) % input_size;
                }
                config_table_index.push(row);
            )*
        }

        config_table_index
        }};

    }
}

///Create a csv file with the experiment results
///"DataFrame" trait allow the function to know field names and
///params list + output list for each configuration runned
pub fn write_csv<A: DataFrame>(name: &str, dataframe: &[A]) -> Result<(), Box<dyn Error>> {
    let csv_name = format!("{}.csv", name);
    let mut wtr = Writer::from_path(csv_name).unwrap();
    //define column name
    wtr.write_record(A::field_names())?;

    for row in dataframe {
        wtr.serialize(row.to_string())?;
    }

    Ok(())
}

///Trait implemented dynamically for our dataframe struct.
///Used into "export_dataframe" function
pub trait DataFrame {
    fn field_names() -> &'static [&'static str];
    fn to_string(&self) -> Vec<String>;
}

///Generate parameter values using a Uniform Distribution
///Params: Type, Min, Max and number of samples
///n_samples is optional, if omitted only a single sample is computed
#[macro_export]
macro_rules! gen_param {
    ( $type:ty, $min:expr, $max:expr, $n:expr) => {{
        let minimum: $type;
        let maximum: $type;
        minimum = $min;
        maximum = $max;
        let mut n = $n as usize;

        // Check parameters range to avoid error with Distribution
        let (minimum, maximum) = if minimum > maximum {
            (maximum, minimum)
        } else if minimum == maximum {
            (minimum, maximum + 1 as $type)
        } else {
            (minimum, maximum)
        };

        if n == 0 {
            n = 1;
        }

        let between = Uniform::from(minimum..maximum);
        let mut rng = rand::thread_rng();
        let dist: Vec<$type> = between.sample_iter(&mut rng).take($n).collect();

        dist
    }};

    // gen a single value
    (  $type:ty, $min:expr, $max:expr) => {{
        gen_param!($type, $min, $max, 1)
    }};
}

#[macro_export]
macro_rules! load_csv {

    ($input_file: expr, $( $x:ident: $x_ty: ty ),*) =>{{
        let mut rdr = Reader::from_path($input_file).unwrap();
        $(
            let mut $x: Vec<$x_ty> = Vec::new();
        )*
        for result in rdr.records() {
            let record = result.unwrap();
            let mut i = 0;
            $(
                let x : $x_ty = record[i].parse().unwrap();
                $x.push(x);
                i += 1;
            )*
        }
        let v = ($( $x, )*);
        v
    }};
}
