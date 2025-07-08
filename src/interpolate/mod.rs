//! d3-interpolate: Interpolators for numbers, colors, arrays, objects, strings, etc.

pub mod number;
pub mod array;
pub mod string;
pub mod rgb;
pub mod hsl;

pub use number::interpolate_number;
pub use array::interpolate_array;
pub use string::interpolate_string;
pub use rgb::{interpolate_rgb, hex_to_hsl, hsl_to_hex};
pub use hsl::interpolate_hsl;

#[cfg(test)]
mod tests;
