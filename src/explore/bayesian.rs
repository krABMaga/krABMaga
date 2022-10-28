#[cfg(any(feature = "bayesian"))]
use {
    friedrich::gaussian_process::GaussianProcess,
    friedrich::kernel::Gaussian,
    friedrich::prior::ConstantPrior,
    statrs::distribution::{Continuous, ContinuousCDF, Normal},
};

/// Macro to perform bayesian optimization.
/// You should use this macro to optimize your simulation, but can be used
/// to optimize any function.
/// # Arguments: Main pattern
/// * `init_parameters` : a function that returns a vector of initial parameters. Returns a vector of vectors of f64.
/// * `objective_function` : a function that returns the objective value of a parameter. This function should be
///                 the execution of your simulation and the evaluation of the results. Returns a f64.
///                 The bayesian optimization will try to maximize this function. If you want to minimize it,
///                 return the negative value of your objective function.
/// * `gen_samples` : a function that returns a vector of samples
/// * `n_iter` : number of iterations
///
/// # Arguments: Optional pattern
/// * `init_parameters` : a function that returns a vector of initial parameters
/// * `objective_function` : a function that returns the objective value of a parameter. This function should be
///                 the execution of your simulation and the evaluation of the results. Returns a f64.
///                 The bayesian optimization will try to maximize this function. If you want to minimize it,
///                 return the negative value of your objective function.
/// * `num_of_params` : number of parameters. This is used to generate random samples
/// * `n_iter` - number of iterations of bayesian optimization algorithm
/// * `batch_size` - number of samples for each iteration
/// * `scale` - factor of scaling to generate samples
///
/// # Example: Main pattern
/// ```
/// # use {krabmaga::bayesian_search, krabmaga::explore::bayesian::*};   
/// // function to initialize parameters
/// fn init_parameters() -> Vec<Vec<f64>> {
///     vec![vec![-2., -2.], vec![8., 1.], vec![-1., 5.], vec![4., -2.]]
/// }
///
/// // function to generate samples
/// fn generate_samples(_x_values: &[Vec<f64>]) -> Vec<Vec<f64>> {
///   let batch_size = 500;
///   let num_params = 2;
///   (0..batch_size)
///      .into_iter()
///      .map(|_| {
///        let mut t_x = Vec::with_capacity(num_params);
///        let mut rng = rand::thread_rng();
///        for _ in 0..num_params {
///          t_x.push(rand::Rng::gen_range(&mut rng, -10.0..=10.0));
///        }
///        t_x
///       })
///     .collect()
/// }
///
/// // function to evaluate the objective function
/// // we want to find the minimum of the function: x^2 + y^2
/// // so we return the inverse of the function,
/// // because the bayesian optimization algorithm tries to maximize the objective function
/// fn objective_square(x: &[f64]) -> f64 {
///   let total = (x[0]).powf(2.) + (x[1]).powf(2.);
///   -1.* total
/// }
/// let (x, y) = bayesian_search!(init_parameters, objective_square, generate_samples, 20);
///
/// println!("---\nFinal res: Point {:?}, val {y}", x);
/// assert!(x[0].abs() < 1. && x[1].abs() < 1.);
///
/// ```
///
/// # Example: Optional pattern
/// ```
/// # use {krabmaga::bayesian_search, krabmaga::explore::bayesian::*};
///
/// // initialize parameters
/// fn init_parameters() -> Vec<Vec<f64>> {
///     vec![vec![-2., -2.], vec![8., 1.], vec![-1., 5.], vec![4., -2.]]
/// }
///
/// // function to evaluate the objective function
/// // we want to find the minimum of the function: x^2 + y^2
/// // so we return the inverse of the function,
/// // because the bayesian optimization algorithm tries to maximize the objective function
/// fn objective_square(x: &[f64]) -> f64 {
///     let total = (x[0]).powf(2.) + (x[1]).powf(2.);
///     -1.* total
/// }
///
/// let num_of_params = 2;
/// let n_iter = 20;
/// let batch_size = 500;
/// let scale = 10.;
///
/// let (x, y) = bayesian_search!(init_parameters, objective_square, num_of_params, n_iter, batch_size, scale);
///
/// println!("---\nFinal res: Point {:?}, val {y}", x);
/// assert!(x[0].abs() < 1. && x[1].abs() < 1.);
///
/// ```

#[cfg(any(feature = "bayesian"))]
#[macro_export]
macro_rules! bayesian_search {
    (
        $init_parameters: tt,
        $objective_function: tt,
        $gen_params: tt,
        $n_iter: expr
    ) => {{
        use $crate::{
            friedrich::gaussian_process::GaussianProcess, friedrich::kernel::Gaussian,
            friedrich::prior::ConstantPrior,
        };

        let mut x_init: Vec<Vec<f64>> = $init_parameters();
        let mut y_init: Vec<f64> = Vec::with_capacity(x_init.len());
        for x in &x_init {
            y_init.push($objective_function(x));
        }

        // find best initial x sample
        // the best is the one with the lowest cost
        let mut y_max = f64::MIN;
        let mut y_index = 0;
        for (i, y) in y_init.clone().iter_mut().enumerate() {
            if *y > y_max {
                y_max = *y;
                y_index = i;
            }
        }

        let mut x_max = x_init[y_index].clone();

        // init gaussian process
        let mut gp = GaussianProcess::default(x_init.clone(), y_init.clone());

        for i in 0..$n_iter {
            println!("-----\nIteration {i}");

            // generate samples
            let x_samples = $gen_params(&x_init);

            let x_next = acquisition_function(&x_init, &x_samples, &gp);
            //evaluation od new val
            let y_next = $objective_function(&x_next);
            println!("New point {:?}", &x_next);
            println!("f(x) = {y_next}");

            // prediction of new val
            let y_pred = gp.predict(&x_next);
            println!("Predicted f(x) = {y_pred}");

            // update gaussian process
            gp.add_samples(&vec![x_next.clone()], &vec![y_next]);

            x_init.push(x_next.clone());

            if y_next > y_max {
                y_max = y_next;
                x_max = x_next;
            }
        }

        (x_max, y_max)
    }};

    (
        $init_parameters: tt,
        $objective_function: tt,
        $num_of_params: expr,
        $n_iter: expr,
        $batch_size: expr,
        $scale: expr

    ) => {{
        use $crate::{
            friedrich::gaussian_process::GaussianProcess, friedrich::kernel::Gaussian,
            friedrich::prior::ConstantPrior,
        };

        let mut x_init: Vec<Vec<f64>> = $init_parameters();
        let mut y_init: Vec<f64> = Vec::with_capacity(x_init.len());
        for x in &x_init {
            y_init.push($objective_function(x));
        }
        let num_of_params: usize = $num_of_params;
        // find best initial x sample
        // the best is the one with the lowest cost
        let mut y_max = f64::MIN;
        let mut y_index = 0;
        for (i, y) in y_init.clone().iter_mut().enumerate() {
            if *y > y_max {
                y_max = *y;
                y_index = i;
            }
        }

        let mut x_max = x_init[y_index].clone();

        // init gaussian process
        let mut gp = GaussianProcess::default(x_init.clone(), y_init.clone());

        // init n_iter

        for i in 0..$n_iter {
            println!("-----\nIteration {i}");

            // generate samples
            let x_samples =
                $crate::explore::bayesian::generate_samples($batch_size, $scale, num_of_params);

            let x_next = acquisition_function(&x_init, &x_samples, &gp);
            //evaluation od new val
            let y_next = $objective_function(&x_next);
            println!("New point {:?}", &x_next);
            println!("f(x) = {y_next}");

            // prediction of new val
            let y_pred = gp.predict(&x_next);
            println!("Predicted f(x) = {y_pred}");

            // update gaussian process
            gp.add_samples(&vec![x_next.clone()], &vec![y_next]);

            x_init.push(x_next.clone());

            if y_next > y_max {
                y_max = y_next;
                x_max = x_next;
            }
        }

        (x_max, y_max)
    }};
}

/// Function to generate samples for the bayesian optimization algorithm.
///
/// # Arguments
/// * `batch_size` - number of samples to generate
/// * `scale` - scale of the samples
/// * `num_of_params` - number of parameters of the objective function
///
#[cfg(any(feature = "bayesian"))]
pub fn generate_samples(batch_size: usize, scale: f64, num_of_params: usize) -> Vec<Vec<f64>> {
    (0..batch_size)
        .into_iter()
        .map(|_| {
            let mut t_x = Vec::with_capacity(num_of_params);
            let mut rng = rand::thread_rng();
            for _ in 0..num_of_params {
                t_x.push(rand::Rng::gen_range(&mut rng, -1.0..=1.0) * scale);
            }
            t_x
        })
        .collect()
}

/// Acquisition function for the bayesian optimization algorithm.
/// returns the point with the highest expected improvement.
///
/// # Arguments
/// * `x_init` - initial samples
/// * `x_samples` - samples to evaluate
/// * `gp` - gaussian process to use for the prediction
#[cfg(any(feature = "bayesian"))]
pub fn acquisition_function(
    x_init: &Vec<Vec<f64>>,
    x_samples: &[Vec<f64>],
    gp: &GaussianProcess<Gaussian, ConstantPrior>,
) -> Vec<f64> {
    let scores = x_samples
        .iter()
        .map(|x| expected_improvement(x_init, x, gp));

    let mut max_score = f64::MIN;
    let mut max_index = 0;
    for (i, score) in scores.enumerate() {
        if score > max_score {
            max_score = score;
            max_index = i;
        }
    }

    x_samples[max_index].clone()
}

/// Expected improvement function for the bayesian optimization algorithm.
///
/// # Arguments
/// * `x_init` - initial samples
/// * `x` - sample to evaluate
/// * `gp` - gaussian process to use for the prediction
#[cfg(any(feature = "bayesian"))]
pub fn expected_improvement(
    x_init: &Vec<Vec<f64>>,
    x: &Vec<f64>,
    gp: &GaussianProcess<Gaussian, ConstantPrior>,
) -> f64 {
    let mut sigma_y_new: f64;

    let mean_y_new = gp.predict(x);
    sigma_y_new = gp.predict_variance(x); //standard deviation
    sigma_y_new = sigma_y_new.sqrt();
    if sigma_y_new == 0. {
        return 0.;
    }

    let mut mean_y: Vec<f64> = Vec::with_capacity(x_init.len());
    for x in x_init {
        mean_y.push(gp.predict(x));
    }

    // let mean_y_max = mean_y.iter().max().expect("Something goes wrong, no input variables");
    let mut mean_y_max = f64::MIN;
    for m_y in &mean_y {
        if *m_y > mean_y_max {
            mean_y_max = *m_y;
        }
    }

    let z = (mean_y_new - mean_y_max) / sigma_y_new;
    let normal = Normal::new(0.0, 1.0)
        .expect("Error building normal distribution inside acquisition function");
    let z_cfd = normal.cdf(z);
    let z_pdf = normal.pdf(z);
    (mean_y_new - mean_y_max) * z_cfd + sigma_y_new * z_pdf
}
