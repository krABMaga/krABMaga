#[cfg(test)]
#[cfg(any(feature = "bayesian"))]
use {krabmaga::bayesian_search, krabmaga::explore::bayesian::*};

#[cfg(any(feature = "bayesian"))]
#[test]
fn bayesian() {
    let (x, y) = bayesian_search!(init_parameters, objective_square, 2, 20, 500, 10.);

    println!("---\nFinal res: Point {:?}, val {y}", x);
    assert!(x[0].abs() < 1. && x[1].abs() < 1.);
}

#[cfg(any(feature = "bayesian"))]
#[test]
fn bayesian2() {
    let (x, y) = bayesian_search!(init_parameters, objective_square, generate_samples, 20);

    println!("---\nFinal res: Point {:?}, val {y}", x);
    assert!(x[0].abs() < 1. && x[1].abs() < 1.);
}

#[cfg(any(feature = "bayesian"))]
fn init_parameters() -> Vec<Vec<f64>> {
    vec![vec![-2., -2.], vec![8., 1.], vec![-1., 5.], vec![4., -2.]]
}

#[cfg(any(feature = "bayesian"))]
pub fn generate_samples(_x_values: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let batch_size = 500;
    let num_params = 2;
    (0..batch_size)
        .into_iter()
        .map(|_| {
            let mut t_x = Vec::with_capacity(num_params);
            let mut rng = rand::thread_rng();
            for _ in 0..num_params {
                t_x.push(rand::Rng::gen_range(&mut rng, -10.0..=10.0));
            }
            t_x
        })
        .collect()
}

#[cfg(any(feature = "bayesian"))]
fn objective_square(x: &Vec<f64>) -> f64 {
    let x1 = x[0];
    let x2 = x[1];

    let total = (x1).powf(2.) + (x2).powf(2.);

    -1. * total
}
