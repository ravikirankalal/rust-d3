// Axis constructor functions

use super::axis_structs::Axis;
use super::orientation::AxisOrientation;

pub fn axis_bottom<S>(scale: S) -> Axis<S> {
    Axis::new(scale, AxisOrientation::Bottom)
}

pub fn axis_top<S>(scale: S) -> Axis<S> {
    Axis::new(scale, AxisOrientation::Top)
}
pub fn axis_right<S>(scale: S) -> Axis<S> {
    Axis::new(scale, AxisOrientation::Right)
}
pub fn axis_left<S>(scale: S) -> Axis<S> {
    Axis::new(scale, AxisOrientation::Left)
}
