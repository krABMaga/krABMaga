pub mod engine;
pub mod utils;
pub use indicatif::ProgressBar;
pub use rand;

#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub mod visualization;

#[cfg(any(feature = "visualization", feature = "visualization_wasm", doc))]
pub use bevy;

pub use rand::distributions::{Distribution, Uniform};

pub use csv::Writer;
pub use rayon::prelude::*;
pub use std::time::Duration;

use std::error::Error;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Info {
    VERBOSE,
    NORMAL,
}


#[macro_export]
//step = simulation step number
//schedule
//agents
//states
//other parametes
macro_rules! simulate {
    ($step:expr, $s:expr, $reps:expr, $info:expr) => {{
        let mut s = $s;
        let mut state = s.as_state_mut();
        let n_step: u64 = $step;

        let mut results: Vec<(Duration, f32)> = Vec::new();
        let option = $info;

        match option {
            Info::VERBOSE => {
                println!("\u{1F980} Rust-AB v1.0\n");
                println!(
                    "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                    "#Rep", "Steps", "Steps/Seconds", "Time"
                );
                println!("--------------------------------------------------");
            }
            Info::NORMAL => {
                println!("{esc}c", esc = 27 as char);
                println!("\u{1F980} Rust-AB v1.0\n");
                println!(
                    "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                    "#Rep", "Steps", "Avg. Steps/Seconds", "Avg. Time"
                );
                println!("----------------------------------------------------------------");
            }
        }
        print!("{:width$}|", 1, width = 14 - $reps.to_string().len());
        println!(
            "{:width$}|",
            n_step,
            width = 15 - n_step.to_string().len() - $reps.to_string().len()
        );
        // println!("{esc}c", esc = 27 as char);

        for r in 0..$reps {
            let mut schedule: Schedule = Schedule::new();
            state.init(&mut schedule);
            let start = std::time::Instant::now();
            let pb = ProgressBar::new(n_step);
            for i in 0..n_step {
                schedule.step(state);
                if state.end_condition(&mut schedule) {
                    break;
                }
                pb.inc(1);
            }
            pb.finish_with_message("\u{1F980}");

            let run_duration = start.elapsed();

            match option {
                Info::VERBOSE => {}
                Info::NORMAL => {
                    println!("{esc}c", esc = 27 as char);
                    println!("\u{1F980} Rust-AB v1.0\n");
                    println!(
                        "{0: >10}|{1: >9}|    {2: >11}|{3: >10}|",
                        "#Rep", "Steps", "Avg. Steps/Seconds", "Avg. Time"
                    );
                    println!("----------------------------------------------------------------");
                }
            }

            let step_seconds =
                format!("{:.0}", schedule.step as f32 / (run_duration.as_secs_f32()));
            let time = format!("{:.4}", run_duration.as_secs_f32());
            print!("{:width$}|", (r + 1), width = 14 - $reps.to_string().len());
            print!(
                "{:width$}|",
                schedule.step,
                width = 15 - n_step.to_string().len() - $reps.to_string().len()
            );
            print!("{:width$}", "", width = 13 - step_seconds.len());

            results.push((
                run_duration,
                schedule.step as f32 / (run_duration.as_nanos() as f32 * 1e-9),
            ));

            match option {
                Info::VERBOSE => {
                    print!("{}|", step_seconds);
                    print!("{:width$}", "", width = 9 - time.len());
                    println!("{}s|", time);
                }
                Info::NORMAL => {
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
        // $s = s; // needed for model_exploration, requires also the state to be mut
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
pub fn export_dataframe<A: DataFrame>(
    name: &str,
    dataframe: &Vec<A>,
) -> Result<(), Box<dyn Error>> {
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
///We use it into "export_dataframe" function
pub trait DataFrame {
    fn field_names() -> &'static [&'static str];
    fn to_string(&self) -> Vec<String>;
}

///Generate parameter values using a Uniform Distribution
///Params: Type, Min, Max and number of samples
///n_samples is optional, can be omitted if you want a single sample
#[macro_export]
macro_rules! gen_param {
    ( $type:ty, $min:expr, $max:expr, $n:expr) => {{
        let minimum: $type;
        let maximum: $type;
        minimum = $min;
        maximum = $max;
        let mut n = $n as usize;

        //Check range parameters to avoid error with Distribution
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

    //gen a single value
    (  $type:ty, $min:expr, $max:expr) => {{
        gen_param!($type, $min, $max, 1)
    }};
}

#[macro_export]
macro_rules! build_dataframe {
    //Dataframe with input and output parameters
    ($name:ident, input {$($input: ident: $input_ty: ty)*,}, output [$($output: ident: $output_ty: ty)*]) => {

        #[derive(Debug)]
        struct $name {
            pub conf_num: u128,
            pub conf_rep: u128,
            $(pub $input: $input_ty,)*
            $(pub $output: $output_ty,)*
            pub run_duration: Duration,
            pub step_per_sec: f32

        }

        impl DataFrame for $name{
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &["Simulation", "Run", $(stringify!($input),)* $(stringify!($output),)*  "Run Duration", "Step per sec"];
                NAMES
            }

            fn to_string(&self) -> Vec<String> {
                let mut v: Vec<String> = Vec::new();
                v.push(self.conf_num.to_string());
                v.push(self.conf_rep.to_string());
                $(
                    v.push(self.$input.to_string());
                )*
                $(
                    v.push(self.$output.to_string());
                )*
                v.push(format!("{:?}", self.run_duration));
                v.push(self.step_per_sec.to_string());
                v
            }

        }

        impl $name {
            pub fn new( conf_num: u128, conf_rep: u128 $(, $input: $input_ty)* $(, $output: $output_ty)*, run_duration: Duration, step_per_sec: f32) -> $name{
                $name {
                    conf_num,
                    conf_rep,
                    $(
                        $input,
                    )*
                    $(
                        $output,
                    )*
                    run_duration,
                    step_per_sec
                }
            }
        }
    };

    //Dataframe with only input parameters
    ($name:ident $(, $element: ident: $input_ty: ty)*) => {
        build_dataframe!($name, input{$($element: $input_ty)*,}, output[]);
    };
}

///Brute force parameter exploration
#[macro_export]
///step = simulation step number,
///schedule,
///states,
///input{input: tipo},
///output[output: tipo]
macro_rules! explore {
    //exploration with explicit output parameters
    ($nstep: expr, $s:expr, $rep_conf:expr,
     input {$($input:ident: $input_ty: ty )*},
     output [$($output:ident: $output_ty: ty )*]) => {{
        //typecheck
        let _rep_conf = $rep_conf as usize;
        let _nstep = $nstep as u128;

        println!("Calculate number of configuration");
        let mut n_conf:usize = 1;
        $( n_conf *= $input.len(); )*

        println!("n_conf {}", n_conf);

        build_dataframe!(FrameRow, input {$( $input:$input_ty)*, }, output[ $( $output:$output_ty )*]);

        //Cartesian product with variadics, to build a table with all parameter combinations
        //They are of different type, so i have to work with indexes
        let mut config_table_index:Vec<Vec<usize>> = build_configurations!(n_conf, $($input )*);

        let mut dataframe: Vec<FrameRow>  = Vec::new();
        for i in 0..n_conf{
            let mut state = $s; //state = State:new()
            
            let mut row_count = 0;
            $(
                state.$input = $input[config_table_index[row_count][i]];
                row_count+=1;
            )*
            
            println!("-----\nCONF {}", i);
            $(
                println!("{}: {}", stringify!(state.$input), state.$input);
            )*

            for j in 0..$rep_conf{
                println!("------\nRun {}", j+1);
                let result = simulate!($nstep, state, 1, Info::VERBOSE);
                dataframe.push( FrameRow::new(i as u128, j + 1 as u128, $(state.$input,)* $(state.$output,)* result[0].0, result[0].1));
            } 
        }
        dataframe
    }};

    //exploration taking default output: total time and step per second
    ($nstep: expr, $s:expr, $rep_conf:expr, input {$($input:ident: $input_ty: ty )*}) => {
        explore!($nstep, $s, $rep_conf, input {$($input: $input_ty)*}, output [])
    }

}

#[macro_export]
macro_rules! explore_parallel {
    ($nstep: expr, $rep_conf:expr, $state_name:ty, param ($($parameter:expr,)*) , input {$($input:ident: $input_ty: ty )*}, output [$($output:ident: $output_ty: ty )*]) => {{

        //typecheck
        let _rep_conf = $rep_conf as usize;
        let _nstep = $nstep as u128;

        println!("Calculate number of configuration");
        let mut n_conf:usize = 1;
        $( n_conf *= $input.len(); )*
        println!("n_conf {}", n_conf);

        build_dataframe!(FrameRow, input {$( $input:$input_ty)*, }, output[ $( $output:$output_ty )*]);

        //let mut dataframe: Vec<FrameRow> = Vec::with_capacity(n_conf * $rep_conf);

        let mut config_table_index:Vec<Vec<usize>> = build_configurations!(n_conf, $($input )*);

        let dataframe: Vec<FrameRow> = (0..n_conf*$rep_conf).into_par_iter().map( |run| {
            let i  = run / $rep_conf;
            let mut state = <$state_name>::new( $( $parameter ),*);
            let mut row_count = 0;

            $(
                state.$input = $input[config_table_index[row_count][i]];
                row_count+=1;
            )*

            let result = simulate!($nstep, state, 1, Info::NORMAL);
            println!("conf {}, rep {}, run {}", i, run / n_conf, run);
            FrameRow::new(i as u128, (run % $rep_conf) as u128, $(state.$input,)* $(state.$output,)* result[0].0, result[0].1)
        })
        .collect();
        dataframe
    }};

    //exploration taking default output and no state constructor: total time and step per second
    ($nstep: expr, $rep_conf:expr, $state_name:ty, input {$($input:ident: $input_ty: ty )*}) => {
            explore_parallel!($nstep, $rep_conf, $state_name, param (), input { $($input: $input_ty)*}, output [])
    };

    //exploration taking default output: total time and step per second
    ($nstep: expr, $rep_conf:expr, $state_name:ty, param ($($parameter:expr,)*), input {$($input:ident: $input_ty: ty )*}) => {
            explore_parallel!($nstep, $rep_conf, $state_name, param ($($parameter,)*), input { $($input: $input_ty)*}, output [])
    };

    //exploration with no state params constructor
    ($nstep: expr, $rep_conf:expr, $state_name:ty, input {$($input:ident: $input_ty: ty )*}, output [$($output:ident: $output_ty: ty )*]) => {
            explore_parallel!($nstep, $rep_conf, $state_name, param (), input { $($input: $input_ty)*}, output [])
    };

}
