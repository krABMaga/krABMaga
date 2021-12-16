use crate::build_configurations;

pub use csv::{Reader, Writer};
pub use rayon::prelude::*;
use std::error::Error;
pub use std::fs::File;
pub use std::fs::OpenOptions;
pub use std::io::Write;
pub use std::sync::{Arc, Mutex};
pub use std::time::Duration;

/**
 * 3 mode to generate the data
 * Exaustive: Brute force parameter exploration
 * Matched: explore every input with the same indexes
 * File: Read from file
 */
/* #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExploreMode {
    Exaustive,
    Matched,
} */

#[macro_export]
macro_rules! simulate_explore {
    ($step:expr, $s:expr) => {{
        let mut s = $s;
        let mut state = s.as_state_mut();
        let n_step: u64 = $step;

        let mut results: Vec<(f32, f32)> = Vec::new();

        let mut schedule: Schedule = Schedule::new();
        state.init(&mut schedule);
        let start = std::time::Instant::now();

        for i in 0..n_step {
            schedule.step(state);

            if state.end_condition(&mut schedule) {
                break;
            }
        }

        let run_duration = start.elapsed();

        results.push((
            run_duration.as_secs_f32(),
            schedule.step as f32 / (run_duration.as_nanos() as f32 * 1e-9),
        ));

        $s = s; // needed for model_exploration, requires also the state to be mut
        results
    }};
}

#[macro_export]
///step = simulation step number,
///schedule,
///states,
///input{input:type},
///output[output:type]
macro_rules! explore_sequential {

        //exploration with explicit output parameters
        ($nstep: expr, $rep_conf:expr, $s:ty,
        input {$($input:ident: $input_ty: ty )*},
        output [$($output:ident: $output_ty: ty )*],
        $mode: expr,
        $( $x:expr ),* ) => {{

            //typecheck
            let _rep_conf = $rep_conf as usize;
            let _nstep = $nstep as u32;

            let mut n_conf:usize = 1;
            let mut config_table_index: Vec<Vec<usize>> = Vec::new();

            build_dataframe!(FrameRow, input {$( $input:$input_ty)* }, output[ $( $output:$output_ty )*], $( $x:$x_ty ),* );

            match $mode {
                ExploreMode::Exaustive =>{
                    $( n_conf *= $input.len(); )*
                    //Cartesian product with variadics, to build a table with all parameter combinations
                    //They are of different type, so i have to work with indexes
                    config_table_index = build_configurations!(n_conf, $($input )*);
                },
                ExploreMode::Matched =>{
                    $( n_conf = $input.len(); )*
                }
            }
            println!("n_conf {}", n_conf);

            let mut dataframe: Vec<FrameRow>  = Vec::new();


            for i in 0..n_conf{
                let mut state;
                // check which mode to use to generate the configurations
                match $mode {
                    // use all the possible combination
                    ExploreMode::Exaustive =>{
                        let mut row_count = -1.;
                        state = <$s>::new(
                            $(
                            $input[config_table_index[{row_count+=1.; row_count as usize}][i]],
                            )*
                        );
                    },
                    // create a configuration using the combination of input with the same index
                    ExploreMode::Matched =>{
                        state = <$s>::new(
                            $(
                                $input[i],
                            )*
                        );
                    }
                }

                println!("-----\nCONF {}", i);
                $(
                    println!("{}: {:?}", stringify!(state.$input), state.$input);
                )*

                for j in 0..$rep_conf{
                    println!("------\nRun {}", j+1);
                    let result = simulate_explore!($nstep, state);
                    dataframe.push(
                        FrameRow::new(i as u32, j + 1 as u32, $(state.$input,)* $(state.$output,)* result[0].0, result[0].1, $($x,)*)
                    );
                }
            }
            dataframe
        }};

        //exploration taking default output: total time and step per second
        ($nstep: expr, $rep_conf:expr, $s:expr, input {$($input:ident: $input_ty: ty )*}, $mode:expr) => {
            explore_sequential!($nstep, $s, $rep_conf, input {$($input: $input_ty)*}, output [], $mode)
        }

    }

#[macro_export]
macro_rules! explore_parallel {
        ($nstep: expr, $rep_conf:expr, $s:ty,
            input {$($input:ident: $input_ty: ty )*},
            output [$($output:ident: $output_ty: ty )*],
            $mode: expr,
            $( $x:expr ),* ) => {{

            //typecheck
            let _rep_conf = $rep_conf as usize;
            let _nstep = $nstep as u32;

            let mut n_conf:usize = 1;
            let mut config_table_index: Vec<Vec<usize>> = Vec::new();

            build_dataframe!(FrameRow, input {$( $input:$input_ty)* }, output[ $( $output:$output_ty )*], $( $x:$x_ty ),* );

            match $mode {
                ExploreMode::Exaustive =>{
                    $( n_conf *= $input.len(); )*
                    //Cartesian product with variadics, to build a table with all parameter combinations
                    //They are of different type, so i have to work with indexes
                    config_table_index = build_configurations!(n_conf, $($input )*);
                },
                ExploreMode::Matched =>{
                    $( n_conf = $input.len(); )*
                }
            }
            println!("n_conf {}", n_conf);

            let dataframe: Vec<FrameRow> = (0..n_conf*$rep_conf).into_par_iter().map( |run| {
                let i  = run / $rep_conf;

                let mut state;
                // check which mode to use to generate the configurations
                match $mode {
                    // use all the possible combination
                    ExploreMode::Exaustive =>{
                        let mut row_count = -1.;
                        state = <$s>::new(
                            $(
                            $input[config_table_index[{row_count+=1.; row_count as usize}][i]],
                            )*
                        );
                    },
                    // create a configuration using the combination of input with the same index
                    ExploreMode::Matched =>{
                        state = <$s>::new(
                            $(
                                $input[i],
                            )*
                        );
                    },
                }

                let result = simulate_explore!($nstep, state);
                FrameRow::new(i as u32, (run % $rep_conf) as u32, $(state.$input,)* $(state.$output,)* result[0].0, result[0].1, $($x,)*)
            })
            .collect();
            dataframe
        }};


        //exploration taking default output: total time and step per second
        ($nstep: expr, $rep_conf:expr, $state_name:ty, input {$($input:ident: $input_ty: ty )*,},
        $mode: expr) => {
                explore_parallel!($nstep, $rep_conf, $state_name, input { $($input: $input_ty)*}, output [],
                $mode)
        };
    }

#[macro_export]
macro_rules! build_dataframe {
        //Dataframe with input and output parameters and optional parameters
        (
            $name:ident, input {$($input: ident: $input_ty: ty)*}, output [$($output: ident: $output_ty: ty)*], $( $x:ident: $x_ty: ty ),*
        ) => {

            #[derive(Default, Clone, Copy, PartialEq, Debug)]
            struct $name {
                pub conf_num: u32,
                pub conf_rep: u32,
                $(pub $input: $input_ty,)*
                $(pub $output: $output_ty,)*
                pub run_duration: f32,
                pub step_per_sec: f32,
                $(pub $x: $x_ty,)*
            }

            impl DataFrame for $name{
                fn field_names() -> &'static [&'static str] {
                    static NAMES: &'static [&'static str]
                        = &["Simulation", "Run", $(stringify!($input),)* $(stringify!($output),)*  "Run Duration", "Step per sec", $(stringify!($x),)*];
                    NAMES
                }

                fn to_string(&self) -> Vec<String> {
                    let mut v: Vec<String> = Vec::new();
                    v.push(self.conf_num.to_string());
                    v.push(self.conf_rep.to_string());
                    $(
                        v.push(format!("{:?}", self.$input));
                    )*
                    $(
                        v.push(format!("{:?}", self.$output));
                    )*
                    v.push(self.run_duration.to_string());
                    v.push(self.step_per_sec.to_string());
                    $(
                        v.push(format!("{:?}", self.$x));
                    )*
                    v
                }

            }

            impl $name {
                pub fn new(
                    conf_num: u32, conf_rep: u32 $(, $input: $input_ty)* $(, $output: $output_ty)*, run_duration: f32, step_per_sec: f32 $(, $x: $x_ty)*,
                ) -> $name{
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
                        step_per_sec,
                        $(
                            $x,
                        )*
                    }
                }
            }
        };

        //Dataframe with only input parameters
        ($name:ident, input{$($element: ident: $input_ty: ty)* }) => {
            build_dataframe!($name, input{$($element: $input_ty)*}, output[]);
        };

        //Dataframe without output parameters
        ($name:ident, input {$($input: ident: $input_ty: ty)*}, $( $x:ident: $x_ty: ty ),*) => {
            build_dataframe!($name, input{$($element: $input_ty)*}, output[], $( $x:ident: $x_ty: ty ),*);
        };
    }

/* #[macro_export]
//macro general to call exploration
macro_rules! explore {

    //exploration with explicit output parameters
    ($nstep: expr, $rep_conf:expr, $s:ty,
    input {$($input:ident: $input_ty: ty )*},
    output [$($output:ident: $output_ty: ty )*],
    $mode: expr,
    $cmode: expr,
    $( $x:ident: $x_ty: ty ),*
    ) => {{

        // optional parameters created for distributed mode
        $(
            // create a new variable for optional parameters and pass it as an optional expression
            let $x = $x;
        )*
        build_dataframe!(FrameRow, input {$( $input:$input_ty)* }, output[ $( $output:$output_ty )*], $( $x:$x_ty ),* );
        // check which computation mode is required for the exploration
        match $cmode {
            ComputationMode::Sequential => explore_sequential!(
                $nstep, $rep_conf, $s, input {$($input: $input_ty)*}, output [$($output: $output_ty)*], $mode, $( $x ),*
            ),
            ComputationMode::Parallel => explore_parallel!(
                $nstep, $rep_conf, $s, input {$($input: $input_ty)*}, output [$($output: $output_ty)*], $mode, $( $x ),*
            ),
            _ => Vec::new()
        }
    }};

    ($nstep: expr, $rep_conf:expr, $state_name:ty, input {$($input:ident: $input_ty: ty )*,},
    $mode: expr,
    $cmode: expr) => {
                explore!($nstep, $rep_conf, $state_name, input { $($input: $input_ty)*}, output [],
                $mode, $cmode)
        };

}
 */