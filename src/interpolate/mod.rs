//! d3-interpolate: Interpolators for numbers, colors, arrays, objects, strings, etc.

pub mod array;
pub mod hsl;
pub mod number;
pub mod rgb;
pub mod string;

pub use array::interpolate_array;
pub use hsl::interpolate_hsl;
pub use number::interpolate_number;
pub use rgb::{hex_to_hsl, hsl_to_hex, interpolate_rgb};
pub use string::interpolate_string;

// Piecewise interpolation (array of stops)
pub fn interpolate_piecewise<T, F>(interpolator: F, values: &[T], t: f64) -> T
where
    T: Clone,
    F: Fn(&T, &T, f64) -> T,
{
    let n = values.len();
    if n == 0 {
        panic!("No values for piecewise interpolation");
    } else if n == 1 {
        return values[0].clone();
    }
    let scaled = t * (n as f64 - 1.0);
    let i = scaled.floor() as usize;
    let frac = scaled - i as f64;
    if i + 1 >= n {
        return values[n - 1].clone();
    }
    interpolator(&values[i], &values[i + 1], frac)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_piecewise_number() {
        let arr = [0.0, 10.0, 20.0];
        let interp = |a: &f64, b: &f64, t: f64| a + (b - a) * t;
        assert_eq!(interpolate_piecewise(interp, &arr, 0.0), 0.0);
        assert_eq!(interpolate_piecewise(interp, &arr, 0.5), 10.0);
        assert_eq!(interpolate_piecewise(interp, &arr, 1.0), 20.0);
    }
}
