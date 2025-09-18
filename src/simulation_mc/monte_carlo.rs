use ndarray::prelude::*;
use ndarray::{Array1, Array2};
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Normal;

pub fn black_scholes_merton_mc(S0: f64, M: f64, T: f64, I: f64, sigma: f64, r: f64) -> Array2<f64> {
    let a = (M + 1.0) as usize;
    let dt = T / M;
    let mut rng = Normal::new(0.0, 1.0).expect("Normal should be in range [0.0, 1.0)");

    let mut S: Array2<f64> = Array2::zeros((a, I as usize));

    S.row_mut(0).fill(S0);

    for t in 1..a {
        let z: Array1<f64> = standard_normal(I as usize);

        for i in 0..I as usize {
            S[[t, i]] =
                S[[t - 1, i]] * ((r - 0.5 * sigma.powi(2)) * dt + sigma * dt.sqrt() * z[i]).exp()
        }
    }

    S
}

fn standard_normal(size: usize) -> Array1<f64> {
    let mut rng = rand::thread_rng();
    let mut normal = Normal::new(0.0, 1.0).expect("Normal distribution should be valid");

    Array1::from_shape_fn(size, |_| normal.sample(&mut rng))
}
