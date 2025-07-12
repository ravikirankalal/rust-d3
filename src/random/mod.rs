//! d3-random: Random number generators (Rust port)
//
// This module aims to provide seeded and unseeded random number generators for common distributions.
// See: https://github.com/d3/d3-random

use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Normal, LogNormal, Exp, Distribution};

pub fn random_uniform() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn random_uniform_range(a: f64, b: f64) -> f64 {
    let mut rng = rand::rng();
    let x: f64 = rng.random();
    a + (b - a) * x
}

pub fn random_normal(mean: f64, stddev: f64) -> f64 {
    let mut rng = rand::rng();
    let normal = Normal::new(mean, stddev).unwrap();
    normal.sample(&mut rng)
}

pub fn random_lognormal(mean: f64, stddev: f64) -> f64 {
    let mut rng = rand::rng();
    let lognormal = LogNormal::new(mean, stddev).unwrap();
    lognormal.sample(&mut rng)
}

pub fn random_exponential(lambda: f64) -> f64 {
    let mut rng = rand::rng();
    let exp = Exp::new(lambda).unwrap();
    exp.sample(&mut rng)
}

// Optional: Add seeding support for reproducibility
pub fn random_uniform_seeded(seed: u64) -> f64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_uniform() {
        let x = random_uniform();
        assert!(x >= 0.0 && x < 1.0);
    }
    #[test]
    fn test_random_uniform_range() {
        let x = random_uniform_range(5.0, 10.0);
        assert!(x >= 5.0 && x < 10.0);
    }
    #[test]
    fn test_random_normal() {
        let x = random_normal(0.0, 1.0);
        // For a normal distribution, most values should be within [-5, 5]
        assert!(x > -5.0 && x < 5.0);
    }
    #[test]
    fn test_random_lognormal() {
        let x = random_lognormal(0.0, 0.25);
        assert!(x > 0.0);
    }
    #[test]
    fn test_random_exponential() {
        let x = random_exponential(1.0);
        assert!(x >= 0.0);
    }
    #[test]
    fn test_random_uniform_seeded() {
        let x1 = random_uniform_seeded(42);
        let x2 = random_uniform_seeded(42);
        assert_eq!(x1, x2);
    }
}
