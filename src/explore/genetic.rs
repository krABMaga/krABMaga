#[macro_export]
macro_rules! build_dataframe_explore {
    //Dataframe with input and output parameters and optional parameters
    (
        $name:ident,
        input {$($input:ident: $input_ty:ty)*},
        vec {$($input_vec:ident: [$input_ty_vec:ty; $input_len:expr])*}
        $($derive: tt)*
    ) => {

        #[derive(Clone, Debug,  $($derive,)*)]
        struct $name {
            $(pub $input: $input_ty,)*
            $(pub $input_vec: [$input_ty_vec; $input_len],)*
        }

        impl DataFrame for $name{
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str]
                    = &[$(stringify!($input),)* $(stringify!($input_vec),)*];
                NAMES
            }

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


        // new of BufferGA
        impl $name {
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

    //only input
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


    //only vec
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

// macro to perform sequential model exploration using a genetic algorithm
// an individual is the state of the simulation to compute
// init_population: function that creates the population, must return an array of individual
// fitness: function that computes the fitness value, takes a single individual and the schedule, must return an f32
// mutation: function that perform the mutation, takes a single individual as parameter
// crossover: function that creates the population, takes the entire population as parameter
// state: state of the simulation representing an individual
// desired_fitness: desired fitness value
// generation_num: max number of generations to compute
// step: number of steps of the single simulation
// reps: number of repetitions of the simulation using each individual
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
                        if $cmp(&fitness, &best_fitness_gen.unwrap()) {
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
                    if $cmp(&best_fitness_gen.unwrap(), &best_fitness.unwrap()) {
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

            println!("- Best fitness in generation {} is {:#?} using {:#?}", generation, best_fitness_gen.unwrap(), best_individual_gen);
            println!("-- Overall best fitness is found in generation {} and is {:#?} using {:#?}", best_generation, best_fitness.unwrap(), best_individual);

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

        println!("Resulting best fitness is {}", best_fitness.unwrap());
        println!("- The best individual is: \n\t{}", best_individual);

        result
    }};

}

// macro to perform parallel model exploration using a genetic algorithm
// an individual is the state of the simulation to compute
// init_population: function that creates the population, must return an array of individual
// fitness: function that computes the fitness value, takes a single individual and the schedule, must return an f32
// mutation: function that perform the mutation, takes a single individual as parameter
// crossover: function that creates the population, takes the entire population as parameter
// state: state of the simulation representing an individual
// desired_fitness: desired fitness value
// generation_num: max number of generations to compute
// step: number of steps of the single simulation
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

            let mut len = population.lock().unwrap().len();

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
                        let mut population = population.lock().unwrap();
                        // create the new state using the parameters
                        individual = <$state>::new_with_parameters(r, &population[index]);
                    }

                    // state initilization
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

                let mut population = population.lock().unwrap();

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
                        if $cmp(&fitness, &best_fitness_gen.unwrap()) {
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
                    if $cmp(&best_fitness_gen.unwrap(), &best_fitness.unwrap()) {
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

            println!("- Best fitness in generation {} is {:#?} using {:#?}", generation, best_fitness_gen.unwrap(), best_individual_gen);
            println!("-- Overall best fitness is found in generation {} and is {:#?} using {:#?}", best_generation, best_fitness.unwrap(), best_individual);

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
                let mut population = population.lock().unwrap();
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

        println!("Resulting best fitness is {:#?}", best_fitness.unwrap());
        println!("- The best individual is:\n\t{}", best_individual);

        res

    }};

}
