//! d3-scale-chromatic: Color schemes and interpolators

pub mod categorical;
pub mod diverging;
pub mod perceptual;
pub mod sequential;

pub use categorical::*;
pub use diverging::*;
pub use perceptual::{cividis, cubehelix, inferno, magma, plasma, turbo};
pub use sequential::*;

#[cfg(test)]
mod perceptual_tests;
