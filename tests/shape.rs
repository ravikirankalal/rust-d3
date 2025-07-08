#[path = "shape/shape_curve.rs"]
mod shape_curve;
#[path = "shape/shape_symbol.rs"]
mod shape_symbol;
#[path = "shape/shape_symbol_output.rs"]
mod shape_symbol_output;

pub use shape_curve::shape_curve_tests::*;
pub use shape_symbol::shape_symbol_tests::*;
pub use shape_symbol_output::shape_symbol_output_tests::*;
