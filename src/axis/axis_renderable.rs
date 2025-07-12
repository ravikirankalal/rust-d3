// AxisRenderable trait and implementations for different scales

use super::axis_structs::Axis;
use super::orientation::AxisOrientation;
use super::ticks::Tick;

pub trait AxisRenderable {
    fn render(&self, selection: &mut crate::selection::Selection);
}

// Removed AxisRenderable impls for ScaleLinear, ScaleTime, ScaleLog, ScaleBand, ScalePoint from this file.
// These are now in axis_renderable_linear.rs, axis_renderable_time.rs, axis_renderable_log.rs, axis_renderable_band.rs, axis_renderable_point.rs respectively.
