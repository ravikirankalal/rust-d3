// AxisRenderable implementation for Axis<ScaleLinear>
use super::axis_structs::{Axis, GridStyle};
use super::orientation::AxisOrientation;
use super::util::TransformBuilder;

impl super::axis_renderable::AxisRenderable for Axis<crate::scale::ScaleLinear> {
    fn render(&self, selection: &mut crate::selection::Selection) {
        // Get existing transform if any
        let existing_transform = selection.get_attr("transform");
        
        // Apply offset for crisp lines based on orientation using TransformBuilder
        let transform = TransformBuilder::with_existing(existing_transform.clone())
            .translate(match self.orientation {
                AxisOrientation::Bottom | AxisOrientation::Top => self.offset,
                AxisOrientation::Left | AxisOrientation::Right => 0.0,
            }, match self.orientation {
                AxisOrientation::Bottom | AxisOrientation::Top => 0.0,
                AxisOrientation::Left | AxisOrientation::Right => self.offset,
            })
            .build();
        
        // Always set the transform attribute to mirror D3 behavior
        selection.attr("transform", &transform);
        
        let ticks = self.ticks();
        let range = self.scale.range();
        
        // Use raw range values to match D3.js exactly - no offset applied here
        let range0 = range[0];
        let range1 = range[1];
        
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

        // Draw domain line using scale.range() for all orientations
        super::axis_common::draw_domain_line(self, selection, range0, range1);
        
        // Draw ticks and labels for all orientations
        super::axis_common::draw_ticks_and_labels(self, selection, &ticks);
        
        // Draw minor ticks if enabled
        if let Some(ref minor_ticks) = self.minor_ticks {
            super::axis_common::draw_minor_ticks(self, selection, minor_ticks);
        }

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

