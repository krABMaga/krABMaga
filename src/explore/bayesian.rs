#[cfg(any(feature = "bayesian"))]
use {
    friedrich::gaussian_process::GaussianProcess,
    friedrich::kernel::Gaussian,
    friedrich::prior::ConstantPrior,
    statrs::distribution::{Continuous, ContinuousCDF, Normal},
};

/// Macro to perform bayesian optimization with default functions.
///
/// # Arguments
///
/// * `x_init` - initial x values
///
/// * `y_init` - costs of x_init elements
///
/// * `objective_function` - function to evaluate the goodness of a solution.
///
/// * `n_iter` - number of iterations of bayesian optimization algorithm
///
/// * `batch_size` - how many samples for each iteration
///
/// * `scale` - factor of scaling to generate samples
#[cfg(any(feature = "bayesian"))]
#[macro_export]
macro_rules! bayesian_search {
    (
        $x_init: expr,
        $objective_function: tt,
        $gen_params: tt,
        $n_iter: expr

    ) => {{
        use $crate::{
            friedrich::gaussian_process::GaussianProcess, friedrich::kernel::Gaussian,
            friedrich::prior::ConstantPrior,
        };

        let mut x_init: Vec<Vec<f64>> = $x_init;
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
        $x_init: expr,
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

        let mut x_init: Vec<Vec<f64>> = $x_init;
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

#[cfg(any(feature = "bayesian"))]
pub fn acquisition_function(
    x_init: &Vec<Vec<f64>>,
    x_samples: &Vec<Vec<f64>>,
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

#[cfg(any(feature = "bayesian"))]
pub fn expected_improvement(
    x_init: &Vec<Vec<f64>>,
    x: &Vec<f64>,
    gp: &GaussianProcess<Gaussian, ConstantPrior>,
) -> f64 {
    let mean_y_new: f64;
    let mut sigma_y_new: f64;

    mean_y_new = gp.predict(x);
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
