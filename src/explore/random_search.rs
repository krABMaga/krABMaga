#[macro_export]
macro_rules! random_search {
    (
        $init_state: expr,
        $n_iter : expr,
        $target : expr,
        $cost_function: tt,
        $gen_sample: tt,
        $batch_size: expr,
        $step: expr,
        $reps: expr,
    ) => {{
        let n_iter = $n_iter as usize;
        let reps = $reps as usize;
        let batch_size = $batch_size as usize;
        let reps = $reps as usize;
        let target = $target as f64;

        // let mut best_state = $init_state;
        let mut min_cost = 0.;
        for r in 0..reps {
            // initialize the state
            let mut schedule: Schedule = Schedule::new();
            $init_state.init(&mut schedule);
            // compute the simulation
            for _ in 0..($step as usize) {
                let state = $init_state.as_state_mut();
                schedule.step(state);
                if state.end_condition(&mut schedule) {
                    break;
                }
            }
            min_cost += $cost_function(&$init_state);
        }
        min_cost /= reps as f64;
        println!("Init cost {}", min_cost);

        let mut best = ($init_state, min_cost);

        if min_cost > target {
            for i in 0..n_iter {
                // let ref_best = Arc::new(&best_state);
                let mut new_samples = Vec::new();
                for _ in 0..batch_size {
                    new_samples.push($gen_sample(&best.0));
                }

                let mut results = Vec::new();
                // init new samples()

                new_samples
                    .par_iter_mut()
                    .map(|new_state| {
                        let mut cost = 0.;
                        for r in 0..reps {
                            // initialize the state
                            let mut schedule: Schedule = Schedule::new();
                            new_state.init(&mut schedule);
                            // compute the simulation
                            for _ in 0..($step as usize) {
                                let new_state = new_state.as_state_mut();
                                schedule.step(new_state);
                                if new_state.end_condition(&mut schedule) {
                                    break;
                                }
                            }
                            cost += $cost_function(&new_state);
                        }
                        cost / (reps as f64)
                    })
                    .collect_into_vec(&mut results);

                let mut min_index = -1;
                for i in 0..batch_size {
                    if results[i] < min_cost {
                        min_cost = results[i];
                        min_index = i as i32;
                    }
                }

                if min_index != -1 {
                    best = (new_samples.remove(min_index as usize), min_cost);
                }
                println!("Iter {}, min cost {}", i, min_cost);

                if min_cost <= target {
                    break;
                }

                // let cost = $cost_function(&$best);
                // if cost < min_cost {
                //     $best_state = new_state;
                //     min_cost = cost;
                //     if( min_cost <= target ) {
                //         return min_cost;
                //     }
                // }
            }
        }
        println!("Search is end");
        best
    }};
}
