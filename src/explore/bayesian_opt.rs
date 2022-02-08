#[cfg(any(feature = "bayesian"))]
use {argmin::prelude::*,
    argmin::solver::neldermead::NelderMead,
    finitediff::FiniteDiff,
    friedrich::gaussian_process::GaussianProcess,
    friedrich::kernel::Gaussian,
    friedrich::prior::ConstantPrior,
    statrs::distribution::{Continuous, ContinuousCDF, Normal},
    statrs::statistics::Distribution,
};

#[cfg(any(feature = "bayesian"))]
use crate::{rand, rand::Rng};

#[cfg(any(feature = "bayesian"))]
#[macro_export]
macro_rules! build_optimizer {
    ($acquisition: tt) => {
        struct Opt{
            gauss_pr: GaussianProcess<Gaussian, ConstantPrior>,
            x: Vec<Vec<f64>>,
        }

        impl Opt {
            pub fn new(x: &Vec<Vec<f64>>, y: &Vec<f64>) -> Opt{
                Opt 
                {
                    gauss_pr: GaussianProcess::default(x.clone(), y.clone()),
                    x: x.clone(),
                }
            }
        }

        impl ArgminOp for Opt {
            type Param = Vec<f64>;
            // Type of the return value computed by the cost function
            type Output = f64;
            // Type of the Hessian. Can be `()` if not needed.
            type Hessian = Vec<f64>;
            // Type of the Jacobian. Can be `()` if not needed.
            type Jacobian = ();
            // Floating point precision
            type Float = f64;

            // Apply the cost function to a parameter `p`
            fn apply(&self, p: &Self::Param) -> Result<Self::Output, Error> {
                Ok($acquisition(&self.gauss_pr, &p.to_vec(), &self.x))
            }

            fn gradient(&self, p: &Self::Param) -> Result<Self::Param, Error> {
                Ok((*p).forward_diff(&|x| $acquisition(&self.gauss_pr, &x.to_vec(), &self.x)))
            }
        }
    };
}



#[cfg(any(feature = "bayesian"))]
#[macro_export]
macro_rules! bayesian_opt {
    (
        $init_population: tt,
        $costly_function: tt,
        $acquisition_function: tt,
        $gen_new_points: tt,
        $check_domain: tt,
        $n_iter: expr,
    ) => {{

        build_optimizer!(acquisition_function);

        let (mut x_init, mut y_init) = $init_population();

        if x_init.len() != y_init.len() {
            panic!("Input and output sizes are different!!")
        }

        let mut y_min = y_init[0];
        let mut y_index = 0;

        for i in 1..y_init.len() {
            if y_init[i] < y_min {
                y_min = y_init[i];
                y_index = i;
            }
        }

        let mut x_min = x_init[y_index].clone();
        let mut optimal_ei = 0.;

        for i in 0..$n_iter {
            println!("-----\nIteration {i}");
            let mut min_ei = f64::MAX/100.;
            let mut optimal: Vec<f64> = Vec::new();

            //check how to manage his wisi
            let gauss_pr = GaussianProcess::default(x_init.clone(), y_init.clone());
            let trial_x = $gen_new_points(&x_init, &gauss_pr);
            // let (mut x_next, ei) = $acquisition_function(&x_init, trial_x, gauss_pr);

            let mut min = f64::MAX;
            let mut x_next:Vec<f64> = Vec::new();
            for i in 0..trial_x.len() {
                let acquisition = Opt::new(&x_init, &y_init);

                let linesearch:MoreThuenteLineSearch<Vec<f64>, f64> = MoreThuenteLineSearch::new().c(1e-4, 0.9).unwrap();

                // Set up solver
                let solver: LBFGS<_,Vec<f64>,f64>= LBFGS::new(linesearch, 7);
            
                // Run solver
                let res = Executor::new(acquisition, solver, trial_x[i].clone())
                    // .add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
                    .max_iters(50)
                    .run();
            

                let res = res.expect("Something goes wrong with NelderMead algo");
                let ei = res.state().get_best_cost();
                if ei < min {
                    min = ei;
                    x_next = res.state.get_best_param();
                }
            }

            // let x_next = optimization(&gauss_pr, &x_init, &trial_x);

            // let solver = NelderMead::new().with_initial_params(trial_x);
            // let res = Executor::new(acquisition, solver, vec![])
            //     //.add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
            //     .max_iters(100)
            //     .run();

            // let res = res.expect("Something goes wrong with NelderMead algo");
            // let mut x_next = res.state().get_best_param();
            // let ei = res.state().get_best_cost();
            // println!("opt {:?}, value {}", &x_next, ei);

            //evaluation od new val
            $check_domain(&mut x_next);
            let y_next = $costly_function(&x_next);
            println!("New point {:?}", &x_next);
            println!("f(x) = {y_next}");
            x_init.push(x_next.clone());
            y_init.push(y_next);

            if y_next < y_min {
                y_min = y_next;
                x_min = x_next;
                optimal_ei = min;
            }
        }

        (x_min, y_min)
    }};
}

#[cfg(any(feature = "bayesian"))]
#[macro_export]
macro_rules! bayesian_opt_base {
    (
        $x_init: expr,
        $y_init: expr,
        $costly_function: tt,
        $n_iter: expr,
        $batch_size: expr,
        $scale: expr

    ) => {{
        let mut y_init: Vec<f64> = $y_init;
        let mut x_init: Vec<Vec<f64>> = $x_init;
        //find max initial x sample
        let mut y_min = f64::MAX;
        let mut y_index = 0;
        for (i, y) in y_init.clone().iter_mut().enumerate() {
            if *y < y_min {
                y_min = *y;
                y_index = i;
            }
        }
        let mut x_min = x_init[y_index].clone();
        //optimal expected improvement
        let mut optimal_ei = 0.;

        for i in 0..$n_iter {
            println!("-----\nIteration {i}");
            //init Gaussian Process Regressio
            // got, using acquisition function a new point
            // get_next_point(&mut gauss_pr, &x_init, $batch_size);

            let (mut x_next, ei) = get_next_point_base(&x_init, &y_init, $batch_size, $scale);
            //evaluation od new val
            let y_next = $costly_function(&x_next);
            println!("New point {:?}", &x_next);
            println!("f(x) = {y_next}");

            x_init.push(x_next.clone());
            y_init.push(y_next);

            if y_next < y_min {
                y_min = y_next;
                x_min = x_next;
                optimal_ei = ei;
            }
        }

        (x_min, y_min)
    }};
}
#[cfg(any(feature = "bayesian"))]
struct OptAcquisition {
    gauss_pr: GaussianProcess<Gaussian, ConstantPrior>,
    x: Vec<Vec<f64>>,
}

#[cfg(any(feature = "bayesian"))]
impl ArgminOp for OptAcquisition {
    type Param = Vec<f64>;
    // Type of the return value computed by the cost function
    type Output = f64;
    // Type of the Hessian. Can be `()` if not needed.
    type Hessian = Vec<Vec<f64>>;
    // Type of the Jacobian. Can be `()` if not needed.
    type Jacobian = ();
    // Floating point precision
    type Float = f64;

    // Apply the cost function to a parameter `p`
    fn apply(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        Ok(acquisition_function_base(
            &self.gauss_pr,
            &p.to_vec(),
            &self.x,
        ))
    }
}

#[cfg(any(feature = "bayesian"))]
#[inline(always)]
///Expected Improvement algorithm
pub fn acquisition_function_base(
    gauss_pr: &GaussianProcess<Gaussian, ConstantPrior>,
    x_new: &Vec<f64>,
    x_init: &Vec<Vec<f64>>,
) -> f64 {
    let mean_y_new: f64;
    let mut sigma_y_new: f64;

    mean_y_new = gauss_pr.predict(x_new);
    sigma_y_new = gauss_pr.predict_variance(x_new); //standard deviation
    sigma_y_new = sigma_y_new.sqrt();
    if sigma_y_new == 0. {
        return 0.;
    }

    let mut mean_y: Vec<f64> = Vec::with_capacity(x_init.len());
    for x in x_init {
        mean_y.push(gauss_pr.predict(x));
    }

    // let mean_y_max = mean_y.iter().max().expect("Something goes wrong, no input variables");
    let mut mean_y_max = f64::MIN;
    for m_y in &mean_y {
        if *m_y > mean_y_max {
            mean_y_max = *m_y;
        }
    }

    let z = (mean_y_new - mean_y_max) / sigma_y_new;
    let normal = Normal::new(0.0, 1.0).unwrap();
    let z_cfd = normal.cdf(z);
    let z_pdf = normal.pdf(z);
    (mean_y_new - mean_y_max) * z_cfd + sigma_y_new * z_pdf
}

#[cfg(any(feature = "bayesian"))]
#[inline(always)]
pub fn get_next_point_base(
    x: &Vec<Vec<f64>>,
    y: &Vec<f64>,
    batch_size: usize,
    scale: f64,
) -> (Vec<f64>, f64) {
    let mut min_ei = f64::MAX;
    let mut optimal: Vec<f64> = Vec::new();

    let gauss_pr = GaussianProcess::default(x.clone(), y.clone());

    let trial_x: Vec<Vec<f64>> = (0..batch_size)
        .into_iter()
        .map(|i| {
            let mut t_x = Vec::with_capacity(x[0].len());
            let mut rng = rand::thread_rng();
            for _ in 0..x[0].len() {
                t_x.push(rng.gen_range(-1.0..=1.0) * scale);
            }
            t_x
        })
        .collect();

    let acquisition = OptAcquisition {
        // gauss_pr: GaussianProcess::default(x.clone(), y.clone()),
        gauss_pr,
        x: x.clone(),
    };

    let solver = NelderMead::new().with_initial_params(trial_x);

    let res = Executor::new(acquisition, solver, vec![])
        //.add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
        .max_iters(100)
        .run();

    let res = res.expect("Something goes wrong with NelderMead algo");

    optimal = res.state().get_best_param();
    min_ei = res.state().get_best_cost();
    (optimal, min_ei)
}
