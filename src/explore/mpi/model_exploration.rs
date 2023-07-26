#[doc(hidden)]
#[macro_export]
/// internal macro to extend the dataframe to standardize the structure for MPI protocol
///
/// name: identifier of the custom structure
///
/// input: all the input fields of the structure
///
/// input_vec: all the input vectors fields of the structure
///
/// output: all the output fields of the structure
macro_rules! extend_dataframe {
    //Dataframe with input and output parameters and optional parameters
    (
        $name:ident,
        input {$($input: ident: $input_ty: ty)*},
        input_vec {$($input_vec:ident: [$input_ty_vec:ty; $input_len: expr])*},
        output [$($output: ident: $output_ty: ty)*]
    ) =>{
        unsafe impl Equivalence for $name {
            type Out = UserDatatype;
            fn equivalent_datatype() -> Self::Out {

                //count input and output parameters to create slice for block lengths
                let v_in = count_tts!($($input)*);
                let v_out = count_tts!($($output)*);

                //count vec optional parameters
                let v_vec_in = count_tts!($($input_vec)*);

                let dim = v_in + v_out + v_vec_in + 4;
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
                            offset_of!($name, $input_vec) as Address,
                        )*
                        $(
                            offset_of!($name, $output) as Address,
                        )*
                        offset_of!($name, run_duration) as Address,
                        offset_of!($name, step_per_sec) as Address,

                    ],
                    &[
                        u32::equivalent_datatype(),
                        u32::equivalent_datatype(),
                        $(
                            <$input_ty>::equivalent_datatype(),
                        )*
                        $(
                            UserDatatype::contiguous($input_len, &<$input_ty_vec>::equivalent_datatype()).as_ref(),
                        )*
                        $(
                            <$output_ty>::equivalent_datatype(),
                        )*
                        f32::equivalent_datatype(),
                        f32::equivalent_datatype(),

                    ]
                )
            }
        }
    };
}

/// Macro to perform distributed model exploration using basic parameter sweeping based on MPI
///
/// * `nstep` - number of steps of the single simulation
/// * `rep_conf` - how many times run a configuration
/// * `state` - struct name implementing trait State
/// * `input {name: type}` - input parameters of simulation
/// * `input_vec { name : [type, size] }` - array params of simulations
/// * `output [name: type]` - output parameters of simulation
/// * `mode` - enum to choose which mode of execution is desired (Supported option: Exhaustive, Matched)
///
/// # Example
///
/// ```
/// let param = vec![1,2,3];
/// let param_array  = vec![[1,2], [3,4], [5,6]];
///
/// // implement trait State
/// struct State {  
///   param: u32,
///   param_array: [u32; 2],
///   result: u32,
/// }
///
/// // input and input_vec are input of State constructor
/// // outputs are fields of State to get results
/// let result = explore_distributed_mpi!(
///     STEP,
///     rep_conf, // How many times run a configuration
///     State,
///     input {
///        param: u32,
///     },
///     input_vec {
///         param_array: [u32; 2],
///     },
///     output [
///        result: u32,
///     ],
///     ExploreMode::Matched,
/// );
///
/// ```
#[macro_export]
macro_rules! explore_distributed_mpi {
        ($nstep: expr, $rep_conf:expr, $state:ty,
        input {$($input:ident: $input_ty: ty )*},
        input_vec {$($input_vec:ident:  [$input_ty_vec:ty; $input_len:expr])*},
        output [$($output:ident: $output_ty: ty )*],
        $mode: expr,
        ) => {{

            // mpi initialization
            let world = UNIVERSE.world();
            let root_rank = 0;
            let root_process = world.process_at_rank(root_rank);
            let my_rank = world.rank();
            let num_procs = world.size() as usize;

            if world.rank() == root_rank {
                println!("Running distributed (MPI) model exploration...");
            }

            //typecheck
            let mut rep_conf = $rep_conf as usize;
            let _nstep = $nstep as u32;


            if rep_conf <= 0 { rep_conf = 1;}

            build_dataframe!(FrameRow,
                input { $($input:$input_ty)* },
                input_vec { $($input_vec: [$input_ty_vec; $input_len] )* },
                output[ $($output:$output_ty)*]
                Copy);

            extend_dataframe!(FrameRow,
                input {$($input:$input_ty)* },
                input_vec { $($input_vec: [$input_ty_vec; $input_len] )* },
                output[ $( $output:$output_ty )*]
            );



            let mut n_conf:usize = 1;
            let mut config_table_index: Vec<Vec<usize>> = Vec::new();

            // check which mode to use for the exploration
            match $mode {
                ExploreMode::Exaustive =>{
                    $( n_conf *= $input.len(); )*
                    $( n_conf *= $input_vec.len(); )*
                    //Cartesian product with variadics, to build a table with all parameter combinations
                    //They are of different type, so i have to work with indexes
                    config_table_index = build_configurations!(n_conf, $($input )* $($input_vec)*);
                },
                ExploreMode::Matched =>{
                    $( n_conf = $input.len(); )*
                    $( n_conf = $input_vec.len(); )*

                },
            }

            let total_sim = n_conf*rep_conf;

            if world.rank() == root_rank {
                println!("Total simulations: {}", total_sim);
                println!("Total configurations: {}", n_conf);
            }

            let mut local_conf_size: usize = n_conf/num_procs;

            //load balancing extra configuration
            if (my_rank as usize) < n_conf%num_procs {
                local_conf_size += 1;
            }

            println!("Processor {}: assigned {} configurations", my_rank, local_conf_size);

            let mut dataframe: Vec<FrameRow>  = Vec::new();
            for i in 0..local_conf_size {
                let mut state;
                // check which mode to use to generate the configurations
                match $mode {
                    // use all the possible combination
                    ExploreMode::Exaustive =>{
                        let mut row_count = -1.;
                        state = <$state>::new(
                            $(
                            $input[config_table_index[{row_count+=1.; row_count as usize}][i*num_procs + (my_rank as usize)]],
                            )*
                            $(
                            $input_vec[config_table_index[{row_count+=1.; row_count as usize}][i*num_procs + (my_rank as usize)]].clone(),
                            )*
                        );
                    },
                    // create a configuration using the combination of input with the same index
                    ExploreMode::Matched =>{
                        state = <$state>::new(
                            $(
                                $input[i*num_procs + (my_rank as usize)],
                            )*
                            $(
                                $input_vec[i*num_procs + (my_rank as usize)].clone(),
                            )*
                        );
                    },
                }

                // execute the exploration for each configuration
                for j in 0..rep_conf{
                    println!("Running configuration #{} - Simulation #{} on processor #{}", i*num_procs + (my_rank as usize), j, my_rank);
                    let result = simulate_explore!($nstep, state);

                    //convert a Vec into a fixed size slice, because Vec can't be sent as MPI message
                    $(
                        let mut $input_vec: [$input_ty_vec; $input_len] = [0; $input_len];
                        let slice = state.$input_vec.clone();
                        $input_vec.copy_from_slice(&slice[..]);
                    )*

                    dataframe.push(
                        FrameRow::new(i as u32, j as u32, $(state.$input,)* $($input_vec,)* $(state.$output,)* result[0].0, result[0].1)
                    );
                }
            }


            // must return a dummy dataframe that will not be used
            // since only the master write the complete dataframe of all procs on csv
            if world.rank() == root_rank {

                let mut samples_count: Vec<Count> = Vec::new();


                for i in 0..num_procs {
                    if i < n_conf%num_procs || n_conf%num_procs == 0 {
                        let temp:usize = local_conf_size*(rep_conf as usize);
                        samples_count.push(temp as Count);
                    }
                    else {
                        let temp:usize = (local_conf_size-1)*(rep_conf as usize);
                        samples_count.push(temp as Count);
                    }
                }


                let displs: Vec<Count> = samples_count
                    .iter()
                    .scan(0, |acc, &x| {
                        let tmp = *acc;
                        *acc += x;
                        Some(tmp)
                    })
                    .collect();

                let mut all_dataframe = vec![dataframe[0]; n_conf*$rep_conf];

                let mut partition = PartitionMut::new(&mut all_dataframe[..], samples_count.clone(), &displs[..]);

                // root receives all results from other processors
                root_process.gather_varcount_into_root(&dataframe[..], &mut partition);
                // root_process.gather_into_root(&dataframe[..], &mut all_dataframe[..]);
                all_dataframe
            } else {
                //every proc send to root every row
                root_process.gather_varcount_into(&dataframe[..]);
                //return dummy dataframe
                dataframe = Vec::new();
                dataframe
            }

        }};

        //exploration taking default output: total time and step per second
        ($nstep: expr, $rep_conf:expr, $state_name:ty,
        input {$($input:ident: $input_ty: ty )*,},
        input_vec {$($input_vec:ident: [$input_vec_type:ty; $input_vec_len:expr])*},
        $mode: expr,
        ) => {
                explore_distributed_mpi!($nstep, $rep_conf, $state_name,
                input { $($input: $input_ty)*},
                input_vec { $($input_vec: [$input_vec_type; $input_vec_len])* },
                output [],
                $mode,)
        };

        //exploration taking  no vec input and default output: total time and step per second
        ($nstep: expr, $rep_conf:expr, $state_name:ty,
        input {$($input:ident: $input_ty: ty )*,},
        $mode: expr,
        ) => {
                explore_distributed_mpi!($nstep, $rep_conf, $state_name,
                input { $($input: $input_ty)*},
                input_vec { },
                output [],
                $mode,)
        };


        //exploration taking vec input and default output: total time and step per second
        ($nstep: expr, $rep_conf:expr, $state_name:ty,
        input_vec {$($input_vec:ident: [$input_vec_type:ty; $input_vec_len:expr])*},
        $mode: expr,
        ) => {
                explore_distributed_mpi!($nstep, $rep_conf, $state_name,
                input { },
                input_vec { $($input_vec: [$input_vec_type; $input_vec_len])* },
                output [],
                $mode,)
        };

    }
