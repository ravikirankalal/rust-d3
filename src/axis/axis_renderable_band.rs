// AxisRenderable implementation for Axis<ScaleBand<T>>
use super::axis_structs::Axis;
use super::axis_structs::GridStyle;
use super::orientation::AxisOrientation;

// Only keep this implementation here, remove any duplicate impls from axis_renderable.rs
impl<T: Clone + PartialEq + ToString> super::axis_renderable::AxisRenderable
    for Axis<crate::scale::ScaleBand<T>>
{
    fn render(&self, selection: &mut crate::selection::Selection) {
        // Apply offset for crisp lines based on orientation
        let transform = match self.orientation {
            AxisOrientation::Bottom | AxisOrientation::Top => {
                format!("translate({},0)", self.offset)
            }
            AxisOrientation::Left | AxisOrientation::Right => {
                format!("translate(0,{})", self.offset)
            }
        };
        selection.attr("transform", &transform);
        
        let ticks = self.ticks();
        let range = self.scale.range();
        let _range0 = range[0] + self.offset;
        let _range1 = range[1] + self.offset;
        
        // Draw grid lines if enabled
        if self.grid {
            let style = self.grid_style.clone().unwrap_or(GridStyle {
                color: "#ccc".to_string(),
                width: 1.0,
                dasharray: None,
            });
            for tick in &ticks {
                match self.orientation {
                    AxisOrientation::Bottom | AxisOrientation::Top => {
                        let mut line = selection.append("line");
                        line.attr("x1", &tick.position.to_string())
                            .attr("x2", &tick.position.to_string())
                            .attr("y1", "0")
                            .attr("y2", "-100%")
                            .attr("stroke", &style.color)
                            .attr("stroke-width", &style.width.to_string())
                            .attr("class", "grid");
                        if let Some(dash) = &style.dasharray {
                            line.attr("stroke-dasharray", dash);
                        }
                    }
                    AxisOrientation::Left | AxisOrientation::Right => {
                        let mut line = selection.append("line");
                        line.attr("x1", "0")
                            .attr("x2", "100%")
                            .attr("y1", &tick.position.to_string())
                            .attr("y2", &tick.position.to_string())
                            .attr("stroke", &style.color)
                            .attr("stroke-width", &style.width.to_string())
                            .attr("class", "grid");
                        if let Some(dash) = &style.dasharray {
                            line.attr("stroke-dasharray", dash);
                        }
                    }
                }
            }
        }
        // // Draw domain line using scale.range() for all orientations
        // self.draw_domain_line(selection, range0, range1);
        
        // // Draw ticks and labels for all orientations
        // self.draw_ticks_and_labels(selection, &ticks);
        
        // // Draw minor ticks if enabled
        // if let Some(ref minor_ticks) = self.minor_ticks {
        //     self.draw_minor_ticks(selection, minor_ticks);
        // }
        
        // Draw axis title if set
        if let Some(ref title) = self.title {
            match self.orientation {
                AxisOrientation::Bottom => {
                    selection
                        .append("text")
                        .attr("x", "50%")
                        .attr("y", "40")
                        .attr("text-anchor", "middle")
                        .attr("class", "axis-title")
                        .text(title);
                }
                AxisOrientation::Left => {
                    selection
                        .append("text")
                        .attr("x", "-40")
                        .attr("y", "50%")
                        .attr("text-anchor", "middle")
                        .attr("class", "axis-title")
                        .text(title);
                }
                _ => {}
            }
        }
    }
}
