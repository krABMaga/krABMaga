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

        // Stuff for AWS 

        // create the folder rab_aws where all the file will be put
        println!("Creating rab_aws folder...");
        let output = Command::new("mkdir").arg("rab_aws").stdout(Stdio::piped()).output().expect("Command \"mkdir rab_aws\" failed!");
        let output = String::from_utf8(output.stdout).unwrap();
        println!("{}", output);

        // configuration of the different aws clients
        let mut aws_config: Option<aws_config::Config> = None;
        let mut client_sqs: Option<aws_sdk_sqs::Client> = None;
        let mut queue_url: String = String::new();

        let _result = Runtime::new().unwrap().block_on({
            async {
                aws_config = Some(aws_config::load_from_env().await);

                // create the sqs client
                client_sqs = Some(aws_sdk_sqs::Client::new(&aws_config.unwrap()));
                
                println!("Creating the SQS queue rab_queue...");
                // create the sqs queue
                let create_queue = client_sqs.as_ref().unwrap().create_queue().queue_name("rab_queue").send().await;
                queue_url = create_queue.as_ref().unwrap().queue_url.as_ref().unwrap().to_string();
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

        // generate the lambda function
        let function_str = format!(r#"
use rust_ab::{{
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
    let id = event["id"].as_str().unwrap();

    let my_population_params = event["individuals"].as_array().unwrap();

    // prepare the result json to send on the queue
    let mut results: String = format!("{{{{\n\t\"function_{{}}\":[", id);
    
    for (index, ind) in my_population_params.iter().enumerate(){{
        let individual = ind.as_str().unwrap().to_string();
        
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

        // compute the fitness value
        let fitness = {}(&mut individual_state, schedule); //$fitness(&mut individual_state, schedule);

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
    let queue_url = queue.queue_url.unwrap();

    let send_request = client_sqs
        .send_message()
        .queue_url(queue_url)
        .message_body(results)
        .send()
        .await?;

    Ok(())
}}
// end of the lambda function
        "#, stringify!($state), stringify!($step), stringify!($fitness));
        
        // join the two strings and write the function.rs file
        main_str.push_str(&function_str);
        
        // write the function in function.rs file
        let file_name = format!("src/function.rs");
        fs::write(file_name, main_str).expect("Unable to write function.rs file.");
        
        // create the rab_aws_deploy.sh file
        let rab_aws_deploy = r#"
#!/bin/bash

echo "Checking that aws-cli is installed..."
which aws
if [ $? -eq 0 ]; then
    echo "aws-cli is installed, continuing..."
else
    echo "You need aws-cli to deploy the lambda function. Exiting...'"
    exit 1
fi

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

echo "Checking that cross is installed..."
which cross
if [ $? -eq 0 ]; then
    echo "cross is installed, continuing..."
else
    echo "cross is not installed, installing..."
    cargo install cross
fi
echo "Function building..."
cross build --release --bin function --target x86_64-unknown-linux-gnu
echo "Zipping the target for the upload..."
cp ./target/x86_64-unknown-linux-gnu/release/function ./bootstrap && zip rab_aws/rab_lambda.zip bootstrap && rm bootstrap 

echo "Creation of the lambda function..."
aws lambda create-function --function-name rab_lambda --handler main --zip-file fileb://rab_aws/rab_lambda.zip --runtime provided.al2 --role ${role_arn//\"} --timeout 900 --memory-size 10240 --environment Variables={RUST_BACKTRACE=1} --tracing-config Mode=Active 
echo "Lambda function created successfully!"

echo "Clearing the rab_aws folder..."
#rm -r rab_aws/
"#;

        // write the deploy_script in function.rs file
        let file_name = format!("rab_aws/rab_aws_deploy.sh");
        fs::write(file_name, rab_aws_deploy).expect("Unable to write rab_aws_deploy.sh file.");

        println!("Running rab_aws_deploy.sh...");
        let output = Command::new("bash").arg("rab_aws/rab_aws_deploy.sh").stdout(Stdio::piped()).output().expect("Command \"bash rab_aws/rab_aws_deploy.sh\" failed!");
        let output = String::from_utf8(output.stdout).unwrap();
        println!("{}", output);

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
                population_params.push(population[i * population_size_per_function + j].clone());  //remove clone
            }

            {
                let mut pop_params_json= serde_json::to_string(&population_params).unwrap();

                let mut params = String::new();
                
                params.push_str(&format!("\t\"individuals\": {}, \n", pop_params_json));

                params = format!("{{\n{}\t\"id\": \"{}\"\n}}", params, i);

                let file_name = format!("rab_aws/parameters_{}.json", i);
                fs::write(file_name, params.clone()).expect("Unable to write parameters.json file.");

                let _result = Runtime::new().unwrap().block_on({
                    async {
                        // configuration of the different aws clients
                        let region_provider = aws_config::meta::region::RegionProviderChain::default_provider();
                        let config = aws_config::from_env().region(region_provider).load().await;
                        
                        // create the lambda client
                        let client_lambda = aws_sdk_lambda::Client::new(&config);
                        
                        println!("Invoking lambda function {}...", i);
                        // invoke the function
                        let invoke_lambda = client_lambda
                        .invoke_async()
                        .function_name("rab_lambda")
                        .invoke_args(aws_sdk_lambda::ByteStream::from(params.as_bytes().to_vec()))
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
        println!("Trying to receive the messages num_msg {} - num_func {}...", num_msg, $num_func);
        while num_msg != $num_func {
            let _result = Runtime::new().unwrap().block_on({
                async {
                    let receive_msg = client_sqs.as_ref().unwrap().receive_message()
                    .queue_url(queue_url.clone())
                    .send().await;
                    // println!("Receiving message {}: {:?}", num_msg, receive_msg.unwrap().messages);
            
                    // save the message received
                    let mut receipts: Vec<String> = Vec::new();

                    for message in receive_msg.unwrap().messages.unwrap_or_default() {
                        messages.push(message.body.unwrap());
                        receipts.push(message.receipt_handle.unwrap());
                    }

                    // delete the message received from the queue
                    for rec in receipts{
                        let delete_msg = client_sqs.as_ref().unwrap().delete_message()
                        .queue_url(queue_url.clone())
                        .receipt_handle(rec)
                        .send().await;
                    }
                    num_msg += 1;
                }
            });
        }

        for i in 0..$num_func{
            for msg in &messages{
                let json: serde_json::Value = serde_json::from_str(&msg).expect("Cannot parse the json file!");
        
                let function_res = json[&format!("function_{}", i)].as_array().unwrap();
                
                for res in function_res {
                    // results.push(BufferGA::new(
                    //     1, //generation,
                    //     res["Index"].as_i64().unwrap() as i32, // index,
                    //     res["Fitness"].as_f64().unwrap() as f32,// fitness,
                    //     res["Individual"].as_str().unwrap().to_string()// individual
                    // ));

                    println!("Res is {:?}", res);
                }
        
                // println!("Message is {}", res);
            }
        }
        println!("&&&&&&&&&&&&&&Result to save is {:?}", results);

    }};

}
