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
// num_func: number of functions to invoke
#[macro_export]
macro_rules! explore_ga_aws {
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
		$num_func: expr,
    ) => {{
        println!("Running GA exploration on AWS...");

		println!("Checking if aws-cli is installed and configured..."); // TODO

        build_dataframe_explore!(BufferGA, input {
            generation: u32
            index: i32
            fitness: f32
            individual: String
        });

        let mut generation = 0;
        let mut best_fitness = 0.;
        let mut best_generation = 0;

        // use init_population custom function to create a vector of individual
        let mut population: Vec<String> = $init_population();
        let mut pop_fitness: Vec<(String, f32)> = Vec::new();
        let mut best_individual: String = String::new();

        let mut my_population_params: Vec<String> = Vec::new();
   
        // flag to break from the loop
        let mut flag = false;

        // population size for each function
        let mut population_size_per_function = population.len() / $num_func;
        let mut remainder = population.len() % $num_func;

        // for each function prepare the population to compute
        for i in 0..$num_func {

            let mut sub_population_size = 0;

            // calculate the workload subdivision
            if remainder > 0 {
                sub_population_size =  population_size_per_function + 1;
                remainder -= 1;
            } else {
                sub_population_size = population_size_per_function;
            }

            // fulfill the parameters arrays
            // we got sub_population_size arrays each one with parameters for individual to compute
            for j in 0..sub_population_size {
                my_population_params.push(population[i * population_size_per_function + j].clone());  //remove clone
            }

            {
                let mut pop_params_json= serde_json::to_string(&my_population_params).unwrap();

                let mut params = String::new();
                
                params.push_str(&format!("\t\"individuals\": {}, \n", pop_params_json));

                params = format!("{{\n{}\t\"step\": \"{}\"\n}}", params, $step);

                let file_name = format!("rab_aws/parameters_{}.json", i);
                fs::write(file_name, params).expect("Unable to write parameters.json file.");
            }

            my_population_params.clear();
        }

        // flag to break from the loop
        let mut flag = false;

        let mut main_file = File::open("src/main.rs").expect("Cannot open main.rs file!");
        let mut contents = String::new();
        main_file.read_to_string(&mut contents);

        let import_str = contents.split_once("fn main() {");

        let mut params_file = File::open("rab_aws/parameters_0.json").expect("Cannot open json file!");
        let mut contents_params = String::new();
        params_file.read_to_string(&mut contents_params);

        let params_json: serde_json::Value = serde_json::from_str(&contents_params).expect("Cannot parse the json file!");

        let my_population_params = params_json["individuals"].as_array().unwrap();

        let mut my_pop: Vec<String> = Vec::new();
        for ind in my_population_params{
            let individual = ind.as_str().unwrap().to_string();

            // initialize the state
            let mut individual_state = <$state>::new_with_parameters(&individual);
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

            // compute the fitness value
            let fitness = $fitness(&mut individual_state, schedule);
        }
                

        let first_code = r#"
        use lambda_runtime::{handler_fn, Context, Error};
        use serde_json::{json, Value};

        #[tokio::main]
        async fn main() -> Result<(), Error> {
            let func = handler_fn(func);
            lambda_runtime::run(func).await?;
            Ok(())
        }

        async fn func(event: Value, _: Context) -> Result<Value, Error> {
            
            // leggo dal payload i parametri che mi servono
            let parameters = event["individuals"].as_str().unwrap().to_string();



            // for i in 0..my_population_size
            // creo gli individui
            //Epid::new(p)

            // eseguo la simulazione

            // calcolo la fitness

            // costruisco il json coi valori da restituire
            // fitness
            // generation
            // trovato o meno la desired_fitness
            // parametri di ritorno del best individual


            let check = event["check"].as_str().unwrap_or("unsuccess");
            Ok(json!({ "message": format!("The function was executed with {}!", check) }))
        }"#;
        
    }};

}