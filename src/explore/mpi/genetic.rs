#[macro_export]
macro_rules! extend_dataframe_explore {
    //implementation of a trait to send/receive with mpi
    ($name:ident,
    input {$($input:ident: $input_ty:ty)*}
  //  vec {$($input_vec:ident: [$input_ty_vec:ty; $input_len: expr])*}
    ) => {
        unsafe impl Equivalence for $name {
            type Out = UserDatatype;
            fn equivalent_datatype() -> Self::Out {

                //count input and output parameters to create slice for blocklen
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

// macro to perform distribued model exploration using a genetic algorithm based on MPI
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
macro_rules! explore_ga_distributed_mpi {
    (
        $init_population:tt,
        $fitness:tt,
        $selection:tt,
        $mutation:tt,
        $crossover:tt,
        $state: ty,
        $desired_fitness: expr,
        $generation_num: expr,
        $step: expr,
        $($reps: expr,)?
    ) => {{

        // MPI initialization
        let mut universe = mpi::initialize().unwrap();
        let world = universe.world();
        let root_rank = 0;
        let root_process = world.process_at_rank(root_rank);
        let my_rank = world.rank();
        let num_procs = world.size() as usize;

        if world.rank() == root_rank {
            println!("Running distributed (MPI) GA exploration...");
        }

        let mut reps = 1;
        $(reps = $reps;)?

        let mut generation: u32 = 0;
        let mut best_fitness = 0.;
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
        let mut all_bests_fitness;
        let mut all_bests_index;
        let mut all_bests_individual;
        let mut best_individual_string = String::new();
        let mut best_individual: Option<BufferGA> = None;
        let mut pop_fitness: Vec<(String, f32)> = Vec::new();
        let mut all_results: Vec<BufferGA> = Vec::new();

        // my best for each proc through generations
        let mut my_best_fitness = 0.;
        let mut my_best_index = 0;
        let mut my_best_individual = String::new();

        //becomes true when the algorithm get desider fitness
        let mut flag = false;

        if world.rank() == root_rank {
            population = $init_population();
            population_size = population.len();

            // dummy initilization
            best_individual = Some(BufferGA::new(
                0,
                0,
                0.
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
                        population_params.push(population[i * population_size_per_process + j].clone());  //remove clone
                    }

                    // send the arrays
                    world.process_at_rank(i as i32).send(&sub_population_size);
                    let mut to_send: Vec<BufferGA> = Vec::new();

                    //const BUFFER_SIZE: usize = 1024 * 1024 * 1024;
                    //universe.set_buffer_size(BUFFER_SIZE);

                    for p in 0..sub_population_size {

                        world.process_at_rank(i as i32).send(&population_params[i * population_size_per_process + p].clone().as_bytes()[..]);

                    }
                    //universe.detach_buffer();


                }
            } else {
                // every other processor receive the parameter
                let (my_population_size, _) = world.any_process().receive::<usize>();
                my_pop_size = my_population_size;

                for i in 0..my_pop_size {
                    let (param, _) = world.any_process().receive_vec::<u8>();
                    let my_param = String::from_utf8(param).unwrap();
                    population_params.push(my_param);
                }
            }

            let mut my_population: Vec<String>  = Vec::new();

            //init local sub-population
            for i in 0..my_pop_size {
                my_population.push(population_params[i].clone());
            }

            let mut best_fitness_gen = 0.;
            let mut local_index = 0;
            // array collecting the results of each simulation run
            let mut my_results: Vec<BufferGA> = Vec::new();

            // counter for master to stop at population size
            let mut master_counter = 0;

            for individual_params in population_params.iter_mut() {

                if my_rank == 0 && (master_counter == my_pop_size) {
                    break;
                }

                // initialize the state
                let mut computed_ind: Vec<($state, Schedule)> = Vec::new();

                for _ in 0..(reps as usize){
                    let mut individual = <$state>::new_with_parameters(&individual_params);
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

                if fitness > my_best_fitness {
                    my_best_fitness = fitness;
                    my_best_index = local_index;
                    my_best_individual = individual_params.clone();
                }

                let result = BufferGA::new(
                    generation,
                    local_index,
                    fitness,
                );

                my_results.push(result);

                // if the desired fitness is reached break
                // setting the flag at true
                if fitness >= $desired_fitness{
                    flag = true;
                }

                local_index += 1;

                if my_rank == 0 {
                    master_counter += 1;
                }
            }

            //best individual sent from each proc
            if world.rank() == root_rank {
                all_bests_fitness = vec![0f32; num_procs];
                all_bests_index= vec![0i32; num_procs];
                all_bests_individual = Vec::with_capacity(num_procs);
                // gather_into remap mpi_gather so we memorize values in rank order
                root_process.gather_into_root(&my_best_fitness, &mut all_bests_fitness[..]);
                root_process.gather_into_root(&my_best_index, &mut all_bests_index[..]);

                for i in 0..num_procs {
                    if i == 0 {
                        all_bests_individual.push(my_best_individual.clone());
                    } else {
                        let (param, _) = world.process_at_rank(i as i32).receive_vec::<u8>();
                        let my_param = String::from_utf8(param).unwrap();
                        all_bests_individual.push(my_param);
                    }
                }

                // get the index of max fitness from all_bests_fitness
                let mut max_fitness_index = 0;
                for i in 0..num_procs {
                    if all_bests_fitness[i] > all_bests_fitness[max_fitness_index] {
                        max_fitness_index = i;
                    }
                }

                best_individual_string = all_bests_individual[max_fitness_index].clone();

            } else {
                root_process.gather_into(&my_best_fitness);
                root_process.gather_into(&my_best_index);
                root_process.send(&my_best_individual.clone().as_bytes()[..]);
            }

            // receive simulations results from each processors
            if world.rank() == root_rank {

                // dummy initialization
                let dummy = BufferGA::new(
                    generation,
                    0,
                    -999.,
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

                best_fitness_gen = 0.;
                // save the best individual of this generation
                let mut i = 0;
                let mut j = 0;
                for elem in partial_results.iter_mut() {
                    // only the master can update the index
                    elem.index += displs[i];

                    if elem.fitness > best_fitness_gen{
                        best_fitness_gen = elem.fitness;
                    }

                    match best_individual.clone() {
                        Some(x) => {
                            if elem.fitness > x.fitness {
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

                // combine the results received
                all_results.append(&mut partial_results);
            } else {
                // send the result to the root processor
                root_process.gather_varcount_into(&my_results[..]);
            }

            // saving the best fitness of all generation computed until n
            if best_fitness_gen > best_fitness {
                best_fitness = best_fitness_gen;
                best_generation = generation;
            }

            if world.rank() == root_rank{
                println!("- Best fitness in generation {} is {}", generation, best_fitness_gen);
                println!("-- Overall best fitness is found in generation {} and is {}", best_generation, best_fitness);
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
                break;
            }

            // the master do selection, mutation and crossover
            if world.rank() == root_rank {

                // set the population parameters owned by the master
                // using the ones received from other processors
                for i in 0..population_size {
                    let fitness = all_results[(generation as usize -1)*population_size + i].fitness;
                    let individual = population_params[i].clone();
                    let tup = (individual, fitness);
                    pop_fitness.insert(i, tup);
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
                    $mutation(individual);
                    population.push(individual.clone());
                }
                pop_fitness.clear();

                // crossover the new population
                $crossover(&mut population);
            }

        population_params.clear();

        } // END OF LOOP
        if world.rank() == root_rank{
            println!("\n\n- Overall best fitness is {}", best_fitness);
            println!("- The best individual is:
                generation:\t{}
                index:\t\t{}
                fitness:\t{}
                string:\t{}\n",
                best_individual.as_ref().unwrap().generation,
                best_individual.as_ref().unwrap().index,
                best_individual.as_ref().unwrap().fitness,
                best_individual_string);
        }
        // return arrays containing all the results of each simulation
        all_results
    }};

}
