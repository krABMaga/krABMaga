mod engine;
mod explore;
#[cfg(test)]
mod model;
mod utils;

#[test]
fn simulate_ui_structs() {
    use krabmaga::*;
    use rand::Rng;

    let mut rng = rand::thread_rng();

    addplot!(
        String::from("Agents"),           // Plot Name
        String::from("Steps"),            // Axis X
        String::from("Number of agents"), //Axis Y
        true                              // Save the plot locally
    );

    for step in 0..10 {
        plot!(
            String::from("Agents"),
            String::from("Wolfs"),
            step as f64,
            rng.gen_range(0..10_u32) as f64
        );

        plot!(
            String::from("Agents"),
            String::from("Sheeps"),
            step as f64,
            rng.gen_range(0..10_u32) as f64
        );
    }

    // {
    //     {
    //         let data;
    //         {
    //             data = DATA.lock().unwrap().clone();
    //         }
    //         for (_, plot) in data.iter() {
    //             if plot.to_be_stored {
    //                 plot.store_plot(0)
    //             }
    //         }
    //     }

    //     use std::path::Path;
    //     let date = CURRENT_DATE.clone();
    //     let path = format!("output/{}/Agents/Agents_0.png", date);

    //     // Check if the plot file exists
    //     assert!(Path::new(&path).exists());

    //     // Remove the file
    //     fs::remove_dir_all("output").expect("Error removing output directory");
    // }

    {
        log!(LogType::Info, "Info Log".to_string(), true);
        log!(LogType::Warning, "Warning Log".to_string());
        log!(LogType::Error, "Error Log".to_string());
        log!(LogType::Critical, "Critical Log".to_string(), true);
    }
    {
        let logs = LOGS.lock().unwrap();
        for log in logs.iter().flatten() {
            match log.ltype {
                LogType::Info => assert_eq!(log.body, "Info Log"),
                LogType::Warning => assert_eq!(log.body, "Warning Log"),
                LogType::Error => assert_eq!(log.body, "Error Log"),
                LogType::Critical => assert_eq!(log.body, "Critical Log"),
            }
        }
    }
}
