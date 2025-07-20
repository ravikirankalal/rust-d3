// Common axis rendering functions that work with any scale type
use super::axis_structs::Axis;
use super::orientation::AxisOrientation;
use super::ticks::Tick;
use super::util::crisp::effective_offset;
use crate::px;

/// Trait for scales that can be used in axis rendering with numeric minor ticks
pub trait ScaleForAxis {
    fn scale(&self, value: f64) -> f64;
}

/// Trait for time scales that require special handling for minor ticks
pub trait TimeScaleForAxis {
    fn scale_timestamp(&self, timestamp_millis: f64) -> f64;
}

/// Additional trait for scales that have range() method
pub trait ScaleWithRange {
    fn range(&self) -> [f64; 2];
}

/// Generic axis rendering functions that work with any scale implementing ScaleWithRange
pub fn draw_domain_line<S: ScaleWithRange>(
    axis: &Axis<S>, 
    selection: &mut crate::selection::Selection, 
    range0: f64, 
    range1: f64
) {
    let axis_line_style = axis.axis_line_style.clone().unwrap_or_default();
    
    // D3.js uses a <path> element for the domain line, not <line>
    let mut domain_path = selection.append("path");
    domain_path.attr("class", "domain");
    
    // Apply axis line styling
    domain_path.attr("stroke", &axis_line_style.color);
    
    if let Some(dash) = &axis_line_style.dasharray {
        domain_path.attr("stroke-dasharray", dash);
    }
    
    // Create SVG path string matching D3.js format exactly
    // D3.js uses .5 pixel offset for crisp lines in the path itself
    let path_d = match axis.orientation {
        AxisOrientation::Bottom | AxisOrientation::Top => {
            format!("M{},0H{}V0", px(range0 + 0.5), px(range1 + 0.5))
        }
        AxisOrientation::Left | AxisOrientation::Right => {
            format!("M0,{}V{}H0", px(range0 + 0.5), px(range1 + 0.5))
        }
    };
    
    domain_path.attr("d", &path_d);
}

/// Generic draw_ticks_and_labels that matches D3.js structure exactly
/// Each tick is wrapped in a <g> element with transform, containing <line> and <text>
pub fn draw_ticks_and_labels<S>(
    axis: &Axis<S>,
    selection: &mut crate::selection::Selection,
    ticks: &[Tick]
) {
    let k = match axis.orientation {
        AxisOrientation::Top | AxisOrientation::Left => -1.0,
        AxisOrientation::Bottom | AxisOrientation::Right => 1.0,
    };
    
    let spacing = axis.tick_size_inner.max(0.0) + axis.tick_padding;
    
    for tick in ticks {
        // Create a group element for each tick with transform (matching D3.js structure)
        let mut tick_group = selection.append("g");
        
        // D3.js uses exact integer coordinates in the transform, no offset applied here
        let transform = match axis.orientation {
            AxisOrientation::Bottom | AxisOrientation::Top => {
                format!("translate({}, 0)", tick.position as i32)
            }
            AxisOrientation::Left | AxisOrientation::Right => {
                format!("translate(0, {})", tick.position as i32)
            }
        };
        
        tick_group.attr("transform", &transform)
            .attr("class", "tick");
        
        // Draw tick line inside the group (no position needed, it's in the transform)
        let mut tick_line = tick_group.append("line");
        tick_line.attr("stroke", "currentColor");
        
        // Draw tick label inside the group
        let mut label = tick_group.append("text");
        label.attr("fill", "currentColor")
            .text(&tick.label);
        
        // Apply tick label styling if set
        if let Some(ref style) = axis.tick_label_style {
            label.attr("font-family", &style.font)
                .attr("fill", &style.color);
        }
        
        match axis.orientation {
            AxisOrientation::Bottom => {
                tick_line.attr("y2", &axis.tick_size_inner.to_string());
                
                label.attr("y", &(k * spacing).to_string())
                    .attr("text-anchor", "middle");
            }
            AxisOrientation::Top => {
                tick_line.attr("y2", &(k * axis.tick_size_inner).to_string());
                
                label.attr("y", &(k * spacing).to_string())
                    .attr("text-anchor", "middle");
            }
            AxisOrientation::Left => {
                tick_line.attr("x2", &(k * axis.tick_size_inner).to_string());
                
                label.attr("x", &(k * spacing).to_string())
                    .attr("text-anchor", "end");
            }
            AxisOrientation::Right => {
                tick_line.attr("x2", &(k * axis.tick_size_inner).to_string());
                
                label.attr("x", &(k * spacing).to_string())
                    .attr("text-anchor", "start");
            }
        }
        
        // Apply tick label rotation if set
        if let Some(angle) = axis.tick_label_angle {
            if angle != 0.0 {
                let rotation_transform = format!("rotate({})", angle);
                label.attr("transform", &rotation_transform);
            }
        }
    }
}

pub fn draw_minor_ticks<S: ScaleForAxis>(
    axis: &Axis<S>,
    selection: &mut crate::selection::Selection,
    minor_ticks: &[f64]
) {
    let minor_size = axis.minor_tick_size.unwrap_or(axis.tick_size_inner * 0.5);
    
    // Get crisp offset for pixel alignment - same as used for major ticks and domain
    let crisp_offset = effective_offset(None);
    
    for &tick_value in minor_ticks {
        let position = axis.scale.scale(tick_value);
        
        // Calculate aligned position - same algorithm as major ticks
        let aligned_position = (position + axis.offset + crisp_offset).round() - crisp_offset;
        
        let mut minor_tick = selection.append("line");
        minor_tick.attr("class", "minor-tick")
            .attr("stroke", "currentColor")
            .attr("stroke-width", "0.5");
        
        match axis.orientation {
            AxisOrientation::Bottom => {
                minor_tick.attr("x1", &px(aligned_position))
                    .attr("x2", &px(aligned_position))
                    .attr("y1", "0")
                    .attr("y2", &px(minor_size));
            }
            AxisOrientation::Top => {
                minor_tick.attr("x1", &px(aligned_position))
                    .attr("x2", &px(aligned_position))
                    .attr("y1", "0")
                    .attr("y2", &px(-minor_size));
            }
            AxisOrientation::Left => {
                minor_tick.attr("x1", "0")
                    .attr("x2", &px(minor_size))
                    .attr("y1", &px(aligned_position))
                    .attr("y2", &px(aligned_position));
            }
            AxisOrientation::Right => {
                minor_tick.attr("x1", "0")
                    .attr("x2", &px(-minor_size))
                    .attr("y1", &px(aligned_position))
                    .attr("y2", &px(aligned_position));
            }
        }
    }
}

/// Special version of draw_minor_ticks for time scales
pub fn draw_minor_ticks_time<S: TimeScaleForAxis>(
    axis: &Axis<S>,
    selection: &mut crate::selection::Selection,
    minor_ticks: &[f64]  // These are timestamps in milliseconds
) {
    let minor_size = axis.minor_tick_size.unwrap_or(axis.tick_size_inner * 0.5);
    
    // Get crisp offset for pixel alignment - same as used for major ticks and domain
    let crisp_offset = effective_offset(None);
    
    for &tick_value in minor_ticks {
        let position = axis.scale.scale_timestamp(tick_value);
        
        // Calculate aligned position - same algorithm as major ticks
        let aligned_position = (position + axis.offset + crisp_offset).round() - crisp_offset;
        
        let mut minor_tick = selection.append("line");
        minor_tick.attr("class", "minor-tick")
            .attr("stroke", "currentColor")
            .attr("stroke-width", "0.5");
        
        match axis.orientation {
            AxisOrientation::Bottom => {
                minor_tick.attr("x1", &px(aligned_position))
                    .attr("x2", &px(aligned_position))
                    .attr("y1", "0")
                    .attr("y2", &px(minor_size));
            }
            AxisOrientation::Top => {
                minor_tick.attr("x1", &px(aligned_position))
                    .attr("x2", &px(aligned_position))
                    .attr("y1", "0")
                    .attr("y2", &px(-minor_size));
            }
            AxisOrientation::Left => {
                minor_tick.attr("x1", "0")
                    .attr("x2", &px(minor_size))
                    .attr("y1", &px(aligned_position))
                    .attr("y2", &px(aligned_position));
            }
            AxisOrientation::Right => {
                minor_tick.attr("x1", "0")
                    .attr("x2", &px(-minor_size))
                    .attr("y1", &px(aligned_position))
                    .attr("y2", &px(aligned_position));
            }
        }
    }
}
