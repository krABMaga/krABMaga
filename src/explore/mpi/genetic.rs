#[doc(hidden)]
#[macro_export]
/// internal macro to extend the dataframe to standardize the structure for MPI protocol
///
/// name: identifier of the custom structure
///
/// input: all the input fields of the structure
macro_rules! extend_dataframe_explore {
    //implementation of a trait to send/receive with mpi
    ($name:ident,
    input {$($input:ident: $input_ty:ty)*}
  //  vec {$($input_vec:ident: [$input_ty_vec:ty; $input_len: expr])*}
    ) => {
        unsafe impl Equivalence for $name {
            type Out = UserDatatype;
            fn equivalent_datatype() -> Self::Out {

                //count input and output parameters to create slice for block length
                let v_in = count_tts!($($input)*);

                let mut vec = Vec::with_capacity(v_in);
                for i in 0..(v_in){
                    vec.push(1);
                }

                UserDatatype::structured(
                    vec.as_slice(),
                    &[
                        $(
                            offset_of!($name, $input) as Address,
                        )*
                    ],
                    &[
                        $(
                            <$input_ty>::equivalent_datatype(),
                        )*
                    ]
                )
            }
        }
    };
}

/// Macro to perform distributed model exploration using a genetic algorithm based on MPI
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
///
///
///
/// # Example
/// To run the example, you need to have installed the mpi library and include the mpi feature in the Cargo.toml file
/// ```toml
/// [features]
/// distributed_mpi = ["krabmaga/distributed_mpi"]
/// ```
///
/// ```rust
/// pub const STEP: u64 = 100;
/// pub const REPETITIONS: u32 = 20;
///
/// pub const DESIRED_FITNESS: f32 = 0.;
/// pub const MAX_GENERATION: u32 = 2_000;
///
/// fn main() {
///     let result = explore_ga_distributed_mpi!(
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
///         // Master process save the results
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
macro_rules! explore_ga_distributed_mpi {
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

        // MPI initialization
        let world = UNIVERSE.world();
        let root_rank = 0;
        let root_process = world.process_at_rank(root_rank);
        let my_rank = world.rank();
        let num_procs = world.size() as usize;
        let start_time = Instant::now();
        if world.rank() == root_rank {
            println!("Running distributed (MPI) GA exploration...");
        }

        let mut reps = 1;
        $(reps = $reps;)?
        let mut generation: u32 = 0;
        let mut best_fitness: Option<f32> = None;
        let mut best_generation = 0;
        let mut my_pop_size: usize = 0;
        let mut population: Vec<String> = Vec::new();
        let mut population_size = 0;

        //definition of a dataframe called BufferGA
        build_dataframe_explore!(BufferGA,
            input {
                generation: u32
                index: i32
                fitness: f32
            }
        );

        //implement trait for BufferGA to send/receive with mpi
        extend_dataframe_explore!(BufferGA,
            input {
                generation: u32
                index: i32
                fitness: f32
            }
        );

        let mut population_params: Vec<String> = Vec::new();

        // only master modifies these four variables
        let mut master_fitness;
        let mut master_index;
        let mut master_individual;
        let mut best_individual_string = String::new();

        let mut best_individual: Option<BufferGA> = None;
        let mut pop_fitness: Vec<(String, f32)> = Vec::new();
        let mut all_results: Vec<BufferGA> = Vec::new();

        // my best for each proc through generations
        let mut my_best_fitness: Option<f32> = None;
        let mut my_best_index: i32 = 0;
        let mut my_best_individual = String::new();

        //becomes true when the algorithm get desider fitness
        let mut flag = false;

        if world.rank() == root_rank {
            population = $init_population();
            population_size = population.len();

            // dummy initialization

            /*
                if fitness desired has to be minimum, set fitness to a very high value
            */
            best_individual = Some(BufferGA::new(
                0,
                0,
                1000.
            ));
        }

        loop {

            if $generation_num != 0 && generation == $generation_num {
                if world.rank() == root_rank {
                    println!("Reached {} generations, exiting...", $generation_num);
                }
                break;
            }

            if flag {
                if world.rank() == root_rank {
                    println!("Reached best fitness on generation {}, exiting...", generation);
                }
                break;
            }

            generation += 1;

            if world.rank() == root_rank {
                println!("Running Generation {}...", generation);
            }

            let mut samples_count: Vec<Count> = Vec::new();

            // only the root process split the workload among the processes
            if world.rank() == root_rank {
                //create the whole population and send it to the other processes
                let mut population_size_per_process = population_size / num_procs;
                let mut remainder = population_size % num_procs;
                let mut index = 0;
                let mut send_index = 0;
                // for each processor
                for i in 0..num_procs {
                    let mut sub_population_size = 0;

                    // calculate the workload subdivision
                    if remainder > 0 {
                        sub_population_size = population_size_per_process + 1;
                        remainder -= 1;
                    } else {
                        sub_population_size = population_size_per_process;
                    }
                    samples_count.push(sub_population_size as Count);

                    // save my_pop_size for master
                    if i == 0 {
                        my_pop_size = sub_population_size;
                    }

                    // fulfill the parameters arrays
                    for j in 0..sub_population_size {
                        //let index = i * population_size_per_process + j;
                        //println!("i: {} - popsize {} - j {}", i, population_size_per_process, j);
                        population_params.push(population[index].clone());
                        index += 1;
                    }
                    //println!("for rank {} population_params: {:?}",i, population_params);

                    // send the arrays
                    world.process_at_rank(i as i32).send(&sub_population_size);
                    let mut to_send: Vec<BufferGA> = Vec::new();

                    // for p in 0..sub_population_size {
                    //     world.process_at_rank(i as i32).send(&population_params[i * population_size_per_process + p].clone().as_bytes()[..]);
                    // }
                    for p in 0..sub_population_size {
                        world.process_at_rank(i as i32).send(&population_params[send_index].clone().as_bytes()[..]);
                        send_index += 1;
                    }

                    //population_params.clear();
                }
            } else {
                // every other processor receive the parameter
                let (my_population_size, _) = world.any_process().receive::<usize>();
                my_pop_size = my_population_size;

                for i in 0..my_pop_size {
                    let (param, _) = world.any_process().receive_vec::<u8>();
                    let my_param = String::from_utf8(param).expect("Error: can't convert parameter as string");
                    population_params.push(my_param);

                }
            }
            // computed_ind
            let mut my_population: Vec<String> = Vec::new();
            if world.rank() == root_rank {

                for i in 0..my_pop_size {
                    my_population.push(population[i].clone());
                }

                //println!("master pop: {:?}", my_population);
            } else {
                //init local sub-population
                for i in 0..my_pop_size {

                    my_population.push(population_params[i].clone());
                }
            }

            let mut best_fitness_gen: Option<f32> = None;
            let mut local_index = 0;
            // array collecting the results of each simulation run
            let mut my_results: Vec<BufferGA> = Vec::new();

            // counter for master to stop at population size
            let mut master_counter = 0;

            for individual_params in my_population.iter_mut() {
                if my_rank == 0 && (master_counter == my_pop_size) {
                    break;
                }

                // initialize the state
                let mut computed_ind: Vec<($state, Schedule)> = Vec::new();

                for r in 0..(reps as usize){
                    let mut individual = <$state>::new_with_parameters(r, &individual_params);
                    let mut schedule: Schedule = Schedule::new();
                    individual.init(&mut schedule);
                    // compute the simulation
                    for _ in 0..($step as usize) {
                        let individual = individual.as_state_mut();
                        schedule.step(individual);
                        if individual.end_condition(&mut schedule) {
                            break;
                        }
                    }
                    computed_ind.push((individual, schedule));
                }

                // compute the fitness value
                let fitness = $fitness(&mut computed_ind);

                // if fitness > my_best_fitness {
                //println!("rank {} --- {}", my_rank, fitness);

                match my_best_fitness {
                    Some(_) =>
                        if $cmp(&fitness, &my_best_fitness.expect("265")){
                            my_best_fitness = Some(fitness);
                            my_best_index = local_index;
                            my_best_individual = individual_params.clone();
                        },
                    None => {
                        my_best_fitness = Some(fitness);
                        my_best_index = local_index;
                        my_best_individual = individual_params.clone();
                    }
                }

                let result = BufferGA::new(
                    generation,
                    local_index,
                    fitness,
                );

                my_results.push(result);

                if $cmp(&fitness, &$desired_fitness) {
                    flag = true;
                }

                local_index += 1;

                if my_rank == 0 {
                    master_counter += 1;
                }
            }

            //best individual sent from each proc
            if world.rank() == root_rank {
                master_fitness = vec![0f32; num_procs];
                master_index= vec![0i32; num_procs];
                master_individual = Vec::with_capacity(num_procs);
                // gather_into remap mpi_gather so we memorize values in rank order
                root_process.gather_into_root(&my_best_fitness.expect("309"), &mut master_fitness[..]);
                root_process.gather_into_root(&my_best_index, &mut master_index[..]);
                for i in 0..num_procs {
                    if i == 0 {
                        master_individual.push(my_best_individual.clone());
                    } else {
                        let (param, _) = world.process_at_rank(i as i32).receive_vec::<u8>();
                        let my_param = String::from_utf8(param).expect("Error: can't convert parameter as string");
                        master_individual.push(my_param);
                    }
                }
                // get the index of max fitness from all_bests_fitness
                let mut max_fitness_index = 0;
                for i in 1..num_procs {
                    if $cmp(&master_fitness[i], &master_fitness[max_fitness_index]) {
                        max_fitness_index = i;
                    }
                }

                best_individual_string = master_individual[max_fitness_index].clone();
                //master_fitness.clear();
            } else {
                root_process.gather_into(&my_best_fitness.expect("336"));
                root_process.gather_into(&my_best_index);
                root_process.send(&my_best_individual.clone().as_bytes()[..]);
            }

            // receive simulations results from each processors
            if world.rank() == root_rank {

                // dummy initialization
                let dummy = BufferGA::new(
                    generation,
                    0,
                    1000.,
                );

                let displs: Vec<Count> = samples_count
                    .iter()
                    .scan(0, |acc, &x| {
                        let tmp = *acc;
                        *acc += x;
                        Some(tmp)
                    })
                    .collect();

                let mut partial_results = vec![dummy; population_size];
                let mut partition = PartitionMut::new(&mut partial_results[..], samples_count.clone(), &displs[..]);
                // root receives all results from other processors
                root_process.gather_varcount_into_root(&my_results[..], &mut partition);

                best_fitness_gen = None;
                // save the best individual of this generation
                let mut i = 0;
                let mut j = 0;
                for elem in partial_results.iter_mut() {
                    // only the master can update the index
                    elem.index += displs[i];

                    // if elem.fitness > best_fitness_gen{
                    //     best_fitness_gen = elem.fitness;
                    // }

                    match best_fitness_gen {
                        Some(_) => {
                            if $cmp(&elem.fitness, &best_fitness_gen.expect("379")) {
                                best_fitness_gen = Some(elem.fitness);
                            }
                        },
                        None => {
                            best_fitness_gen = Some(elem.fitness);
                        }
                    }

                    match best_individual {
                        Some(_) => {
                            if $cmp(&elem.fitness, &best_individual.clone().expect("Error: can't read best individual").fitness) {
                                //println!("----- {}", elem.fitness);
                                best_individual = Some(elem.clone());
                            }
                        },
                        None => {
                            best_individual = Some(elem.clone());
                        }
                    }

                    j += 1;
                    if j == samples_count[i]{
                        i += 1;
                        j = 0;
                    }
                }


                for elem in partial_results.iter() {
                    if elem.fitness == 1000. {
                        panic!("partial_results contains dummy");
                    }
                }

                // combine the results received
                all_results.append(&mut partial_results);
            } else {
                // send the result to the root processor
                root_process.gather_varcount_into(&my_results[..]);
            }

            // saving the best fitness of all generation computed until n
            // if best_fitness_gen > best_fitness {
            //     best_fitness = best_fitness_gen;
            //     best_generation = generation;
            // }

            if world.rank() == root_rank {

                match best_fitness {
                    Some(mut x) => {
                        // println!("best fitness before: {}", x);
                        if $cmp(&best_fitness_gen.expect("416111"), &best_fitness.expect("416")) {
                            best_fitness = Some(best_fitness_gen.expect("417"));
                            best_generation = generation;
                            // println!("here 1");
                        }
                        // println!("best fitness after: {}", x);

                    },
                    None => {
                        best_fitness = Some(best_fitness_gen.expect("422"));
                        best_generation = generation;
                        // println!("here 2");

                    }
                }

            }

            if world.rank() == root_rank{
                let elapsed_time = start_time.elapsed();
                println!("*** Completed generation {} after {} seconds ***", generation, elapsed_time.as_secs_f32());
                println!("- Best fitness in generation {} is {}", generation, best_fitness_gen.expect("Error: can't read best fitness gen"));
                println!("-- Overall best fitness is found in generation {} and is {}", best_generation, best_fitness.expect("Error: can't read best fitness"));
            }

            // if flag is true the desired fitness is found
            // and the master warns the other procs to exit
            // gather a vec of flag because we don't know which proc has set the flag to true
            if world.rank() == root_rank {
                let mut all_flags = vec![false; num_procs];
                root_process.gather_into_root(&flag, &mut all_flags[..]);

                if all_flags.contains(&true) {
                    flag = true;
                }
            } else {
                root_process.gather_into(&flag);
            }

            //master process sends the flag to the other procs
            // if the flag is true all process will exit
            root_process.broadcast_into(&mut flag);
            if flag {
                //print!("flag true");
                break;
            }

            // the master do selection, mutation and crossover
            if world.rank() == root_rank {

                // set the population parameters owned by the master
                // using the ones received from other processors
                for i in 0..population_size {
                    let index = (generation as usize -1)*population_size + i;
                    let fitness = all_results[index].fitness;
                    let individual = population_params[i].clone();
                    let tup = (individual, fitness);
                    pop_fitness.push(tup);
                }

                // compute selection
                $selection(&mut pop_fitness);

                // check if after selection the population size is too small
                if pop_fitness.len() <= 1 {
                    println!("Population size <= 1, exiting...");
                    break;
                }

                population.clear();

                // mutate the new population
                for (individual, _) in pop_fitness.iter_mut() {
                    population.push(individual.clone());
                }
                pop_fitness.clear();

                // crossover the new population
                $crossover(&mut population);

                // Update population size, it can change after selection and crossover
                population_size = population.len();

                for i in 0..population.len() {
                    $mutation(&mut population[i]);
                }
            }

        population_params.clear();

        } // END OF LOOP
        if world.rank() == root_rank{
            println!("\n\n- Overall best fitness is {}", best_fitness.expect("Error: can't read best fitness"));
            println!("- The best individual is:
                generation:\t{}
                index:\t\t{}
                fitness:\t{}
                string:\t{}\n",
                best_individual.as_ref().expect("Error: can't read best individual").generation,
                best_individual.as_ref().expect("Error: can't read best individual").index,
                best_individual.as_ref().expect("Error: can't read best individual").fitness,
                best_individual_string);
        }
        // return arrays containing all the results of each simulation
        all_results
    }};

}
