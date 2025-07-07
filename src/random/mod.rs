// D3 random module for Rust D3
// Provides random number utilities similar to d3-random.

use rand::Rng;
use rand_distr::LogNormal;

// All major d3-random distributions implemented: random_uniform, random_normal, random_exponential, random_bernoulli, random_bates, random_irwin_hall, random_log_normal.
// See: https://github.com/d3/d3-random#api-reference

pub fn random_uniform(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn random_normal(mean: f64, stddev: f64) -> f64 {
    let mut rng = rand::rng();
    let normal = rand_distr::Normal::new(mean, stddev).unwrap();
    rng.sample(normal)
}

pub fn random_exponential(lambda: f64) -> f64 {
    let mut rng = rand::rng();
    let exp = rand_distr::Exp::new(lambda).unwrap();
    rng.sample(exp)
}

pub fn random_bernoulli(p: f64) -> bool {
    let mut rng = rand::rng();
    rng.random_bool(p)
}

pub fn random_bates(n: usize) -> f64 {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random::<f64>()).sum::<f64>() / n as f64
}

pub fn random_irwin_hall(n: usize) -> f64 {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random::<f64>()).sum::<f64>()
}

pub fn random_log_normal(mu: f64, sigma: f64) -> f64 {
    let mut rng = rand::rng();
    let dist = LogNormal::new(mu, sigma).unwrap();
    rng.sample(dist)
}
