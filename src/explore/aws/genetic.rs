/// Macro to perform sequential model exploration using a genetic algorithm on AWS
///
/// # Arguments
/// * `init_population` - function that creates the population, must return an array of individual (an individual is the state of the simulation to compute)
/// * `fitness` - function that computes the fitness value, takes a single individual and the schedule, must return an f32
/// * `mutation` - function that perform the mutation, takes a single individual as parameter
/// * `crossover` - function that creates the population, takes the entire population as parameter
/// * `state` - state of the simulation representing an individual
/// * `desired_fitness` - desired fitness value
/// * `generation_num` - max number of generations to compute
/// * `step` - number of steps of the single simulation
/// * `num_func` - number of lambda functions to be spawned
/// * `reps` - number of repetitions of the simulation using each individual
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
        $($reps: expr,)?
    ) => {{
        println!("Running GA exploration on AWS...");

        println!("Checking if requirements are installed...");

        let rab_aws_check = r#"
#!/bin/bash

echo "Checking that aws-cli is installed and configured..."
which aws
if [ $? -eq 0 ]; then
    echo "aws-cli is installed, continuing..."
else
    echo "You need aws-cli to deploy the lambda function! Exiting..."
    exit 1
fi

aws configure get region
if [ $? -eq 0 ]; then
    echo "aws-cli is configured, continuing..."
else
    echo "You need to configure the aws-cli to deploy the lambda function! Exiting..."
    exit 1
fi

echo "Checking that docker is installed and configured..."
which docker
if [ $? -eq 0 ]; then
    echo "docker is installed, continuing..."
else
    echo "You need docker to build the lambda function! Exiting..."
    exit 1
fi

docker_check=$(groups $USER)
if [[ $docker_check == *"docker"* ]]; then 
    echo "docker is configured correctly, continuing..."
else 
    echo "You need to configure docker to run without sudo permission! Exiting..."
    exit 1
fi

echo "Checking that cross is installed..."
which cross
if [ $? -eq 0 ]; then
    echo "cross is installed, continuing..."
else
    echo "cross is not installed, installing..."
    cargo install cross
fi

"#;

        // write the deploy_script in function.rs file
        let file_name = format!("check.sh");
        fs::write(file_name, rab_aws_check).expect("Unable to write check.sh file.");

        let check = Command::new("bash").arg("check.sh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Command \"bash check.sh\" failed!");

        let check_output = check
        .wait_with_output()
        .expect("Failed to wait on child");

        let check_output = String::from_utf8(check_output.stdout).expect("Cannot cast the check output to string!");
        println!("{}", check_output);

        let mkdir_output = Command::new("rm")
        .arg("check.sh")
        .stdout(Stdio::piped())
        .output()
        .expect("Command \"rm check.sh\" failed!");
        let mkdir_output = String::from_utf8(mkdir_output.stdout).expect("Cannot cast output of command into String!");
        println!("{}", mkdir_output);

        if check_output.contains("Exiting") {
            std::process::exit(0);
        }

        // create the folder rab_aws where all the file will be put
        println!("Creating rab_aws folder...");
        let mkdir_output = Command::new("mkdir")
        .arg("rab_aws")
        .stdout(Stdio::piped())
        .output()
        .expect("Command \"mkdir rab_aws\" failed!");
        let mkdir_output = String::from_utf8(mkdir_output.stdout).expect("Cannot cast output of command into String!");
        println!("{}", mkdir_output);

        // configuration of the different aws clients
        let mut aws_config: Option<aws_config::Config> = None;
        let mut client_sqs: Option<aws_sdk_sqs::Client> = None;
        let mut queue_url: String = String::new();

        // wait until all the async operations completes
        let _result = Runtime::new().expect("Cannot create Runtime!").block_on({
            async {
                aws_config = Some(aws_config::load_from_env().await);

                // create the sqs client
                client_sqs = Some(aws_sdk_sqs::Client::new(&aws_config.expect("Cannot create SQS client!")));

                println!("Creating the SQS queue rab_queue...");
                // create the sqs queue
                let create_queue = client_sqs.as_ref().expect("Cannot create the create queue request!")
                .create_queue()
                .queue_name("rab_queue")
                .send().await;

                queue_url = create_queue.as_ref().expect("Cannot create the get queue request!")
                .queue_url.as_ref().expect("Cannot create the get queue request!")
                .to_string();
                println!("SQS queue creation {:?}", create_queue);
            }
        });

        // create the string that will be written in the function.rs file and deployed on aws
        // copy the main.rs content
        let mut main_file = File::open("src/main.rs").expect("Cannot open main.rs file!");
        let mut main_str = String::new();
        main_file.read_to_string(&mut main_str);
        // replace the main with a dummy main to avoid overlapping
        main_str = main_str.replace("fn main", "fn dummy_main");

        let mut reps = 1;
        $(reps = $reps;)?

        // generate the lambda function
        let function_str = format!(r#"
use krabmaga::{{
    lambda_runtime,
    aws_sdk_sqs,
    aws_config,
    tokio
}};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {{
    let func = lambda_runtime::handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}}

async fn func(event: Value, _: lambda_runtime::Context) -> Result<(), lambda_runtime::Error> {{

    // read the payload
    let my_population_params = event["individuals"].as_array().expect("Cannot parse individuals value from event!");

    // prepare the result json to send on the queue
    let mut results: String = format!("{{{{\n\t\"function\":[");

    //let reps = {}; // $reps
    
    for (index, ind) in my_population_params.iter().enumerate(){{
        let individual = ind.as_str().expect("Cannot cast individual!").to_string();
        
        let mut computed_ind: Vec<({}, Schedule)> = Vec::new(); // $state

        //for _ in 0..(reps as usize){{
            // initialize the state
            let mut individual_state = <{}>::new_with_parameters(&individual); // <$state>::new_with_parameters(&individual);
            let mut schedule: Schedule = Schedule::new();
            individual_state.init(&mut schedule);
            // compute the simulation
            for _ in 0..({} as usize) {{ // $step as usize
                let individual_state = individual_state.as_state_mut();
                schedule.step(individual_state);
                if individual_state.end_condition(&mut schedule) {{
                    break;
                }}
            }}

            computed_ind.push((individual_state, schedule));


       // }}

        // compute the fitness value
        let fitness = {}(&mut computed_ind); //$fitness(&mut computed_ind);

        {{
            results.push_str(&format!("\n\t{{{{\n\t\t\"Index\": {{}}, \n\t\t\"Fitness\": {{}}, \n\t\t\"Individual\": \"{{}}\"\n\t}}}},", index, fitness, individual).to_string());
        }}
    }}

    results.truncate(results.len()-1); // required to remove the last comma
    results.push_str(&format!("\n\t]\n}}}}").to_string());

    // send the result on the SQS queue
    send_on_sqs(results.to_string()).await;
    
    Ok(())
}}

async fn send_on_sqs(results: String) -> Result<(), aws_sdk_sqs::Error> {{
    // configuration of the aws client
	let region_provider = aws_config::meta::region::RegionProviderChain::default_provider();
	let config = aws_config::from_env().region(region_provider).load().await;

    // create the SQS client
	let client_sqs = aws_sdk_sqs::Client::new(&config);
    

    // get the queue_url of the queue
    let queue = client_sqs.get_queue_url().queue_name("rab_queue".to_string()).send().await?;
    let queue_url = queue.queue_url.expect("Cannot get the queue url!");

    let send_request = client_sqs
        .send_message()
        .queue_url(queue_url)
        .message_body(results)
        .send()
        .await?;

    Ok(())
}}
// end of the lambda function
        "#, reps, stringify!($state), stringify!($state), stringify!($step), stringify!($fitness));

        // join the two strings and write the function.rs file
        main_str.push_str(&function_str);

        // write the function in function.rs file
        let file_name = format!("src/function.rs");
        fs::write(file_name, main_str).expect("Unable to write function.rs file.");

        // create the rab_aws_deploy.sh file
        let rab_aws_deploy = r#"
#!/bin/bash

echo "Generating the json files required for lambda creation..."
echo '{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "sqs:*"
            ],
            "Resource": "*" 
        },
        {
            "Effect":"Allow",
            "Action": [
                "logs:CreateLogGroup",
                "logs:CreateLogStream",
                "logs:PutLogEvents"
            ],
            "Resource": "*"
        }
    ]
}' > rab_aws/policy.json
    
echo '{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": {
                "Service": "lambda.amazonaws.com"
            },
            "Action": "sts:AssumeRole" 
        }
    ]
}' > rab_aws/rolePolicy.json

echo "Creation of IAM Role rab_role..."
role_arn=$(aws iam create-role --role-name rab_role --assume-role-policy-document file://rab_aws/rolePolicy.json --query 'Role.Arn')
echo "IAM Role rab_role created at ARN "${role_arn//\"}

echo "Attacching policy to IAM Role..."	
aws iam put-role-policy --role-name rab_role --policy-name rab_policy --policy-document file://rab_aws/policy.json

echo "Function building..."
cross build --release --features aws --bin function --target x86_64-unknown-linux-gnu
echo "Zipping the target for the upload..."
cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip rab_aws/rab_lambda.zip bootstrap && rm bootstrap 

echo "Creation of the lambda function..."
aws lambda create-function --function-name rab_lambda --handler main --zip-file fileb://rab_aws/rab_lambda.zip --runtime provided.al2 --role ${role_arn//\"} --timeout 900 --memory-size 10240 --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active 
"#;

        // write the deploy_script in function.rs file
        let file_name = format!("rab_aws/rab_aws_deploy.sh");
        fs::write(file_name, rab_aws_deploy).expect("Unable to write rab_aws_deploy.sh file.");

        println!("Running rab_aws_deploy.sh...");
        let deploy = Command::new("bash").arg("rab_aws/rab_aws_deploy.sh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Command \"bash rab_aws/rab_aws_deploy.sh\" failed!");

        let deploy_output = deploy
        .wait_with_output()
        .expect("Failed to wait on child");

        let deploy_output = String::from_utf8(deploy_output.stdout).expect("Cannot cast the deploy output to string!");
        println!("{}", deploy_output);

        build_dataframe_explore!(BufferGA, input {
            generation: u32
            index: i32
            fitness: f32
            individual: String
        });

        let mut generation = 0;
        let mut best_fitness = 0.;
        let mut best_generation = 0;

        let mut results: Vec<BufferGA> = Vec::new();

        // use init_population custom function to create a vector of individual
        let mut population: Vec<String> = $init_population();
        let mut pop_fitness: Vec<(String, f32)> = Vec::new();
        let mut best_individual: String = String::new();

        let mut population_params: Vec<String> = Vec::new();

        // flag to break from the loop
        let mut flag = false;

        // iterates until the desired fitness is found or
        // maximum number of generation is reached
        loop {

            if $generation_num != 0 && generation == $generation_num {
                println!("Reached {} generations, exiting...", $generation_num);
                break;
            }

            if flag {
                println!("Reached best fitness on generation {}, exiting...", generation);
                break;
            }

            generation += 1;
            println!("Running Generation {}...", generation);

            // population size for each function
            let mut total_functions = (population.len() * reps);
            let mut population_size_per_function = total_functions / $num_func;
            let mut remainder = total_functions % $num_func;

            let mut best_fitness_gen = 0.;
            let mut best_individual_gen: String = String::new();

            //counter for functions without additional reps from remainder
            let mut remained_funcs = 0;
            let mut update = false;
            // for each function prepare the population to compute and
            // invoke the function with that population
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
                    //added the % operation to balance if # of functions is bigger then rep
                    // if there is remainder, we calculate the index in different way
                    if (total_functions % $num_func == 0) {
                        population_params.push(population[(i * sub_population_size + j)%population.len()].clone());
                    } else {
                        if (i < (total_functions % $num_func)) {
                            population_params.push(population[(i * sub_population_size + j)%population.len()].clone());
                        } else {
                            let base_func = total_functions % $num_func;
                            let initial_offset = base_func * (population_size_per_function + 1);
                            let additional_offset = remained_funcs * (population_size_per_function);
                            let final_index = initial_offset + additional_offset + j;
                            population_params.push(population[(final_index)%population.len()].clone());  //remove clone
                            update = true;
                        }
                    }
                }

                // update the counter of the functions for the offset
                if (update) {
                    remained_funcs += 1;
                    update = false;
                }

                // create the json file with the parameters required to run the lambda function
                {
                    let mut pop_params_json= serde_json::to_string(&population_params).expect("Cannot parse params!");

                    let mut params = String::new();

                    params.push_str(&format!("{{\n\t\"individuals\": {}\n}}", pop_params_json));

                    // wait until all the async operations completes
                    let _result = Runtime::new().expect("Cannot create Runtime!").block_on({
                        async {
                            // create the lambda client
                            let config = aws_config::load_from_env().await;
                            let client_lambda = aws_sdk_lambda::Client::new(&config);

                            println!("Invoking lambda function {}...", i);
                            // invoke the function
                            let invoke_lambda = client_lambda
                            .invoke_async()
                            .function_name("rab_lambda")
                            .invoke_args(
                                aws_sdk_lambda::ByteStream::from(params.as_bytes().to_vec())
                            )
                            .send().await;
                            println!("Result of the invocation: {:?}", invoke_lambda);
                        }
                    });

                }
                population_params.clear();
            }

            // retrieve the result of the function from the SQS queue
            // receive messages until we got the same number of messages as the number of functions invoked
            let mut num_msg = 0;
            let mut messages: Vec<String> = Vec::new();
            println!("Receiving messages from the SQS queue...");
            while num_msg != $num_func {
                // wait until all the async operations completes

                let _result = Runtime::new().expect("Cannot create Runtime!").block_on({
                    async {
                        // receive the message from the queue
                        let receive_msg = client_sqs.as_ref().expect("Cannot create the receive message request!")
                        .receive_message()
                        .queue_url(queue_url.clone())
                        .wait_time_seconds(20)
                        .send().await;

                        // save the messages received and their receipts
                        let mut receipts: Vec<String> = Vec::new();
                        for message in receive_msg.expect("Cannot use the receive message request!")
                        .messages.expect("Cannot get the message from the receive request!") {
                            messages.push(message.body.expect("Cannot get the body from message!"));
                            receipts.push(message.receipt_handle.expect("Cannot get the receipt from message!"));
                        }

                        // delete the message received from the queue
                        // using the receipts
                        for rec in receipts{
                            let delete_msg = client_sqs.as_ref().expect("Cannot create the delete message request!").delete_message()
                            .queue_url(queue_url.clone())
                            .receipt_handle(rec)
                            .send().await;
                        }
                        num_msg += 1;
                    }
                });
            }

            // parse the messages received
            for i in 0..messages.len(){
                let json: serde_json::Value = serde_json::from_str(&messages[i]).expect("Cannot parse the json file!");

                let function_res = json["function"].as_array().expect("Cannot parse messages of function!");

                let mut json_fitness = 0.;
                let mut json_individual: String = String::new();

                for res in function_res {

                    json_fitness = res["Fitness"].as_f64().expect("Cannot parse \"Fitness\" field!") as f32;
                    json_individual = res["Individual"].as_str().expect("Cannot parse \"Individual\" field!").to_string();

                    pop_fitness.push((json_individual.clone(), json_fitness));

                    if json_fitness > best_fitness_gen {
                        best_fitness_gen = json_fitness;
                        best_individual_gen = json_individual.clone();
                    }

                    if json_fitness >= $desired_fitness{
                        flag = true;
                    }

                    results.push(BufferGA::new(
                        generation, //generation,
                        res["Index"].as_i64().expect("Cannot parse \"Index\" field!") as i32, // index,
                        json_fitness,
                        json_individual
                    ));
                }
            }

            if best_fitness_gen > best_fitness {
                best_fitness = best_fitness_gen;
                best_individual = best_individual_gen.clone();
                best_generation = generation;
            }

            println!("- Best fitness in generation {} is {}", generation, best_fitness_gen);
            println!("-- Overall best fitness is found in generation {} and is {}", best_generation, best_fitness);

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

            {
                // mutate the new population
                population.clear();

                for (individual, _) in pop_fitness.iter_mut() {
                    $mutation(individual);
                    population.push(individual.clone())
                }
                pop_fitness.clear();

                // crossover the new population
                $crossover(&mut population);
            }
        }

        println!("Resulting best fitness is {}", best_fitness);
        println!("- The best individual is:\n\t{}", best_individual);

        let rab_aws_undeploy = r#"
echo "Deleting resources created on AWS for the execution..."

echo "Deleting the lambda function rab_lambda..."
aws lambda delete-function --function-name rab_lambda

echo "Deleting the SQS queue rab_queue..."
queue_url=$(aws sqs get-queue-url --queue-name rab_queue --query "QueueUrl")
aws sqs delete-queue --queue-url ${queue_url//\"}

echo "Deleting the IAM role rab_role..."
aws iam delete-role-policy --role-name rab_role --policy-name rab_policy
aws iam delete-role --role-name rab_role

rm -r rab_aws
rm src/function.rs
"#;

        // write the deploy_script in function.rs file
        let file_name = format!("rab_aws/rab_aws_undeploy.sh");
        fs::write(file_name, rab_aws_undeploy).expect("Unable to write rab_aws_undeploy.sh file.");

        println!("Running rab_aws_undeploy.sh...");
        let undeploy = Command::new("bash").arg("rab_aws/rab_aws_undeploy.sh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Command \"bash rab_aws/rab_aws_undeploy.sh\" failed!");

        let undeploy_output = undeploy
        .wait_with_output()
        .expect("Failed to wait on child");

        let undeploy_output = String::from_utf8(undeploy_output.stdout).expect("Cannot cast the undeploy output to string!");
        println!("{}", undeploy_output);

        results
    }};

}
