// Interpolate module root

mod interpolate;

pub use interpolate::{interpolate, interpolate_array, interpolate_round, interpolate_rgb, interpolate_hsl, interpolate_object};
pub use crate::interpolate::interpolate::{interpolate_number, interpolate_string};
