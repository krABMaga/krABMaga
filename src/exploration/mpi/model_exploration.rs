pub use csv::{Reader, Writer};
pub use rayon::prelude::*;
use std::error::Error;
pub use std::fs::File;
pub use std::fs::OpenOptions;
pub use std::io::Write;
pub use std::sync::{Arc, Mutex};
pub use std::time::Duration;

#[cfg(feature = "explore")]
pub use {
    memoffset::{offset_of, span_of},
    mpi::datatype::DynBufferMut,
    mpi::datatype::PartitionMut,
    mpi::point_to_point as p2p,
    mpi::Count,
    mpi::{datatype::UserDatatype, traits::*, Address},
};

#[cfg(feature = "explore")]
pub extern crate mpi;

#[macro_export]
macro_rules! extend_dataframe {
    //Dataframe with input and output parameters and optional parameters
    (
        $name:ident, input {$($input: ident: $input_ty: ty)*}, output [$($output: ident: $output_ty: ty)*], $( $x:ident: $x_ty: ty ),*
    ) =>{
        unsafe impl Equivalence for $name {
            type Out = UserDatatype;
            fn equivalent_datatype() -> Self::Out {

                //count input and output parameters to create slice for blocklen
                let v_in = count_tts!($($input)*);
                let v_out = count_tts!($($output)*);
                let v_x = count_tts!($($x)*);

                let dim = v_in + v_out + v_x + 4;
                let mut vec = Vec::with_capacity(dim);
                for i in 0..dim {
                    vec.push(1);
                }

                UserDatatype::structured(
                    vec.as_slice(),
                    &[
                        offset_of!($name, conf_num) as Address,
                        offset_of!($name, conf_rep) as Address,
                        $(
                            offset_of!($name, $input) as Address,
                        )*
                        $(
                            offset_of!($name, $output) as Address,
                        )*
                        offset_of!($name, run_duration) as Address,
                        offset_of!($name, step_per_sec) as Address,
                        $(
                            offset_of!($name, $x) as Address,
                        )*
                    ],
                    &[
                        u32::equivalent_datatype(),
                        u32::equivalent_datatype(),
                        $(
                            <$input_ty>::equivalent_datatype(),
                        )*
                        $(
                            <$output_ty>::equivalent_datatype(),
                        )*
                        f32::equivalent_datatype(),
                        f32::equivalent_datatype(),
                        $(
                            <$x_ty>::equivalent_datatype(),
                        )*
                    ]
                )
            }
        }
    };
}

#[macro_export]
macro_rules! explore_distributed_mpi {
        ($nstep: expr, $rep_conf:expr, $s:ty,
            input {$($input:ident: $input_ty: ty )*},
            output [$($output:ident: $output_ty: ty )*],
            $mode: expr,
            $( $x:expr ),* ) => {{

            // mpi initilization
            let universe = mpi::initialize().unwrap();
            let world = universe.world();
            let root_rank = 0;
            let root_process = world.process_at_rank(root_rank);
            let my_rank = world.rank();
            let num_procs = world.size() as usize;

            //typecheck
            let _rep_conf = $rep_conf as usize;
            let _nstep = $nstep as u32;

            build_dataframe!(FrameRow, input {$( $input:$input_ty)* }, output[ $( $output:$output_ty )*], $( $x:$x_ty ),* );
            extend_dataframe!(FrameRow, input {$( $input:$input_ty)* }, output[ $( $output:$output_ty )*], $( $x:$x_ty ),* );


            let mut n_conf:usize = 1;
            let mut config_table_index: Vec<Vec<usize>> = Vec::new();

            // check which mode to use for the exploration
            match $mode {
                ExploreMode::Exaustive =>{
                    $( n_conf *= $input.len(); )*
                    //Cartesian product with variadics, to build a table with all parameter combinations
                    //They are of different type, so i have to work with indexes
                    config_table_index = build_configurations!(n_conf, $($input )*);
                },
                ExploreMode::Matched =>{
                    $( n_conf = $input.len(); )*
                },
            }
            println!("n_conf {}", n_conf/num_procs);

            let mut dataframe: Vec<FrameRow>  = Vec::new();
            for i in 0..n_conf/num_procs {
                let mut state;
                // check which mode to use to generate the configurations
                match $mode {
                    // use all the possible combination
                    ExploreMode::Exaustive =>{
                        let mut row_count = -1.;
                        state = <$s>::new(
                            $(
                            $input[config_table_index[{row_count+=1.; row_count as usize}][i*num_procs + (my_rank as usize)]],
                            )*
                        );
                    },
                    // create a configuration using the combination of input with the same index
                    ExploreMode::Matched =>{
                        state = <$s>::new(
                            $(
                                $input[i*num_procs + (my_rank as usize)],
                            )*
                        );
                    },
                }

                // execute the exploration for each configuration
                for j in 0..$rep_conf{
                    println!("conf {}, rep {}, pid: {}", i*num_procs + (my_rank as usize), j, my_rank);
                    let result = simulate_explore!($nstep, state);
                    dataframe.push(
                        FrameRow::new(i as u32, j + 1 as u32, $(state.$input,)* $(state.$output,)* result[0].0, result[0].1, $($x,)*)
                    );
                }
            }

            // must return a dummy dataframe that will not be used
            // since only the master write the complete dataframe of all procs on csv
            if world.rank() == root_rank {
                let mut all_dataframe = vec![dataframe[0]; n_conf];
                root_process.gather_into_root(&dataframe[..], &mut all_dataframe[..]);
                all_dataframe
            } else {
                //every proc send to root every row
                root_process.gather_into(&dataframe[..]);
                //return dummy dataframe
                dataframe = Vec::new();
                dataframe
            }
        }};

        //exploration taking default output: total time and step per second
        ($nstep: expr, $rep_conf:expr, $state_name:ty, input {$($input:ident: $input_ty: ty )*,},
        $mode: expr,
        $( $x:expr ),* ) => {
                explore_distributed_mpi!($nstep, $rep_conf, $state_name, input { $($input: $input_ty)*}, output [],
                $mode, $( $x:expr ),*)
        };
    }

