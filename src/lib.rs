//! # Rust D3
//! 
//! A Rust library for D3-style data visualizations.
//! 
//! This library provides tools for creating various types of charts and data visualizations,
//! similar to the popular D3.js library but written in Rust.
//! 
//! ## Features
//! 
//! - Bar charts
//! - Line charts
//! - Pie charts
//! - Customizable scales and axes
//! - SVG output generation
//! 
//! ## Example
//! 
//! ```rust
//! use rust_d3::charts::{BarChart, Chart};
//! use rust_d3::data::DataPoint;
//! 
//! let data = vec![
//!     DataPoint::new("A", 10.0),
//!     DataPoint::new("B", 20.0),
//!     DataPoint::new("C", 15.0),
//! ];
//! 
//! let chart = BarChart::new()
//!     .width(400)
//!     .height(300)
//!     .data(data);
//! 
//! let svg = chart.render();
//! ```

pub mod data;
pub mod scales;
pub mod charts;
pub mod svg_utils;

pub use data::*;
pub use scales::*;
pub use charts::*;