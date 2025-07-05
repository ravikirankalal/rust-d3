//! SVG utility functions for generating chart elements

use svg::node::element::{Circle, Group, Line, Path, Rectangle, Text};
use svg::Document;

/// SVG utilities for creating chart elements
pub struct SvgUtils;

impl SvgUtils {
    /// Create an SVG group
    pub fn group() -> Group {
        Group::new()
    }

    /// Create a rectangle
    pub fn rect(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
    }

    /// Create a circle
    pub fn circle(cx: f64, cy: f64, r: f64) -> Circle {
        Circle::new().set("cx", cx).set("cy", cy).set("r", r)
    }

    /// Create a line
    pub fn line(x1: f64, y1: f64, x2: f64, y2: f64) -> Line {
        Line::new()
            .set("x1", x1)
            .set("y1", y1)
            .set("x2", x2)
            .set("y2", y2)
    }

    /// Create a path element
    pub fn path(d: &str) -> Path {
        Path::new().set("d", d)
    }

    /// Create a text element
    pub fn text(x: f64, y: f64, content: &str) -> Text {
        Text::new()
            .set("x", x)
            .set("y", y)
            .add(svg::node::Text::new(content))
    }

    /// Create a basic SVG document with viewBox
    pub fn document(width: u32, height: u32) -> Document {
        Document::new()
            .set("viewBox", (0, 0, width, height))
            .set("width", width)
            .set("height", height)
            .set("xmlns", "http://www.w3.org/2000/svg")
    }

    /// Generate a path string for a line chart
    pub fn line_path(points: &[(f64, f64)]) -> String {
        if points.is_empty() {
            return String::new();
        }

        let mut path = format!("M {} {}", points[0].0, points[0].1);

        for point in points.iter().skip(1) {
            path.push_str(&format!(" L {} {}", point.0, point.1));
        }

        path
    }

    /// Generate a path string for a pie slice
    pub fn pie_slice_path(
        cx: f64,
        cy: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> String {
        let x1 = cx + radius * start_angle.cos();
        let y1 = cy + radius * start_angle.sin();
        let x2 = cx + radius * end_angle.cos();
        let y2 = cy + radius * end_angle.sin();

        let large_arc = if end_angle - start_angle > std::f64::consts::PI {
            1
        } else {
            0
        };

        format!("M {cx} {cy} L {x1} {y1} A {radius} {radius} 0 {large_arc} 1 {x2} {y2} Z")
    }

    /// Create default CSS styles for charts
    pub fn default_styles() -> String {
        r#"
        .chart-bar { fill: steelblue; stroke: none; }
        .chart-line { fill: none; stroke: steelblue; stroke-width: 2; }
        .chart-point { fill: steelblue; stroke: white; stroke-width: 1; }
        .chart-pie { stroke: white; stroke-width: 1; }
        .chart-axis { stroke: black; stroke-width: 1; }
        .chart-axis-text { font-family: Arial, sans-serif; font-size: 12px; fill: black; }
        .chart-title { font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; fill: black; text-anchor: middle; }
        "#.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_rect() {
        let _rect = SvgUtils::rect(10.0, 20.0, 100.0, 50.0);
        // We can't easily test the actual SVG content, but we can test that it creates without panic
    }

    #[test]
    fn test_line_path() {
        let points = vec![(0.0, 0.0), (10.0, 20.0), (20.0, 15.0)];
        let path = SvgUtils::line_path(&points);
        assert_eq!(path, "M 0 0 L 10 20 L 20 15");
    }

    #[test]
    fn test_empty_line_path() {
        let points = vec![];
        let path = SvgUtils::line_path(&points);
        assert_eq!(path, "");
    }

    #[test]
    fn test_pie_slice_path() {
        let path = SvgUtils::pie_slice_path(100.0, 100.0, 50.0, 0.0, std::f64::consts::PI / 2.0);
        assert!(path.contains("M 100 100"));
        assert!(path.contains("A 50 50"));
        assert!(path.ends_with(" Z"));
    }
}
