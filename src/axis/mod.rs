// d3-axis parity root
// Implements Axis, AxisOrientation, and tick generation for D3-like axes

pub mod orientation;
pub mod ticks;
pub mod axis_structs;
pub mod axis_impl;
pub mod axis_renderable;
pub mod axis_renderable_linear;
pub mod axis_renderable_time;
pub mod axis_renderable_log;
pub mod axis_renderable_band;
pub mod axis_renderable_point;
pub mod axis_ticks;
pub mod axis_constructors;

pub use orientation::AxisOrientation;
pub use ticks::{Tick, TickFormat};
pub use axis_structs::{Axis, AxisLayout};
pub use axis_renderable::AxisRenderable;
pub use axis_constructors::{axis_bottom, axis_top, axis_right, axis_left};
// All AxisRenderable implementations are now in their own files for each scale type.
// This ensures 100% D3 parity and modularity for the axis module.
