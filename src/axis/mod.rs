// d3-axis parity root
// Implements Axis, AxisOrientation, and tick generation for D3-like axes
//
// This module provides comprehensive axis functionality including:
// - Tick generation for different scale types (linear, logarithmic, time-based)
// - Customizable tick formatting with locale support
// - Axis rendering with grid lines, titles, and styling options
// - Context-aware formatting decisions based on data range and scale type
//
// Key design decisions:
// - Tick generation uses D3-compatible algorithms for consistent spacing
// - Formatting defaults to D3's ".6g" format but supports custom formatters
// - Time scales use context-aware format patterns based on tick intervals
// - Axis styling is modular with separate style structs for different components

pub mod axis_common;
pub mod axis_constructors;
pub mod axis_impl;
pub mod axis_renderable;
pub mod axis_renderable_band;
pub mod axis_renderable_linear;
pub mod axis_renderable_log;
pub mod axis_renderable_point;
pub mod axis_renderable_time;
pub mod axis_structs;
pub mod axis_ticks;
pub mod orientation;
pub mod ticks;
pub mod util;

pub use axis_constructors::{axis_bottom, axis_left, axis_right, axis_top};
pub use axis_renderable::AxisRenderable;
pub use axis_structs::{Axis, AxisLayout, GridStyle, TitleStyle, TickLabelStyle, AxisLineStyle};
pub use orientation::AxisOrientation;
pub use ticks::{Tick, TickFormat};
pub use util::{default_offset, default_device_pixel_ratio, crisp_offset, effective_offset, TransformBuilder};
// All AxisRenderable implementations are now in their own files for each scale type.
// This ensures 100% D3 parity and modularity for the axis module.
