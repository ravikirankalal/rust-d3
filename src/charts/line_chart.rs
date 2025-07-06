//! Line chart implementation

use super::{add_styles_to_chart, add_title_to_chart, Chart, ChartConfig};
use crate::data::{DataUtils, Point2D};
use crate::scales::LinearScale;
use crate::svg_utils::SvgUtils;
use svg::Document;

/// Line chart for visualizing continuous data
#[derive(Debug, Clone)]
pub struct LineChart {
    data: Vec<Point2D>,
    config: ChartConfig,
    color: String,
    show_points: bool,
    line_width: f64,
}

impl LineChart {
    /// Create a new line chart
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            config: ChartConfig::default(),
            color: "steelblue".to_string(),
            show_points: true,
            line_width: 2.0,
        }
    }

    /// Set the chart data
    pub fn data(mut self, data: Vec<Point2D>) -> Self {
        self.data = data;
        self
    }

    /// Set the chart width
    pub fn width(mut self, width: u32) -> Self {
        self.config.width = width;
        self
    }

    /// Set the chart height
    pub fn height(mut self, height: u32) -> Self {
        self.config.height = height;
        self
    }

    /// Set the chart title
    pub fn title(mut self, title: &str) -> Self {
        self.config.title = Some(title.to_string());
        self
    }

    /// Set the line color
    pub fn color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }

    /// Set whether to show points
    pub fn show_points(mut self, show: bool) -> Self {
        self.show_points = show;
        self
    }

    /// Set the line width
    pub fn line_width(mut self, width: f64) -> Self {
        self.line_width = width;
        self
    }

    /// Set margins
    pub fn margins(mut self, top: u32, right: u32, bottom: u32, left: u32) -> Self {
        self.config.margin_top = top;
        self.config.margin_right = right;
        self.config.margin_bottom = bottom;
        self.config.margin_left = left;
        self
    }
}

impl Chart for LineChart {
    fn render(&self) -> Document {
        let mut doc = SvgUtils::document(self.config.width, self.config.height);
        doc = add_styles_to_chart(doc);

        if self.data.is_empty() {
            return add_title_to_chart(doc, &self.config);
        }

        // Get data extents
        let (min_point, max_point) = DataUtils::extent_2d(&self.data).unwrap();

        // Create scales
        let x_scale = LinearScale::new().domain(min_point.x, max_point.x).range(
            self.config.margin_left as f64,
            (self.config.width - self.config.margin_right) as f64,
        );

        let y_scale = LinearScale::new().domain(min_point.y, max_point.y).range(
            (self.config.height - self.config.margin_bottom) as f64,
            self.config.margin_top as f64,
        );

        // Convert data points to screen coordinates
        let screen_points: Vec<(f64, f64)> = self
            .data
            .iter()
            .map(|p| (x_scale.scale(p.x), y_scale.scale(p.y)))
            .collect();

        // Draw line
        if screen_points.len() > 1 {
            let path_data = SvgUtils::line_path(&screen_points);
            let line = SvgUtils::path(&path_data)
                .set("class", "chart-line")
                .set("stroke", self.color.as_str())
                .set("stroke-width", self.line_width)
                .set("fill", "none");

            doc = doc.add(line);
        }

        // Draw points if enabled
        if self.show_points {
            for (screen_x, screen_y) in screen_points {
                let point = SvgUtils::circle(screen_x, screen_y, 3.0)
                    .set("class", "chart-point")
                    .set("fill", self.color.as_str());

                doc = doc.add(point);
            }
        }

        // Add axes
        let x_axis = SvgUtils::line(
            self.config.margin_left as f64,
            (self.config.height - self.config.margin_bottom) as f64,
            (self.config.width - self.config.margin_right) as f64,
            (self.config.height - self.config.margin_bottom) as f64,
        )
        .set("class", "chart-axis");

        let y_axis = SvgUtils::line(
            self.config.margin_left as f64,
            self.config.margin_top as f64,
            self.config.margin_left as f64,
            (self.config.height - self.config.margin_bottom) as f64,
        )
        .set("class", "chart-axis");

        doc = doc.add(x_axis).add(y_axis);

        // Add x-axis ticks and labels
        let x_ticks = x_scale.ticks(5);
        for tick in x_ticks {
            let x = x_scale.scale(tick);
            let tick_line = SvgUtils::line(
                x,
                (self.config.height - self.config.margin_bottom) as f64,
                x,
                (self.config.height - self.config.margin_bottom + 5) as f64,
            )
            .set("class", "chart-axis");

            let tick_label = SvgUtils::text(
                x,
                (self.config.height - self.config.margin_bottom + 18) as f64,
                &format!("{tick:.1}"),
            )
            .set("class", "chart-axis-text")
            .set("text-anchor", "middle");

            doc = doc.add(tick_line).add(tick_label);
        }

        // Add y-axis ticks and labels
        let y_ticks = y_scale.ticks(5);
        for tick in y_ticks {
            let y = y_scale.scale(tick);
            let tick_line = SvgUtils::line(
                (self.config.margin_left - 5) as f64,
                y,
                self.config.margin_left as f64,
                y,
            )
            .set("class", "chart-axis");

            let tick_label = SvgUtils::text(
                (self.config.margin_left - 8) as f64,
                y + 4.0,
                &format!("{tick:.1}"),
            )
            .set("class", "chart-axis-text")
            .set("text-anchor", "end");

            doc = doc.add(tick_line).add(tick_label);
        }

        add_title_to_chart(doc, &self.config)
    }

    fn config(&self) -> &ChartConfig {
        &self.config
    }
}

impl Default for LineChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_chart_creation() {
        let chart = LineChart::new()
            .width(400)
            .height(300)
            .title("Test Chart")
            .color("red")
            .show_points(false);

        assert_eq!(chart.config.width, 400);
        assert_eq!(chart.config.height, 300);
        assert_eq!(chart.config.title, Some("Test Chart".to_string()));
        assert_eq!(chart.color, "red");
        assert!(!chart.show_points);
    }

    #[test]
    fn test_line_chart_with_data() {
        let data = vec![
            Point2D::new(0.0, 10.0),
            Point2D::new(1.0, 20.0),
            Point2D::new(2.0, 15.0),
        ];

        let chart = LineChart::new().data(data.clone());
        assert_eq!(chart.data.len(), 3);
        assert_eq!(chart.data[0].x, 0.0);
        assert_eq!(chart.data[0].y, 10.0);
    }

    #[test]
    fn test_line_chart_render() {
        let data = vec![
            Point2D::new(0.0, 10.0),
            Point2D::new(1.0, 20.0),
            Point2D::new(2.0, 15.0),
        ];

        let chart = LineChart::new().data(data).title("Test Line Chart");

        let svg_string = chart.render().to_string();
        assert!(svg_string.contains("<path class=\"chart-line\""));
        assert!(svg_string.contains("stroke=\"steelblue\""));
        assert!(svg_string.contains("stroke-width=\"2\""));
        assert!(svg_string.contains("fill=\"none\""));
        assert!(svg_string.contains("<circle class=\"chart-point\""));
        assert!(svg_string.contains("fill=\"steelblue\""));
    }

    #[test]
    fn test_single_point_line_chart() {
        let data = vec![Point2D::new(0.0, 10.0)];

        let chart = LineChart::new().data(data);
        let svg_string = chart.render().to_string();
        assert!(!svg_string.contains("<path")); // No line for single point
        assert!(svg_string.contains("<circle")); // Point should still be there
    }

    #[test]
    fn test_empty_line_chart_render() {
        let chart = LineChart::new().title("Empty Chart");
        let svg_string = chart.render().to_string();
        assert!(svg_string.contains("Empty Chart"));
        assert!(!svg_string.contains("<path"));
        assert!(!svg_string.contains("<circle"));
    }

    #[test]
    fn test_line_chart_margins() {
        let chart = LineChart::new().margins(10, 20, 30, 40);
        assert_eq!(chart.config.margin_top, 10);
        assert_eq!(chart.config.margin_right, 20);
        assert_eq!(chart.config.margin_bottom, 30);
        assert_eq!(chart.config.margin_left, 40);
    }

    #[test]
    fn test_line_chart_line_width() {
        let chart = LineChart::new().line_width(5.0);
        assert_eq!(chart.line_width, 5.0);
    }

    #[test]
    fn test_line_chart_color() {
        let data = vec![Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0)];
        let chart = LineChart::new().data(data).color("red");
        let svg_string = chart.render().to_string();
        assert!(svg_string.contains("stroke=\"red\""));
        assert!(svg_string.contains("fill=\"red\"")); // This checks the fill for the points
    }
}
