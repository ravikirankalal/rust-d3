// AxisRenderable implementation for Axis<ScalePoint<T>>
use super::axis_structs::{Axis, GridStyle};
use super::orientation::AxisOrientation;

impl<T: Clone + PartialEq + ToString> super::axis_renderable::AxisRenderable
    for Axis<crate::scale::ScalePoint<T>>
{
    fn render(&self, selection: &mut crate::selection::Selection) {
        if let Some(hook) = &self.on_render {
            hook();
        }
        let ticks = self.ticks();
        // Minor ticks
        if let Some(minor_ticks) = &self.minor_ticks {
            for &pos in minor_ticks {
                match self.orientation {
                    AxisOrientation::Bottom | AxisOrientation::Top => {
                        selection
                            .append("line")
                            .attr("x1", &pos.to_string())
                            .attr("x2", &pos.to_string())
                            .attr("y1", "0")
                            .attr("y2", &self.minor_tick_size.unwrap_or(3.0).to_string())
                            .attr("stroke", "#aaa")
                            .attr("class", "minor-tick");
                    }
                    AxisOrientation::Left | AxisOrientation::Right => {
                        selection
                            .append("line")
                            .attr("x1", "0")
                            .attr("x2", &self.minor_tick_size.unwrap_or(3.0).to_string())
                            .attr("y1", &pos.to_string())
                            .attr("y2", &pos.to_string())
                            .attr("stroke", "#aaa")
                            .attr("class", "minor-tick");
                    }
                }
            }
        }
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
        // Draw axis line
        match self.orientation {
            AxisOrientation::Bottom => {
                selection
                    .append("line")
                    .attr("x1", "0")
                    .attr("x2", "100%")
                    .attr("y1", "0")
                    .attr("y2", "0")
                    .attr("stroke", "black")
                    .attr("class", "domain");
            }
            AxisOrientation::Left => {
                selection
                    .append("line")
                    .attr("x1", "0")
                    .attr("x2", "0")
                    .attr("y1", "0")
                    .attr("y2", "100%")
                    .attr("stroke", "black")
                    .attr("class", "domain");
            }
            AxisOrientation::Top => {
                selection
                    .append("line")
                    .attr("x1", "0")
                    .attr("x2", "100%")
                    .attr("y1", "0")
                    .attr("y2", "0")
                    .attr("stroke", "black")
                    .attr("class", "domain");
            }
            AxisOrientation::Right => {
                selection
                    .append("line")
                    .attr("x1", "0")
                    .attr("x2", "0")
                    .attr("y1", "0")
                    .attr("y2", "100%")
                    .attr("stroke", "black")
                    .attr("class", "domain");
            }
        }
        match self.orientation {
            AxisOrientation::Bottom => {
                for tick in &ticks {
                    selection
                        .append("line")
                        .attr("x1", &tick.position.to_string())
                        .attr("x2", &tick.position.to_string())
                        .attr("y1", "0")
                        .attr("y2", &self.tick_size_inner.to_string())
                        .attr("stroke", "black")
                        .attr("class", "tick");
                    let mut text = selection.append("text");
                    text.attr("x", &tick.position.to_string())
                        .attr(
                            "y",
                            &format!("{}", self.tick_size_inner + self.tick_padding + 12.0),
                        )
                        .attr("text-anchor", "middle")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .attr("class", "tick-label");
                    if let Some(angle) = self.tick_label_angle {
                        text.attr(
                            "transform",
                            &format!(
                                "rotate({},{},{})",
                                angle,
                                tick.position,
                                self.tick_size_inner + self.tick_padding + 12.0
                            ),
                        );
                    }
                    if let Some(style) = &self.tick_label_style {
                        text.attr("font-family", &style.font)
                            .attr("fill", &style.color);
                        if let Some(pad) = style.padding {
                            text.attr("dy", &pad.to_string());
                        }
                    }
                    text.text(&tick.label);
                }
            }
            AxisOrientation::Left => {
                for tick in &ticks {
                    selection
                        .append("line")
                        .attr("x1", "0")
                        .attr("x2", &self.tick_size_inner.to_string())
                        .attr("y1", &tick.position.to_string())
                        .attr("y2", &tick.position.to_string())
                        .attr("stroke", "black")
                        .attr("class", "tick");
                    let mut text = selection.append("text");
                    text.attr(
                        "x",
                        &format!("{}", self.tick_size_inner + self.tick_padding + 2.0),
                    )
                    .attr("y", &tick.position.to_string())
                    .attr("text-anchor", "start")
                    .attr("font-size", "12px")
                    .attr("fill", "black")
                    .attr("font-family", "Arial, sans-serif")
                    .attr("class", "tick-label");
                    if let Some(angle) = self.tick_label_angle {
                        text.attr(
                            "transform",
                            &format!(
                                "rotate({},{},{})",
                                angle,
                                self.tick_size_inner + self.tick_padding + 2.0,
                                tick.position
                            ),
                        );
                    }
                    if let Some(style) = &self.tick_label_style {
                        text.attr("font-family", &style.font)
                            .attr("fill", &style.color);
                        if let Some(pad) = style.padding {
                            text.attr("dy", &pad.to_string());
                        }
                    }
                    text.text(&tick.label);
                }
            }
            AxisOrientation::Top => {
                for tick in &ticks {
                    selection
                        .append("line")
                        .attr("x1", &tick.position.to_string())
                        .attr("x2", &tick.position.to_string())
                        .attr("y1", &(-self.tick_size_inner).to_string())
                        .attr("y2", "0")
                        .attr("stroke", "black")
                        .attr("class", "tick");
                    let mut text = selection.append("text");
                    text.attr("x", &tick.position.to_string())
                        .attr(
                            "y",
                            &format!("{}", -(self.tick_size_inner + self.tick_padding)),
                        )
                        .attr("text-anchor", "middle")
                        .attr("font-size", "12px")
                        .attr("fill", "black")
                        .attr("font-family", "Arial, sans-serif")
                        .attr("class", "tick-label");
                    if let Some(angle) = self.tick_label_angle {
                        text.attr(
                            "transform",
                            &format!(
                                "rotate({},{},{})",
                                angle,
                                tick.position,
                                -(self.tick_size_inner + self.tick_padding)
                            ),
                        );
                    }
                    if let Some(style) = &self.tick_label_style {
                        text.attr("font-family", &style.font)
                            .attr("fill", &style.color);
                        if let Some(pad) = style.padding {
                            text.attr("dy", &pad.to_string());
                        }
                    }
                    text.text(&tick.label);
                }
            }
            AxisOrientation::Right => {
                for tick in &ticks {
                    selection
                        .append("line")
                        .attr("x1", "0")
                        .attr("x2", &(-self.tick_size_inner).to_string())
                        .attr("y1", &tick.position.to_string())
                        .attr("y2", &tick.position.to_string())
                        .attr("stroke", "black")
                        .attr("class", "tick");
                    let mut text = selection.append("text");
                    text.attr(
                        "x",
                        &format!("{}", -(self.tick_size_inner + self.tick_padding)),
                    )
                    .attr("y", &tick.position.to_string())
                    .attr("text-anchor", "end")
                    .attr("font-size", "12px")
                    .attr("fill", "black")
                    .attr("font-family", "Arial, sans-serif")
                    .attr("class", "tick-label");
                    if let Some(angle) = self.tick_label_angle {
                        text.attr(
                            "transform",
                            &format!(
                                "rotate({},{},{})",
                                angle,
                                -(self.tick_size_inner + self.tick_padding),
                                tick.position
                            ),
                        );
                    }
                    if let Some(style) = &self.tick_label_style {
                        text.attr("font-family", &style.font)
                            .attr("fill", &style.color);
                        if let Some(pad) = style.padding {
                            text.attr("dy", &pad.to_string());
                        }
                    }
                    text.text(&tick.label);
                }
            }
        }
        // Draw axis title if set
        if let Some(ref title) = self.title {
            match self.orientation {
                AxisOrientation::Bottom => {
                    let mut title_text = selection.append("text");
                    title_text
                        .attr("x", "50%")
                        .attr("y", "40")
                        .attr("text-anchor", "middle")
                        .attr("class", "axis-title")
                        .text(title);
                    if let Some(style) = &self.title_style {
                        title_text
                            .attr("font-family", &style.font)
                            .attr("fill", &style.color);
                        if let Some((x, y)) = style.position {
                            title_text
                                .attr("x", &x.to_string())
                                .attr("y", &y.to_string());
                        }
                    }
                }
                AxisOrientation::Left => {
                    let mut title_text = selection.append("text");
                    title_text
                        .attr("x", "-40")
                        .attr("y", "50%")
                        .attr("text-anchor", "middle")
                        .attr("class", "axis-title")
                        .text(title);
                    if let Some(style) = &self.title_style {
                        title_text
                            .attr("font-family", &style.font)
                            .attr("fill", &style.color);
                        if let Some((x, y)) = style.position {
                            title_text
                                .attr("x", &x.to_string())
                                .attr("y", &y.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(hook) = &self.on_render {
            hook();
        }
    }
}
