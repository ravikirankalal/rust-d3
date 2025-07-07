// D3 nice utility for Rust D3
// Extends a domain to nice round values.

pub fn nice(domain: (f64, f64), step: f64) -> (f64, f64) {
    let (mut min, mut max) = domain;
    min = (min / step).floor() * step;
    max = (max / step).ceil() * step;
    (min, max)
}

// (Unit tests moved to tests/integration.rs)
