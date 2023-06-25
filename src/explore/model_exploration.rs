#[macro_export]
/// Macro to perform a model exploration.
/// The macro will generate a dataframe with the results of the exploration.
/// The dataframe can be saved in a csv file.
///
/// # Arguments
/// * `step` - simulation step number,
/// * `repconf` - number of repetitions
/// * `state` - state of the simulation
/// * `input` - multiple custom input, pair of a identifier and its type
/// * `output` - multiple custom input, pair of a identifier and its type
/// * `explore_mode` - enum to choose how manage parameters combination (Supported option: Exhaustive, Matched)
///
/// Last parameter is the computing mode to use.
/// Without the last parameter, the macro will use the default computing mode (Sequential).
/// The computing mode can be:
/// * `ComputingMode::Parallel`: the exploration will be performed in parallel
/// * `ComputingMode::Distributed`: the exploration will be performed distributing the computation
///    on different machines
/// * `ComputingMode::Cloud`: computation will be performed on the cloud.
///
/// # Example
///
/// ```
/// let param1 = gen_param!(u32, 0, 10, 5);
/// let param2 = gen_param!(f64, 0, 10, 5);
///
/// // implement trait State
/// struct State {  
///   param: u32,
///   param2: f64,
///   result: f64,
/// }
///
/// // input and input_vec are input of State constructor
/// // outputs are fields of State to get results
/// // ComputingMode by default is Sequential
/// explore!(
///     STEP,
///     rep_conf, // How many times run a configuration
///     State,
///     input {
///        param: u32,
///        param2: f64,
///     },
///     output [
///       result: f64,
///     ],
///     ExploreMode::Matched,
///     // ComputingMode::Parallel, ComputingMode::Distributed, ComputingMode::Cloud
/// );
///
///
///
/// ````
///
macro_rules! explore {
    (
        $nstep: expr, $rep_conf:expr, $state:ty,
        input {$($input:ident: $input_ty: ty )*},
        input_vec {$($input_vec:ident:  [$input_ty_vec:ty; $input_len:expr])*},
        output [$($output:ident: $output_ty: ty )*],
        $explore_mode: expr,
        $computing_mode: expr
    ) => {{
        use $crate::cfg_if::cfg_if;
        use $crate::engine::schedule::Schedule;
        use $crate::engine::state::State;
        use $crate::ComputingMode;

        let cp_mode : ComputingMode = $computing_mode;
        match cp_mode {
            ComputingMode::Parallel => {
                cfg_if!{
                    if #[cfg(not(any(feature = "distributed_mpi", feature = "cloud")))] {
                        println!("Parallel exploration");
                        explore_parallel!($nstep, $rep_conf, $state,
                            input {$($input: $input_ty )*},
                            output [$($output: $output_ty )*],
                            $explore_mode,
                        )
                    } else {
                        panic!("Parallel computing mode doesn't require distributed or cloud features");
                    }
                }
            },
            ComputingMode::Distributed => {
                cfg_if!{
                    if #[cfg(feature="distributed_mpi")] {
                        explore_distributed_mpi!($nstep, $rep_conf, $state,
                            input {$($input: $input_ty )*},
                            input_vec {$($input_vec:  [$input_ty_vec; $input_len])*},
                            output [$($output: $output_ty )*],
                            $explore_mode,
                        )

                    } else {
                        panic!("Distributed computing mode is not available. Please enable the feature 'distributed_mpi' to use this mode.");
                    }
                }

            },
            ComputingMode::Cloud => {
                cfg_if!{
                    if #[cfg(feature="aws")] {
                        println!("Cloud exploration with AWS");
                        println!("WARNING: this mode is not yet implemented");
                        // explore_cloud!($nstep, $rep_conf, $state,
                        //     input {$($input: $input_ty )*},
                        //     output [$($output: $output_ty )*],
                        //     $explore_mode,
                        // );
                    }
                    else {
                        panic!("Cloud computing mode is not available. Please enable the feature 'aws' to use this mode.");
                    }
                }
            }

            _ => { panic!("Computing mode not supported"); }

        }
    }};

    (
        $nstep: expr, $rep_conf:expr, $state:ty,
        input {$($input:ident: $input_ty: ty )*},
        output [$($output:ident: $output_ty: ty )*],
        $explore_mode: expr,
        $computing_mode: expr
    ) => {{
        explore!($nstep, $rep_conf, $state,
            input {$($input: $input_ty )*},
            input_vec {},
            output [$($output: $output_ty )*],
            $explore_mode,
            $computing_mode
        )
    }};

    (
    $nstep: expr, $rep_conf:expr, $state:ty,
    input {$($input:ident: $input_ty: ty )*},
    output [$($output:ident: $output_ty: ty )*],
    $mode: expr
    ) => {{
        use $crate::engine::schedule::Schedule;
        use $crate::engine::state::State;
        explore_sequential!($nstep, $rep_conf, $state, input {$($input: $input_ty )*}, output [$($output: $output_ty )*], $mode,)
    }}

}

#[doc(hidden)]
/// Internal function to run the simulation inside the explore macros
///
/// step : number of total step of the simulation
///
/// state : the State of the simulation
#[macro_export]
macro_rules! simulate_explore {
    ($step:expr, $state:expr) => {{
        let mut s = $state;
        let mut state = s.as_state_mut();
        let n_step: u32 = $step;

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

        $state = s; // needed for model_exploration, requires also the state to be mut
        results
    }};
}

#[macro_export]
/// Macro to to perform sequential model exploration using basic parameters sweeping
///
/// # Arguments
/// * `step` - simulation step number,
/// * `repconf` - number of repetitions
/// * `state` - state of the simulation
/// * `input {name: type}` - input parameters of simulation
/// * `output [name: type]` - output parameters of simulation
/// * `mode` - enum to choose which mode of execution is desired (Supported option: Exhaustive, Matched)
///
/// # Example
///
/// ```
/// let param1 = gen_param!(u32, 0, 10, 5);
/// let param2 = gen_param!(f64, 0, 10, 5);
///
/// // implement trait State
/// struct State {  
///   param: u32,
///   param2: f64,
///   result: f64,
/// }
///
/// // input and input_vec are input of State constructor
/// // outputs are fields of State to get results
/// explore_sequrntial!(
///     STEP,
///     rep_conf, // How many times run a configuration
///     State,
///     input {
///        param: u32,
///        param2: f64,
///     },
///     output [
///       result: f64,
///     ],
///     ExploreMode::Matched,
/// );
/// ```
macro_rules! explore_sequential {

        //exploration with explicit output parameters
        ($nstep: expr, $rep_conf:expr, $state:ty,
        input {$($input:ident: $input_ty: ty )*},
        output [$($output:ident: $output_ty: ty )*],
        $mode: expr,
         ) => {{

            println!("Running sequential model exploration...");

            //typecheck
            let rep_conf = $rep_conf as usize;
            let nstep = $nstep as u32;

            let mut n_conf:usize = 1;
            let mut config_table_index: Vec<Vec<usize>> = Vec::new();
            build_dataframe!(FrameRow, input {$( $input:$input_ty)* }, output[ $( $output:$output_ty )*] );

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

            println!("Number of configuration in input {}", n_conf);

            let mut dataframe: Vec<FrameRow>  = Vec::new();

            for i in 0..n_conf{
                let mut state;
                // check which mode to use to generate the configurations
                match $mode {
                    // use all the possible combination
                    ExploreMode::Exaustive =>{
                        let mut row_count = -1.;
                        state = <$state>::new(
                            $(
                            $input[config_table_index[{row_count+=1.; row_count as usize}][i]].clone(),
                            )*
                        );
                    },
                    // create a configuration using the combination of input with the same index
                    ExploreMode::Matched =>{
                        state = <$state>::new(
                            $(
                                $input[i].clone(),
                            )*
                        );
                    }
                }

                println!("\n- Configuration {}", i);
                $(
                    println!("-- {}: {:?}", stringify!(state.$input), state.$input);
                )*

                for j in 0..rep_conf{
                    println!("Running simulation {}", j+1);
                    let result = simulate_explore!(nstep, state);
                    dataframe.push(
                        FrameRow::new(i as u32, (j + 1) as u32, $(state.$input.clone(),)* $(state.$output,)* result[0].0, result[0].1)
                    );
                }
            }
            dataframe

        }};

        //exploration taking default output: total time and step per second
        ($nstep: expr, $rep_conf:expr, $state:expr, input {$($input:ident: $input_ty: ty )*}, $mode:expr) => {
            explore_sequential!($nstep, $state, $rep_conf, input {$($input: $input_ty)*}, output [], $mode)
        }

    }

#[macro_export]
/// Macro to to perform parallel model exploration using basic parameters sweeping
///
/// # Arguments
/// * `step` - simulation step number,
/// * `repconf` - number of repetitions
/// * `state` - state of the simulation
/// * `input {name: type}` - input parameters of simulation
/// * `output [name: type]` - output parameters of simulation
/// * `mode` - enum to choose which mode of execution is desired (Supported option: Exhaustive, Matched)
///
/// # Example
///
/// ```
/// let param1 = gen_param!(u32, 0, 10, 5);
/// let param2 = gen_param!(f64, 0, 10, 5);
///
/// // implement trait State
/// struct State {  
///   param: u32,
///   param2: f64,
///   result: f64,
/// }
///
/// // input and input_vec are input of State constructor
/// // outputs are fields of State to get results
/// let result = explore_parallel!(
///     STEP,
///     rep_conf, // How many times run a configuration
///     State,
///     input {
///        param: u32,
///        param2: f64,
///     },
///     output [
///       result: f64,
///     ],
///     ExploreMode::Matched,
/// );
/// ```
macro_rules! explore_parallel {
        ($nstep: expr, $rep_conf:expr, $state:ty,
            input {$($input:ident: $input_ty: ty )*},
            output [$($output:ident: $output_ty: ty )*],
            $mode: expr,
             ) => {{

            println!("Running parallel model exploration...");

            //typecheck
            let _rep_conf = $rep_conf as usize;
            let _nstep = $nstep as u32;

            let mut n_conf:usize = 1;
            let mut config_table_index: Vec<Vec<usize>> = Vec::new();

            build_dataframe!(FrameRow, input {$( $input:$input_ty)* }, output[ $( $output:$output_ty )*]);

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

            println!("Number of configuration in input {}", n_conf);

            //create a task for each simulation
            let dataframe: Vec<FrameRow> = (0..n_conf*$rep_conf).into_par_iter().map( |run| {
                let i  = run / $rep_conf;

                let mut state;
                // check which mode to use to generate the configurations
                match $mode {
                    // use all the possible combination
                    ExploreMode::Exaustive =>{
                        let mut row_count = -1.;
                        state = <$state>::new(
                            $(
                            $input[config_table_index[{row_count+=1.; row_count as usize}][i]],
                            )*
                        );
                    },
                    // create a configuration using the combination of input with the same index
                    ExploreMode::Matched =>{
                        state = <$state>::new(
                            $(
                                $input[i],
                            )*
                        );
                    },
                }

                println!("\n- Configuration {}", i);
                $(
                    println!("-- {}: {:?}", stringify!(state.$input), state.$input);
                )*

                println!("\nRunning simulation {} of configuration {}", run % $rep_conf, i);
                let result = simulate_explore!($nstep, state);
                FrameRow::new(i as u32, (run % $rep_conf) as u32, $(state.$input,)* $(state.$output,)* result[0].0, result[0].1)
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

#[doc(hidden)]
#[macro_export]
/// Internal function for automatic building the structure for the Dataframe
///
/// The dataframe allow to write the data of the simulations into a comfort structure that can be saved inside a file or easily manipulated
///
/// Complete pattern of the macro
///
/// name : custom name of the structure
///
/// input : multiple pairs of identifier and type
///
/// input_vec : vectors of elements, must specify the identifier, the type and the vector length
///
/// output : multiple pairs of identifier and type
///
/// derive : optional parameter for the derive directive
macro_rules! build_dataframe {
        //Dataframe with input and output parameters and optional parameters
        (
            $name:ident,
            input {$($input: ident: $input_ty: ty)*},
            input_vec {$($input_vec:ident: [$input_ty_vec:ty; $input_len:expr])*},
            output [$($output: ident: $output_ty: ty)*]
            $($derive: tt)*
        ) => {

            #[derive(Default, Clone, PartialEq, Debug, $($derive,)*)]
            struct $name {
                pub conf_num: u32,
                pub conf_rep: u32,
                $(pub $input: $input_ty,)*
                $(pub $output: $output_ty,)*
                $(pub $input_vec: [$input_ty_vec; $input_len],)*
                pub run_duration: f32,
                pub step_per_sec: f32,
            }

            impl DataFrame for $name{
                fn field_names() -> &'static [&'static str] {
                    static NAMES: &'static [&'static str]
                        = &["Simulation", "Run", $(stringify!($input),)* $(stringify!($input_vec),)* $(stringify!($output),)*  "Run Duration", "Step per sec"];
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
                        v.push(format!("{:?}", self.$input_vec));
                    )*
                    $(
                        v.push(format!("{:?}", self.$output));
                    )*
                    v.push(self.run_duration.to_string());
                    v.push(self.step_per_sec.to_string());

                    v
                }

            }

            impl $name {
                pub fn new(
                    conf_num: u32, conf_rep: u32, $($input: $input_ty,)* $($input_vec: [$input_ty_vec; $input_len],)* $($output: $output_ty,)* run_duration: f32, step_per_sec: f32,
                ) -> $name{
                    $name {
                        conf_num,
                        conf_rep,
                        $(
                            $input,
                        )*
                        $(
                            $input_vec,
                        )*
                        $(
                            $output,
                        )*
                        run_duration,
                        step_per_sec,

                    }
                }
            }


        };

        (
            $name:ident,
            input {$($input: ident: $input_ty: ty)*},
            output [$($output: ident: $output_ty: ty)*]
            $($derive: tt)*
        ) => {
                build_dataframe!(
                        $name,
                        input {$($input: $input_ty)*},
                        input_vec { },
                        output [$($output: $output_ty)*]
                        $($derive)*
                )
        };
}
