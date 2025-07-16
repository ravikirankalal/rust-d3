// AxisRenderable implementation for Axis<ScaleTime>
use super::axis_structs::Axis;
use super::axis_structs::GridStyle;
use super::orientation::AxisOrientation;

impl super::axis_renderable::AxisRenderable for Axis<crate::scale::ScaleTime> {
    fn render(&self, selection: &mut crate::selection::Selection) {
        // Get existing transform if any
        let existing_transform = selection.get_attr("transform");
        
        // Apply half-pixel offset for crisp lines based on orientation
        let offset_transform = match self.orientation {
            AxisOrientation::Bottom | AxisOrientation::Top => {
                format!("translate({},0)", self.offset)
            }
            AxisOrientation::Left | AxisOrientation::Right => {
                format!("translate(0,{})", self.offset)
            }
        };
        
        // Combine existing transform with offset
        let final_transform = match &existing_transform {
            Some(existing) => {
                if self.offset != 0.0 {
                    format!("{} {}", existing, offset_transform)
                } else {
                    existing.clone()
                }
            }
            None => {
                if self.offset != 0.0 {
                    offset_transform
                } else {
                    String::new()
                }
            }
        };
        
        if !final_transform.is_empty() {
            selection.attr("transform", &final_transform);
        }
        let ticks = self.ticks();
        let range = self.scale.range();
        let range0 = range[0] + self.offset;
        let range1 = range[1] + self.offset;
        
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
        self.draw_domain_line(selection, range0, range1);
        
        // Draw ticks and labels for all orientations
        self.draw_ticks_and_labels(selection, &ticks);
        
        // Draw minor ticks if enabled
        if let Some(ref minor_ticks) = self.minor_ticks {
            self.draw_minor_ticks(selection, minor_ticks);
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

// Helper methods for time axis rendering
impl Axis<crate::scale::ScaleTime> {
    fn draw_domain_line(&self, selection: &mut crate::selection::Selection, range0: f64, range1: f64) {
        let axis_line_style = self.axis_line_style.clone().unwrap_or_default();
        
        let mut domain_line = selection.append("line");
        domain_line.attr("class", "domain");
        
        // Apply axis line styling
        domain_line.attr("stroke", &axis_line_style.color)
            .attr("stroke-width", &axis_line_style.width.to_string());
        
        if let Some(dash) = &axis_line_style.dasharray {
            domain_line.attr("stroke-dasharray", dash);
        }
        
        // Set domain line coordinates using scale.range()
        match self.orientation {
            AxisOrientation::Bottom | AxisOrientation::Top => {
                domain_line.attr("x1", &range0.to_string())
                    .attr("x2", &range1.to_string())
                    .attr("y1", "0")
                    .attr("y2", "0");
            }
            AxisOrientation::Left | AxisOrientation::Right => {
                domain_line.attr("x1", "0")
                    .attr("x2", "0")
                    .attr("y1", &range0.to_string())
                    .attr("y2", &range1.to_string());
            }
        }
    }
    
    fn draw_ticks_and_labels(&self, selection: &mut crate::selection::Selection, ticks: &[super::ticks::Tick]) {
        let k = match self.orientation {
            AxisOrientation::Top | AxisOrientation::Left => -1.0,
            AxisOrientation::Bottom | AxisOrientation::Right => 1.0,
        };
        
        let spacing = self.tick_size_inner.max(0.0) + self.tick_padding;
        
        for tick in ticks {
            // Draw tick lines
            let mut tick_line = selection.append("line");
            tick_line.attr("class", "tick")
                .attr("stroke", "currentColor");
            
            // Draw tick labels
            let mut label = selection.append("text");
            label.attr("fill", "currentColor")
                .attr("font-size", "10px")
                .attr("font-family", "sans-serif")
                .text(&tick.label);
            
            // Apply tick label styling if set
            if let Some(ref style) = self.tick_label_style {
                label.attr("font-family", &style.font)
                    .attr("fill", &style.color);
                if let Some(_padding) = style.padding {
                    // Additional padding logic could be applied here
                }
            }
            
            match self.orientation {
                AxisOrientation::Bottom => {
                    tick_line.attr("x1", &(tick.position + self.offset).to_string())
                        .attr("x2", &(tick.position + self.offset).to_string())
                        .attr("y1", "0")
                        .attr("y2", &self.tick_size_inner.to_string());
                    
                    label.attr("x", &(tick.position + self.offset).to_string())
                        .attr("y", &(k * spacing).to_string())
                        .attr("text-anchor", "middle")
                        .attr("dy", "0.71em");
                }
                AxisOrientation::Top => {
                    tick_line.attr("x1", &(tick.position + self.offset).to_string())
                        .attr("x2", &(tick.position + self.offset).to_string())
                        .attr("y1", "0")
                        .attr("y2", &(k * self.tick_size_inner).to_string());
                    
                    label.attr("x", &(tick.position + self.offset).to_string())
                        .attr("y", &(k * spacing).to_string())
                        .attr("text-anchor", "middle")
                        .attr("dy", "0em");
                }
                AxisOrientation::Left => {
                    tick_line.attr("x1", "0")
                        .attr("x2", &(k * self.tick_size_inner).to_string())
                        .attr("y1", &(tick.position + self.offset).to_string())
                        .attr("y2", &(tick.position + self.offset).to_string());
                    
                    label.attr("x", &(k * spacing).to_string())
                        .attr("y", &(tick.position + self.offset).to_string())
                        .attr("text-anchor", "end")
                        .attr("dy", "0.32em");
                }
                AxisOrientation::Right => {
                    tick_line.attr("x1", "0")
                        .attr("x2", &(k * self.tick_size_inner).to_string())
                        .attr("y1", &(tick.position + self.offset).to_string())
                        .attr("y2", &(tick.position + self.offset).to_string());
                    
                    label.attr("x", &(k * spacing).to_string())
                        .attr("y", &(tick.position + self.offset).to_string())
                        .attr("text-anchor", "start")
                        .attr("dy", "0.32em");
                }
            }
            
            // Apply tick label rotation if set
            if let Some(angle) = self.tick_label_angle {
                if angle != 0.0 {
                    let transform = format!("rotate({} {} {})", 
                        angle, 
                        tick.position + self.offset, 
                        match self.orientation {
                            AxisOrientation::Bottom => k * spacing,
                            AxisOrientation::Top => k * spacing,
                            AxisOrientation::Left | AxisOrientation::Right => tick.position + self.offset,
                        }
                    );
                    label.attr("transform", &transform);
                }
            }
        }
    }
    
    fn draw_minor_ticks(&self, selection: &mut crate::selection::Selection, minor_ticks: &[f64]) {
        let minor_size = self.minor_tick_size.unwrap_or(self.tick_size_inner * 0.5);
        
        for &tick_value in minor_ticks {
            // For time scale, we need to convert timestamp to datetime for scaling
            let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(
                (tick_value / 1000.0) as i64,
                ((tick_value % 1000.0) * 1_000_000.0) as u32,
            ).unwrap().naive_utc();
            
            let position = self.scale.scale(datetime);
            
            let mut minor_tick = selection.append("line");
            minor_tick.attr("class", "minor-tick")
                .attr("stroke", "currentColor")
                .attr("stroke-width", "0.5");
            
            match self.orientation {
                AxisOrientation::Bottom => {
                    minor_tick.attr("x1", &(position + self.offset).to_string())
                        .attr("x2", &(position + self.offset).to_string())
                        .attr("y1", "0")
                        .attr("y2", &minor_size.to_string());
                }
                AxisOrientation::Top => {
                    minor_tick.attr("x1", &(position + self.offset).to_string())
                        .attr("x2", &(position + self.offset).to_string())
                        .attr("y1", "0")
                        .attr("y2", &(-minor_size).to_string());
                }
                AxisOrientation::Left => {
                    minor_tick.attr("x1", "0")
                        .attr("x2", &minor_size.to_string())
                        .attr("y1", &(position + self.offset).to_string())
                        .attr("y2", &(position + self.offset).to_string());
                }
                AxisOrientation::Right => {
                    minor_tick.attr("x1", "0")
                        .attr("x2", &(-minor_size).to_string())
                        .attr("y1", &(position + self.offset).to_string())
                        .attr("y2", &(position + self.offset).to_string());
                }
            }
        }
    }
}
