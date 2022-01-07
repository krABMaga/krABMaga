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
// parameters(optional): parameter to write the csv, if not specified only fitness will be written
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
        parameters {
            $($p_name:ident: $p_type:ty)*
        }
    ) => {{
        println!("Running GA exploration on AWS...");

		println!("Checking if aws-cli is installed and configured..."); // TODO

        build_dataframe_explore!(BufferGA, input {
            // generation: u32
            // index: i32
            // fitness: f32
            $(
                $p_name: $p_type
            )*
            Serialize
            Deserialize
        });

        let mut generation = 0;
        let mut best_fitness = 0.;
        let mut best_generation = 0;

        let mut result:Vec<BufferGA> = Vec::new();

        // use init_population custom function to create a vector of state
        let mut population: Vec<$state> = $init_population();

        let population_size = population.len();

        // parameters of the individual
        $(
            let mut $p_name: Vec<$p_type> = Vec::new();
        )*

        let mut population_size_per_function = population_size / $num_func;
        let mut remainder = population_size % $num_func;

        // for each function
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
                $(
                    $p_name.push(population[i * population_size_per_function + j].$p_name.clone());  //remove clone
                )*
            }
            // creates an array of bufferGA containing the parameters for the simulation
            let mut to_send: Vec<BufferGA> = Vec::new();
            for p in 0..sub_population_size {
                to_send.push(
                    serde_json::to_string(
                        &BufferGA::new(
                            $(
                                $p_name[i * population_size_per_function + p].clone(),
                            )*
                        )
                    ).unwrap()
                );
            }

            
            {
                $(
                    let mut $p_name = serde_json::to_string(&$p_name).unwrap();
                )*

                let mut params = String::new();
                
                $(
                    params.push_str(&format!("\t\"{}\": \"{}\", \n", stringify!($p_name), $p_name));
                )*

                params = format!("{{\n{}\t\"step\": \"{}\"\n}}", params, $step);

                let file_name = format!("rab_aws/parameters_{}.json", i);
                fs::write(file_name, params).expect("Unable to write parameters.json file.");
            }

            $(
                $p_name.clear();
            )*
        }

        // flag to break from the loop
        let mut flag = false;

        let mut main_file = File::open("src/main.rs").expect("Cannot open main.rs file!");
        let mut contents = String::new();
        main_file.read_to_string(&mut contents);

        let import_str = contents.split_once("fn main() {");

        let mut params_file = File::open("rab_aws/parameters_t.json").expect("Cannot open json file!");
        let mut contents_params = String::new();
        params_file.read_to_string(&mut contents_params);

        let params_json: serde_json::Value = serde_json::from_str(&contents_params).expect("Cannot parse the json file!");

        println!("Json \n {}", &params_json["positions"][0]);

        $(
            let $p_name = &params_json[stringify!($p_name)][0];
            println!("{} {:?}", stringify!($p_name), $p_name);
        )*
        
        let indivudal = <$state>::new(
            $(
                $p_name[i].clone(),
            )*
        );
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


            let my_pop_size = event["pop_size"].as_str();
            let parameters = event["parameters"]



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

    // perform the model exploration with genetic algorithm on AWS without writing additional parameters
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
    ) => {
        explore_ga_aws!(
            $init_population,
            $fitness,
            $selection,
            $mutation,
            $crossover,
            $state,
            $desired_fitness,
            $generation_num,
            $step,
			$num_func,
            parameters { }
        );
    };
}