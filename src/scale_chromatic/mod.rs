//! d3-scale-chromatic: Color schemes and interpolators

pub mod categorical;
pub mod sequential;
pub mod diverging;
pub mod perceptual;

pub use categorical::*;
pub use sequential::*;
pub use diverging::*;
pub use perceptual::{inferno, magma, plasma, cividis, turbo, cubehelix};

#[cfg(test)]
mod perceptual_tests;
