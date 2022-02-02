#[macro_export]
macro_rules! simulate_explore {
    ($step:expr, $state:expr) => {{
        let mut s = $state;
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

        $state = s; // needed for model_exploration, requires also the state to be mut
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
        ($nstep: expr, $rep_conf:expr, $state:ty,
        input {$($input:ident: $input_ty: ty )*},
        output [$($output:ident: $output_ty: ty )*],
        $mode: expr,
         ) => {{

            println!("Running sequential model exploration...");

            //typecheck
            let _rep_conf = $rep_conf as usize;
            let _nstep = $nstep as u32;

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

                for j in 0..$rep_conf{
                    println!("Running simulation {}", j+1);
                    let result = simulate_explore!($nstep, state);
                    dataframe.push(
                        FrameRow::new(i as u32, j + 1 as u32, $(state.$input.clone(),)* $(state.$output,)* result[0].0, result[0].1)
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

#[macro_export]
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

/* #[macro_export]
//macro general to call exploration
macro_rules! explore {

    //exploration with explicit output parameters
    ($nstep: expr, $rep_conf:expr, $state:ty,
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
                $nstep, $rep_conf, $state, input {$($input: $input_ty)*}, output [$($output: $output_ty)*], $mode, $( $x ),*
            ),
            ComputationMode::Parallel => explore_parallel!(
                $nstep, $rep_conf, $state, input {$($input: $input_ty)*}, output [$($output: $output_ty)*], $mode, $( $x ),*
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
