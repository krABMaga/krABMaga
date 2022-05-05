#[cfg(any(feature = "bayesian"))]
use std::{
    mem::MaybeUninit,
    sync::{Mutex, Once},
};
#[cfg(test)]
#[cfg(any(feature = "bayesian"))]
use {
    argmin::prelude::Error,
    argmin::prelude::*,
    argmin::solver::linesearch::MoreThuenteLineSearch,
    argmin::solver::quasinewton::LBFGS,
    finitediff::FiniteDiff,
    friedrich::gaussian_process::GaussianProcess,
    friedrich::kernel::Gaussian,
    friedrich::prior::ConstantPrior,
    krABMaga::explore::bayesian_opt::*,
    krABMaga::*,
    krABMaga::{rand, Rng},
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
fn costly_function(x: &Vec<f64>) -> f64 {
    let x1 = x[0];
    let x2 = x[1];

    let total = (x1).powf(2.) + (x2).powf(2.);

    total
}
