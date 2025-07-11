// AxisRenderable implementation for Axis<ScaleLog>
use super::axis_structs::Axis;
use super::orientation::AxisOrientation;
use super::ticks::Tick;

impl super::axis_renderable::AxisRenderable for Axis<crate::scale::ScaleLog> {
    fn render(&self, selection: &mut crate::selection::Selection) {
        match self.orientation {
            AxisOrientation::Bottom => {
                let ticks = self.ticks();
                println!("[AxisRenderable::ScaleLog] Bottom axis ticks:");
                for tick in &ticks {
                    println!("  label: '{}' at position: {}", tick.label, tick.position);
                }
                if let (Some(first), Some(last)) = (ticks.first(), ticks.last()) {
                    selection.append("line")
                        .attr("x1", &first.position.to_string())
                        .attr("x2", &last.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", "0")
                        .attr("stroke", "black")
                        .attr("stroke-width", "1")
                        .attr("class", "domain");
                }
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
                let ticks = self.ticks();
                println!("[AxisRenderable::ScaleLog] Left axis ticks:");
                for tick in &ticks {
                    println!("  label: '{}' at position: {}", tick.label, tick.position);
                }
                if let (Some(first), Some(last)) = (ticks.first(), ticks.last()) {
                    selection.append("line")
                        .attr("x1", "0")
                        .attr("x2", "0")
                        .attr("y1", &first.position.to_string())
                        .attr("y2", &last.position.to_string())
                        .attr("stroke", "black")
                        .attr("stroke-width", "1")
                        .attr("class", "domain");
                }
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
            _ => {}
        }
    }
}
