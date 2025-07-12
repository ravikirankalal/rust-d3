// AxisRenderable implementation for Axis<ScaleBand<T>>
use super::axis_structs::Axis;
use super::orientation::AxisOrientation;
use super::ticks::Tick;

// Only keep this implementation here, remove any duplicate impls from axis_renderable.rs
impl<T: Clone + PartialEq + ToString> super::axis_renderable::AxisRenderable for Axis<crate::scale::ScaleBand<T>> {
    fn render(&self, selection: &mut crate::selection::Selection) {
        let ticks = self.ticks();
        match self.orientation {
            AxisOrientation::Bottom => {
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", &tick.position.to_string())
                        .attr("x2", &tick.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", &self.tick_size_inner.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &tick.position.to_string())
                        .attr("y", &format!("{}", self.tick_size_inner + self.tick_padding + 12.0))
                        .attr("text-anchor", "middle")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            AxisOrientation::Left => {
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", &self.tick_size_inner.to_string())
                        .attr("y1", &tick.position.to_string())
                        .attr("y2", &tick.position.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &format!("{}", self.tick_size_inner + self.tick_padding + 2.0))
                        .attr("y", &tick.position.to_string())
                        .attr("text-anchor", "start")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            AxisOrientation::Top => {
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", &tick.position.to_string())
                        .attr("x2", &tick.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", &(-self.tick_size_inner).to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &tick.position.to_string())
                        .attr("y", &format!("{}", -(self.tick_size_inner + self.tick_padding)))
                        .attr("text-anchor", "middle")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
            AxisOrientation::Right => {
                for tick in &ticks {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", &(-self.tick_size_inner).to_string())
                        .attr("y1", &tick.position.to_string())
                        .attr("y2", &tick.position.to_string())
                        .attr("stroke", "black");
                    selection.append("text")
                        .attr("x", &format!("{}", -(self.tick_size_inner + self.tick_padding)))
                        .attr("y", &tick.position.to_string())
                        .attr("text-anchor", "end")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .text(&tick.label);
                }
            }
        }
    }
}
