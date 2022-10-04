/// Macro to perform model exploration using the random search algorithm.
/// Try to find the best model by randomly sampling the parameter space.
/// The goal is to minimize the cost function.
///
/// Parallelize the search using the `rayon` crate at each iteration.
///
/// # Arguments
/// * `init_state` - state that has to be initialized
/// * `n_iter` - number of iterations
/// * `target` - Goal of the search. If the cost function is below this value, the search stops
/// * `cost_function` - fitness function to calculate the optimal value
/// * `gen_sample` - function to generate the samples that have to be analyzed
/// * `batch_size` - number of samples
/// * `step` - number of steps for the simulation
/// * `reps` - number of repetitions for each sample
///
/// # Example
/// ```
/// # use krabmaga::*;
/// fn main() {
///     let init_state = State::new();
///     let n_iter = 10;
///     let target = 0.0;
///     let batch_size = 10;
///     let step = 1000;
///     let reps = 5;
///     let (best_state, cost)  = random_search!(
///         init_state,
///         n_iter,
///         target,
///         costly_function,
///         gen_sample,
///         batch_size,
///         step,
///         reps
///     );
/// }
///
/// fn costly_function(state: &State) -> f32 { ... }
///
/// fn gen_sample(state: &State) -> State { ... }
/// ```
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
        $reps: expr
    ) => {{
        let n_iter = $n_iter as usize;
        let reps = $reps as usize;
        let batch_size = $batch_size as usize;
        let reps = $reps as usize;
        let target = $target as f32;

        // assign to min cost max f32 value
        let mut best = ($init_state, std::f32::MAX);
        let mut min_cost = std::f32::MAX;

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
                    cost / reps as f32
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
        }

        println!("Search is end");
        best
    }};
}
