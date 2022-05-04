mod engine;
mod explore;
#[cfg(test)]
mod model;
mod utils;

#[cfg(test)]
fn simulate_ui_structs() {
    use rand::Rng;
    use krABMaga::*;

    let mut rng = rand::thread_rng();

    addplot!(
        String::from("Agents"),           // Plot Name
        String::from("Steps"),            // Axis X
        String::from("Number of agents")  //Axis Y
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

    log!(LogType::Info, "Info Log".to_string());
    log!(LogType::Warning, "Warning Log".to_string());
    log!(LogType::Error, "Error Log".to_string());
    log!(LogType::Critical, "Critical Log".to_string());

    let logs = LOGS.lock().unwrap();

    for log in logs.iter() {
        match log.ltype {
            LogType::Info => assert_eq!(log.body, "Info Log"),
            LogType::Warning => assert_eq!(log.body, "Warning Log"),
            LogType::Error => assert_eq!(log.body, "Error Log"),
            LogType::Critical => assert_eq!(log.body, "Critical Log"),
        }
    }
}
