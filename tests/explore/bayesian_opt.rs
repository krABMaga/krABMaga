#[cfg(any(feature = "bayesian"))]
use std::{
    mem::MaybeUninit,
    sync::{Mutex, Once},
};
#[cfg(test)]
#[cfg(any(feature = "bayesian"))]
use {
    argmin::prelude::*,
    argmin::solver::linesearch::MoreThuenteLineSearch,
    argmin::solver::quasinewton::LBFGS,
    finitediff::FiniteDiff,
    friedrich::gaussian_process::GaussianProcess,
    friedrich::kernel::Gaussian,
    friedrich::prior::ConstantPrior,
    rust_ab::explore::bayesian_opt::*,
    rust_ab::*,
    rust_ab::{rand, Rng},
    statrs::distribution::{Continuous, ContinuousCDF, Normal},
};

#[cfg(any(feature = "bayesian"))]
#[test]
fn bayesian_base() {
    //init popultation method
    let x_init: Vec<Vec<f64>> = vec![vec![-2., -2.], vec![1., 1.], vec![-1., 1.], vec![4., -2.]];
    let mut y_init: Vec<f64> = Vec::with_capacity(x_init.len());

    for x in &x_init {
        y_init.push(costly_function(x));
    }

    let (x, y) = bayesian_opt_base!(x_init, y_init, costly_function, 10, 30, 10.);

    println!("---\nFinal res: Point {:?}, val {y}", x);
    assert!(x[0].abs() < 1. && x[1].abs() < 1.);
}

#[cfg(any(feature = "bayesian"))]
#[test]
fn bayesian_optimization() {
    let (x, y) = bayesian_opt!(
        init_population,
        costly_function,
        acquisition_function,
        get_points,
        domain_check,
        20,
    );

    println!("---\nFinal res: Point {:?}, val {y}", x);
    assert!(x[0].abs() < 1. && x[1].abs() < 1.);
}

#[cfg(any(feature = "bayesian"))]
fn init_population() -> (Vec<Vec<f64>>, Vec<f64>) {
    let x_init: Vec<Vec<f64>> = vec![vec![-2., -2.], vec![1., 1.], vec![-1., 0.5], vec![3., -2.]];
    let mut y_init: Vec<f64> = Vec::with_capacity(x_init.len());

    for x in &x_init {
        let y = costly_function(x);
        println!("{:?} scores: {}", x, y);
        y_init.push(y);
        // y_init.push(costly_function(x));
    }

    (x_init, y_init)
}

#[cfg(any(feature = "bayesian"))]
fn costly_function(x: &Vec<f64>) -> f64 {
    let x1 = x[0];
    let x2 = x[1];

    let total = (x1).powf(2.) + (x2).powf(2.);

    total
}

#[cfg(any(feature = "bayesian"))]
fn get_points(x: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let batch_size = 30;
    let scale = 10.;

    let _gauss_pr = get_instance(&vec![vec![0.0_f64]], &vec![0.0_f64])
        .gauss_pr
        .lock()
        .unwrap();

    let trial_x: Vec<Vec<f64>> = (0..batch_size)
        .into_iter()
        .map(|_| {
            let mut t_x = Vec::with_capacity(x[0].len());
            let mut rng = rand::thread_rng();
            for _ in 0..x[0].len() {
                t_x.push(rng.gen_range(-1.0..=1.0) * scale);
            }
            t_x
        })
        .collect();
    trial_x
}

#[cfg(any(feature = "bayesian"))]
#[inline(always)]
///Expected Improvement algorithm
pub fn acquisition_function(
    gauss_pr: &GaussianProcess<Gaussian, ConstantPrior>,
    x_new: &Vec<f64>,
    x_init: &Vec<Vec<f64>>,
) -> f64 {
    let mean_y_new: f64;
    let mut sigma_y_new: f64;

    mean_y_new = gauss_pr.predict(x_new);
    sigma_y_new = gauss_pr.predict_variance(x_new);
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
    let ei = (mean_y_new - mean_y_max) * z_cfd + sigma_y_new * z_pdf;
    // x_new[0] * x_new[1]
    // println!("EI {}");
    ei
}

#[cfg(any(feature = "bayesian"))]
pub fn domain_check(_new_x: &mut Vec<f64>) {}
