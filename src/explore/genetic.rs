//hidden documentation
#[doc(hidden)]
#[macro_export]
/// Internal function for automatic building the structure for the Dataframe.
///
/// The dataframe allow to write the data of the simulations into a comfort structure that can be saved inside a file or easily manipulated
///
/// # Arguments
/// * `name` - The custom name of the dataframe
///
/// * `input` - multiple pairs of identifier and type
///
/// * `vec` - vectors of elements, must specify the identifier, the type and the vector length
///
/// derive : optional parameter for the derive directive
macro_rules! build_dataframe_explore {
    //Dataframe with input and output parameters and optional parameters
    (
        $name:ident,
        input {$($input:ident: $input_ty:ty)*},
        vec {$($input_vec:ident: [$input_ty_vec:ty; $input_len:expr])*}
        $($derive: tt)*
    ) => {

        // create the struct with the given name and all the input values
        #[derive(Clone, Debug,  $($derive,)*)]
        struct $name {
            $(pub $input: $input_ty,)*
            $(pub $input_vec: [$input_ty_vec; $input_len],)*
        }

        impl DataFrame for $name{
            /// internal function to define the first row of the csv
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str]
                    = &[$(stringify!($input),)* $(stringify!($input_vec),)*];
                NAMES
            }

            /// internal function to print all the aggregate values
            fn to_string(&self) -> Vec<String> {
                let mut v: Vec<String> = Vec::new();
                $(
                    v.push(format!("{:?}", self.$input));
                )*
                $(
                    v.push(format!("{:?}", self.$input_vec));
                )*
                v
            }

        }


        /// Public for the structure
        impl $name {
            /// create a new instance of the custom structure
            pub fn new(
                $($input: $input_ty,)* $($input_vec: [$input_ty_vec; $input_len],)*
            ) -> $name{
                $name {
                    $(
                        $input,
                    )*
                    $(
                        $input_vec,
                    )*
                }
            }
        }

    };

    // Internal function for automatic building the structure for the Dataframe
    //
    // The dataframe allow to write the data of the simulations into a comfort structure that can be saved inside a file or easily manipulated
    //
    // This pattern cover the case when no vector are passed by in the macro
    //
    // name : custom name of the structure
    //
    // input : pair of identifier and type
    //
    // derive : optional parameter for the derive directive
    (
        $name:ident,
        input {$($input:ident: $input_ty:ty)*}
        $($derive: tt)*
    ) => {
        build_dataframe_explore!(
            $name,
            input {$($input: $input_ty)*},
            vec { }
            $($derive)*
        );
    };


    // Internal function for automatic building the structure for the Dataframe
    //
    // The dataframe allow to write the data of the simulations into a comfort structure that can be saved inside a file or easily manipulated
    //
    // This pattern cover the case when only vectors are passed by in the macro
    //
    // name : custom name of the structure
    //
    // vec : vector of elements, must specify the identifier, the type and the vector length
    //
    // derive : optional parameter for the derive directive
    (
        $name:ident,
        vec {$($input_vec:ident: [$input_ty_vec:ty; $input_len:expr])*}
        $($derive: tt)*
    ) => {
        build_dataframe_explore!(
            $name,
            input { },
            vec {$($input_vec: [$input_ty_vec; $input_len])*}
            $($derive)*
        );
    };

}

#[macro_export]
/// Macro to optimizes the simulation parameters by adopting an evolutionary
/// searching strategy. Specifically, krABMaga provides a genetic algorithm-based approach.
/// The macro will generate a dataframe with the results of the exploration.
/// The dataframe can be saved in a csv file.
///
/// # Arguments
///
/// * `init_population` - function that creates the population, must return an array of individual (an individual is the state of the simulation to compute)
/// * `fitness` - function that computes the fitness value, takes a single individual and the schedule, must return an f32
/// * `mutation` - function that perform the mutation, takes a single individual as parameter
/// * `crossover` - function that creates the population, takes the entire population as parameter
/// * `state` - state of the simulation representing an individual
/// * `desired_fitness` - desired fitness value
/// * `generation_num` - max number of generations to compute
/// * `step` - number of steps of the single simulation
/// * `reps` - number of repetitions of the simulation using each individual (optional, default is 1)
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
/// ```rust
/// pub const STEP: u64 = 100;
/// pub const REPETITIONS: u32 = 20;
///
/// pub const DESIRED_FITNESS: f32 = 0.;
/// pub const MAX_GENERATION: u32 = 2_000;
///
/// fn main() {
///     let result = explore_ga_sequential!(
///         init_population,
///         fitness,
///         selection,
///         mutation,
///         crossover,
///         cmp,
///         State,
///         DESIRED_FITNESS,
///         MAX_GENERATION,
///         STEP,
///         REPETITIONS, //optional
///         // ComputingMode::Parallel, ComputingMode::Distributed or ComputingMode::Cloud
///     );
///     if !result.is_empty() {
///         let name = "explore_result".to_string();
///         let _res = write_csv(&name, &result);
///     }
/// }
///
/// // Create the initial population. In genetic algorithms, an individual is represented as a String
/// fn init_population() -> Vec<String> { ... }
///
/// // Compute the fitness value of an individual using results of each repetition
/// // * computed_ind: Vec with couple of (state, fitness)
/// //   of an individual for each repetition in the current generation
/// fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 { ... }
///
/// // Select/Order the population based on the fitness value
/// fn selection(population_fitness: &mut Vec<(String, f32)>) { ... }
///
/// // Perform the mutation of an individual
/// fn mutation(individual: &mut String) { ... }
///
/// // Perform the crossover to generate the new population
/// fn crossover(population: &mut Vec<String>) { ... }
///
/// // Compare two individuals
/// fn cmp(fitness1: &f32, fitness2: &f32) -> bool { ... }
///
/// ```
///
macro_rules! evolutionary_search {
    (
        $init_population:tt,
        $fitness:tt,
        $selection:tt,
        $mutation:tt,
        $crossover:tt,
        $cmp:tt,
        $state: ty,
        $desired_fitness: expr,
        $generation_num: expr,
        $step: expr,
        $($reps: expr,)?
        ComputingMode::$computing_mode: tt
    ) => {{
        use $crate::cfg_if::cfg_if;
        use $crate::engine::schedule::Schedule;
        use $crate::engine::state::State;
        use $crate::ComputingMode;

        let cp_mode = ComputingMode::$computing_mode;
        match cp_mode {
            ComputingMode::Parallel => {
                cfg_if!{
                    if #[cfg(not(any(feature = "distributed_mpi", feature = "cloud")))] {
                        println!("Parallel exploration");
                        explore_ga_parallel!(
                            $init_population,
                            $fitness,
                            $selection,
                            $mutation,
                            $crossover,
                            $cmp,
                            $state,
                            $desired_fitness,
                            $generation_num,
                            $step,
                            $($reps,)?
                        )
                    } else {
                        panic!("Parallel computing mode doesn't require distributed or cloud features");
                    }
                }
            },
            ComputingMode::Distributed => {
                cfg_if!{
                    if #[cfg(feature = "distributed_mpi")] {
                        explore_ga_distributed_mpi!(
                            $init_population,
                            $fitness,
                            $selection,
                            $mutation,
                            $crossover,
                            $cmp,
                            $state,
                            $desired_fitness,
                            $generation_num,
                            $step,
                            $($reps,)?
                        )
                    } else {
                        panic!("Distributed computing mode requires distributed_mpi feature");
                    }
                }
            },
            ComputingMode::Cloud => {
                cfg_if!{
                    if #[cfg(feature="aws")] {
                        println!("Cloud GA exploration with AWS");
                        println!("WARNING: this mode is not yet implemented");
                    }
                    else {
                        panic!("Cloud computing mode is not available. Please enable the feature 'aws' to use this mode.");
                    }
                }
            },
        }
    }};

    (
        $init_population:tt,
        $fitness:tt,
        $selection:tt,
        $mutation:tt,
        $crossover:tt,
        $cmp:tt,
        $state: ty,
        $desired_fitness: expr,
        $generation_num: expr,
        $step: expr,
        $($reps: expr)?
    ) => {{

        use $crate::engine::schedule::Schedule;
        use $crate::engine::state::State;

        explore_ga_sequential!(
            $init_population,
            $fitness,
            $selection,
            $mutation,
            $crossover,
            $cmp,
            $state,
            $desired_fitness,
            $generation_num,
            $step,
            $($reps,)?
        )
    }}
}

/// Macro to perform sequential model exploration using a genetic algorithm.
///
/// # Arguments
///
/// # Arguments
/// * `init_population` - function that creates the population, must return an array of individual. An individual is the state of the simulation to compute
/// * `fitness` - function that computes the fitness value, takes a single individual and the schedule, must return an f32
/// * `selection` - function that select pair of individuals with some criterion and create a new individual for the next generation
/// * `mutation` - function that perform the mutation, takes a single individual as parameter
/// * `crossover` - function that creates the population, takes the entire population as parameter
/// * `cmp` - function that compare two individuals, takes two individuals as parameter and return true if the first is better than the second
/// * `state` - state of the simulation representing an individual
/// * `desired_fitness` - desired fitness value
/// * `generation_num` - max number of generations to compute
/// * `step` - number of steps of the single simulation
/// * `reps` - optional values for number of repetitions
///
/// # Example
///
/// ```rust
/// pub const STEP: u64 = 100;
/// pub const REPETITIONS: u32 = 20;
///
/// pub const DESIRED_FITNESS: f32 = 0.;
/// pub const MAX_GENERATION: u32 = 2_000;
///
/// fn main() {
///     let result = explore_ga_sequential!(
///         init_population,
///         fitness,
///         selection,
///         mutation,
///         crossover,
///         cmp,
///         State,
///         DESIRED_FITNESS,
///         MAX_GENERATION,
///         STEP,
///         REPETITIONS,
///     );
///     if !result.is_empty() {
///         let name = "explore_result".to_string();
///         let _res = write_csv(&name, &result);
///     }
/// }
///
/// // Create the initial population. In genetic algorithms, an individual is represented as a String
/// fn init_population() -> Vec<String> { ... }
///
/// // Compute the fitness value of an individual using results of each repetition
/// // * computed_ind: Vec with couple of (state, fitness)
/// //   of an individual for each repetition in the current generation
/// fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 { ... }
///
/// // Select/Order the population based on the fitness value
/// fn selection(population_fitness: &mut Vec<(String, f32)>) { ... }
///
/// // Perform the mutation of an individual
/// fn mutation(individual: &mut String) { ... }
///
/// // Perform the crossover to generate the new population
/// fn crossover(population: &mut Vec<String>) { ... }
///
/// // Compare two individuals
/// fn cmp(fitness1: &f32, fitness2: &f32) -> bool { ... }
///
/// ```
///
#[macro_export]
macro_rules! explore_ga_sequential {
    (
        $init_population:tt,
        $fitness:tt,
        $selection:tt,
        $mutation:tt,
        $crossover:tt,
        $cmp:tt,
        $state: ty,
        $desired_fitness: expr,
        $generation_num: expr,
        $step: expr,
        $($reps: expr,)?
    ) => {{
        println!("Running sequential GA exploration...");
        let start = Instant::now();

        build_dataframe_explore!(BufferGA, input {
            generation: u32
            index: i32
            fitness: f32
            individual: String
        });

        let mut reps = 1;
        $(reps = $reps;)?

        let mut generation = 0;
        let mut best_fitness: Option<f32> = None;
        let mut best_generation = 0;

        let mut result: Vec<BufferGA> = Vec::new();

        // use init_population custom function to create a vector of state
        let mut population: Vec<String> = $init_population();
        let mut pop_fitness: Vec<(String, f32)> = Vec::new();

        // flag to break from the loop
        let mut flag = false;
        let mut best_individual: String = String::new();

        // calculate the fitness for the first population
        loop {

            // if generation_num is passed as 0, we have infinite generations
            if $generation_num != 0 && generation == $generation_num {
                println!("Reached {} generations, exiting...", $generation_num);
                break;
            }
            generation += 1;
            println!("Computing generation {}...", generation);

            let mut best_fitness_gen: Option<f32> = None;
            let mut best_individual_gen: String = String::new();

            // execute the simulation for each member of population
            // iterates through the population
            let mut index = 0;

            for individual in population.iter_mut() {

                let mut computed_ind: Vec<($state, Schedule)> = Vec::new();

                for r in 0..(reps as usize){
                    // initialize the state
                    let mut individual_state = <$state>::new_with_parameters(r, individual);
                    let mut schedule: Schedule = Schedule::new();
                    individual_state.init(&mut schedule);
                    // compute the simulation
                    for _ in 0..($step as usize) {
                        let individual_state = individual_state.as_state_mut();
                        schedule.step(individual_state);
                        if individual_state.end_condition(&mut schedule) {
                            break;
                        }
                    }
                    computed_ind.push((individual_state, schedule));
                }

                // compute the fitness value
                let fitness = $fitness(&mut computed_ind);
                pop_fitness.push((individual.clone(), fitness));

                // saving the best fitness of this generation
                // if fitness >= best_fitness_gen {
                match best_fitness_gen{
                    Some(_) =>
                        if $cmp(&fitness, &best_fitness_gen.expect("Error reading best fitness gen")) {
                            best_fitness_gen = Some(fitness);
                            best_individual_gen = individual.clone();
                        },
                    None => {
                        best_fitness_gen = Some(fitness);
                        best_individual_gen = individual.clone();
                    }
                }
                // result is here
                result.push(BufferGA::new(
                    generation,
                    index,
                    fitness,
                    individual.clone()
                ));

                // if the desired fitness is reached break
                // setting the flag at true
                // if fitness >= $desired_fitness{
                if $cmp(&fitness, &$desired_fitness) {
                    println!("Found individual with desired fitness! Exiting...");
                    flag = true;
                    break;
                }
                index += 1;
            }

            // saving the best fitness of all generation computed until n
            // if best_fitness_gen > best_fitness {

            match best_fitness{
                Some(_) =>
                    if $cmp(&best_fitness_gen.expect("Error reading best fitness gen"), &best_fitness.expect("Error reading best fitness")) {
                        best_fitness = best_fitness_gen;
                        best_individual = best_individual_gen.clone();
                        best_generation = generation;
                    },
                None => {
                    best_fitness = best_fitness_gen;
                    best_individual = best_individual_gen.clone();
                    best_generation = generation;
                }
            }

            let elapsed_time = start.elapsed();
            println!("*** Completed generation {} after {} seconds ***", generation, elapsed_time.as_secs_f32());
            println!("- Best fitness in generation {} is {:#?} using {:#?}", generation, best_fitness_gen.expect("Error reading best fitness gen"), best_individual_gen);
            println!("-- Overall best fitness is found in generation {} and is {:#?} using {:#?}", best_generation, best_fitness.expect("Error reading best fitness"), best_individual);

            // if flag is true the desired fitness is found
            if flag {
                break;
            }

            // compute selection
            $selection(&mut pop_fitness);

            // check if after selection the population size is too small
            if pop_fitness.len() <= 1 {
                println!("Population size <= 1, exiting...");
                break;
            }

            // mutate the new population
            population.clear();
            for (individual, _) in pop_fitness.iter_mut() {
                population.push(individual.clone());
            }
            pop_fitness.clear();

            // crossover the new population
            $crossover(&mut population);

            for i in 0..population.len() {
                $mutation(&mut population[i]);
            }
        }

        println!("Resulting best fitness is {}", best_fitness.expect("Error reading best fitness"));
        println!("- The best individual is: \n\t{}", best_individual);

        result
    }};

}

/// Macro to perform sequential model exploration using a genetic algorithm.
///
/// # Arguments
///
/// # Arguments
/// * `init_population` - function that creates the population, must return an array of individual. An individual is the state of the simulation to compute
/// * `fitness` - function that computes the fitness value, takes a single individual and the schedule, must return an f32
/// * `selection` - function that select pair of individuals with some criterion and create a new individual for the next generation
/// * `mutation` - function that perform the mutation, takes a single individual as parameter
/// * `crossover` - function that creates the population, takes the entire population as parameter
/// * `cmp` - function that compare two individuals, takes two individuals as parameter and return true if the first is better than the second
/// * `state` - state of the simulation representing an individual
/// * `desired_fitness` - desired fitness value
/// * `generation_num` - max number of generations to compute
/// * `step` - number of steps of the single simulation
/// * `reps` - optional values for number of repetitions
///
/// # Example
///
/// ```rust
/// pub const STEP: u64 = 100;
/// pub const REPETITIONS: u32 = 20;
///
/// pub const DESIRED_FITNESS: f32 = 0.;
/// pub const MAX_GENERATION: u32 = 2_000;
///
/// fn main() {
///     let result = explore_ga_parallel!(
///         init_population,
///         fitness,
///         selection,
///         mutation,
///         crossover,
///         cmp,
///         State,
///         DESIRED_FITNESS,
///         MAX_GENERATION,
///         STEP,
///         REPETITIONS,
///     );
///     if !result.is_empty() {
///         let name = "explore_result".to_string();
///         let _res = write_csv(&name, &result);
///     }
/// }
///
/// // Create the initial population. In genetic algorithms, an individual is represented as a String
/// fn init_population() -> Vec<String> { ... }
///
/// // Compute the fitness value of an individual using results of each repetition
/// // * computed_ind: Vec with couple of (state, fitness)
/// //   of an individual for each repetition in the current generation
/// fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 { ... }
///
/// // Select/Order the population based on the fitness value
/// fn selection(population_fitness: &mut Vec<(String, f32)>) { ... }
///
/// // Perform the mutation of an individual
/// fn mutation(individual: &mut String) { ... }
///
/// // Perform the crossover to generate the new population
/// fn crossover(population: &mut Vec<String>) { ... }
///
/// // Compare two individuals
/// fn cmp(fitness1: &f32, fitness2: &f32) -> bool { ... }
///
/// ```
///
#[macro_export]
macro_rules! explore_ga_parallel {
    (
        $init_population:tt,
        $fitness:tt,
        $selection:tt,
        $mutation:tt,
        $crossover:tt,
        $cmp: tt,
        $state: ty,
        $desired_fitness: expr,
        $generation_num: expr,
        $step: expr,
        $($reps: expr,)?
    ) => {{
        println!("Running parallel GA exploration...");
        let start = Instant::now();

        build_dataframe_explore!(BufferGA, input {
            generation: u32
            index: i32
            fitness: f32
            individual: String
            state: String
        });

        let mut reps = 1;
        $( reps = $reps;)?

        let mut generation = 0;
        let mut best_fitness: Option<f32> = None;
        let mut best_generation = 0;

        // use init_population custom function to create a vector of individual
        let mut population: Vec<String> = $init_population();
        let mut pop_fitness: Vec<(String, f32)> = Vec::new();
        let mut best_individual: String = String::new();

        // flag to break from the loop
        let mut flag = false;

        // Wrap the population into a Mutex to be safely shared
        let population = Arc::new(Mutex::new(population));
        let mut res: Vec<BufferGA> = Vec::new();

        // calculate the fitness for the first population
        loop {

            // if generation_num is passed as 0, we have infinite generations
            if $generation_num != 0 && generation == $generation_num {
                println!("Reached {} generations, exiting...", $generation_num);
                break;
            }
            generation += 1;
            println!("Computing generation {}...", generation);

            let mut best_fitness_gen: Option<f32> = None;
            let mut best_individual_gen: String = String::new();

            let mut len = population.lock().expect("Error in population lock acquisition").len();

            let mut result = Vec::new();
            // execute the simulation for each member of population
            // iterates through the population

            //todo change 0..len into population.iter()
            // to remove lock on population
            (0..len).into_par_iter().map( |index| {
                let mut computed_ind: Vec<($state, Schedule)> = Vec::new();

                let mut save_state: String = String::new();

                for r in 0..(reps as usize){
                    // initialize the state
                    let mut schedule: Schedule = Schedule::new();
                    let mut individual: $state;
                    {
                        let mut population = population.lock().expect("Error in population lock acquisition");
                        // create the new state using the parameters
                        individual = <$state>::new_with_parameters(r, &population[index]);
                    }

                    // state initialization
                    individual.init(&mut schedule);
                    // simulation computation
                    for _ in 0..($step as usize) {
                        let individual = individual.as_state_mut();
                        schedule.step(individual);
                        if individual.end_condition(&mut schedule) {
                            break;
                        }
                    }
                    save_state = format!("{}", individual);
                    computed_ind.push((individual, schedule));
                }

                // compute the fitness value
                let fitness = $fitness(&mut computed_ind);

                let mut population = population.lock().expect("Error in population lock acquisition");

                BufferGA::new(
                    generation,
                    index as i32,
                    fitness,
                    population[index].clone(),
                    save_state
                )

                // return an array containing the results of the simulation to be written in the csv file
            }).collect_into_vec(&mut result);

            // for each simulation result
            for i in 0..result.len() {

                let fitness = result[i].fitness;
                let individual = result[i].individual.clone();

                pop_fitness.push((individual.clone(), fitness));

                // saving the best fitness of this generation
                // if fitness >= best_fitness_gen {
                match best_fitness_gen {
                    Some(_) =>
                        if $cmp(&fitness, &best_fitness_gen.expect("Error reading best fitness gen")) {
                            best_fitness_gen = Some(fitness);
                            best_individual_gen = individual.clone();
                        },
                    None => {
                        best_fitness_gen = Some(fitness);
                        best_individual_gen = individual.clone();
                    }
                }

                // if the desired fitness set the flag at true
                // if fitness >= $desired_fitness {
                if $cmp(&fitness, &$desired_fitness) {
                    println!("Found individual with desired fitness! Exiting...");
                    flag = true;
                }
            }

            // saving the best fitness of all generation computed until now
            // if best_fitness_gen > best_fitness {
            match best_fitness {
                Some(_) =>
                    if $cmp(&best_fitness_gen.expect("Error reading best fitness gen"), &best_fitness.expect("Error reading best fitness")) {
                        best_fitness = best_fitness_gen.clone();
                        best_individual = best_individual_gen.clone();
                        best_generation = generation;
                    },
                None => {
                    best_fitness = best_fitness_gen.clone();
                    best_individual = best_individual_gen.clone();
                    best_generation = generation;
                }
            }

            let elapsed_time = start.elapsed();
            println!("*** Completed generation {} after {} seconds ***", generation, elapsed_time.as_secs_f32());
            println!("- Best fitness in generation {} is {:#?} using {:#?}", generation, best_fitness_gen.expect("Error reading best fitness gen"), best_individual_gen);
            println!("-- Overall best fitness is found in generation {} and is {:#?} using {:#?}", best_generation, best_fitness.expect("Error reading best fitness"), best_individual);

            res.append(&mut result);

            // if flag is true the desired fitness is found
            if flag {
                break;
            }

            // compute selection
            $selection(&mut pop_fitness);

            // check if after selection the population size is too small
            if pop_fitness.len() < 1 {
                println!("Population size < 1, exiting...");
                break;
            }

            {
                let mut population = population.lock().expect("Error in population lock acquisition");
                population.clear();
                for (individual, _) in pop_fitness.iter_mut() {
                    population.push(individual.clone())
                }
                pop_fitness.clear();

                // crossover the new population
                $crossover(&mut population);
                // mutate the new population
                for i in 0..population.len() {
                    $mutation(&mut population[i]);
                }
            }
        }

        println!("Resulting best fitness is {:#?}", best_fitness.expect("Error reading best fitness"));
        println!("- The best individual is:\n\t{}", best_individual);

        res

    }};

}
