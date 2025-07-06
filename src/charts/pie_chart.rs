//! Pie chart implementation

use super::{add_styles_to_chart, add_title_to_chart, Chart, ChartConfig};
use crate::data::{DataPoint, DataUtils};
use crate::svg_utils::SvgUtils;
use svg::Document;

/// Pie chart for visualizing proportional data
#[derive(Debug, Clone)]
pub struct PieChart {
    data: Vec<DataPoint>,
    config: ChartConfig,
    colors: Vec<String>,
    inner_radius: f64,
    outer_radius: f64,
    show_labels: bool,
}

impl PieChart {
    /// Create a new pie chart
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            config: ChartConfig::default(),
            colors: vec![
                "#1f77b4".to_string(),
                "#ff7f0e".to_string(),
                "#2ca02c".to_string(),
                "#d62728".to_string(),
                "#9467bd".to_string(),
                "#8c564b".to_string(),
                "#e377c2".to_string(),
                "#7f7f7f".to_string(),
                "#bcbd22".to_string(),
                "#17becf".to_string(),
            ],
            inner_radius: 0.0,
            outer_radius: 0.0,
            show_labels: true,
        }
    }

    /// Set the chart data
    pub fn data(mut self, data: Vec<DataPoint>) -> Self {
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

    /// Set custom colors
    pub fn colors(mut self, colors: Vec<String>) -> Self {
        self.colors = colors;
        self
    }

    /// Set inner radius (for donut chart)
    pub fn inner_radius(mut self, radius: f64) -> Self {
        self.inner_radius = radius;
        self
    }

    /// Set outer radius
    pub fn outer_radius(mut self, radius: f64) -> Self {
        self.outer_radius = radius;
        self
    }

    /// Set whether to show labels
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
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

    /// Calculate the effective outer radius
    fn effective_outer_radius(&self) -> f64 {
        if self.outer_radius > 0.0 {
            self.outer_radius
        } else {
            let inner_width = self.config.inner_width() as f64;
            let inner_height = self.config.inner_height() as f64;
            (inner_width.min(inner_height) / 2.0) * 0.8
        }
    }
}

impl Chart for PieChart {
    fn render(&self) -> Document {
        let mut doc = SvgUtils::document(self.config.width, self.config.height);
        doc = add_styles_to_chart(doc);

        if self.data.is_empty() {
            return add_title_to_chart(doc, &self.config);
        }

        let total = DataUtils::sum(&self.data);
        if total <= 0.0 {
            return add_title_to_chart(doc, &self.config);
        }

        let center_x = self.config.width as f64 / 2.0;
        let center_y = self.config.height as f64 / 2.0;
        let outer_radius = self.effective_outer_radius();

        let mut current_angle = -std::f64::consts::PI / 2.0; // Start at top

        for (i, data_point) in self.data.iter().enumerate() {
            let slice_angle = (data_point.value / total) * 2.0 * std::f64::consts::PI;
            let end_angle = current_angle + slice_angle;

            // Get color for this slice
            let color = &self.colors[i % self.colors.len()];

            if self.inner_radius > 0.0 {
                // Donut chart - create an annular sector
                let outer_path = SvgUtils::pie_slice_path(
                    center_x,
                    center_y,
                    outer_radius,
                    current_angle,
                    end_angle,
                );
                let inner_path = SvgUtils::pie_slice_path(
                    center_x,
                    center_y,
                    self.inner_radius,
                    current_angle,
                    end_angle,
                );

                // Create compound path for donut slice
                let path_data = format!(
                    "{} {} Z",
                    outer_path.trim_end_matches(" Z"),
                    inner_path.replace("M", "L").replace("Z", "")
                );

                let slice = SvgUtils::path(&path_data)
                    .set("class", "chart-pie")
                    .set("fill", color.as_str());

                doc = doc.add(slice);
            } else {
                // Regular pie chart
                let path_data = SvgUtils::pie_slice_path(
                    center_x,
                    center_y,
                    outer_radius,
                    current_angle,
                    end_angle,
                );

                let slice = SvgUtils::path(&path_data)
                    .set("class", "chart-pie")
                    .set("fill", color.as_str());

                doc = doc.add(slice);
            }

            // Add labels if enabled
            if self.show_labels {
                let mid_angle = current_angle + slice_angle / 2.0;
                let label_radius = outer_radius * 0.7;
                let label_x = center_x + label_radius * mid_angle.cos();
                let label_y = center_y + label_radius * mid_angle.sin();

                let percentage = (data_point.value / total * 100.0).round();
                let label_text = if percentage >= 5.0 {
                    format!("{percentage}%")
                } else {
                    String::new() // Don't show labels for very small slices
                };

                if !label_text.is_empty() {
                    let label = SvgUtils::text(label_x, label_y, &label_text)
                        .set("class", "chart-axis-text")
                        .set("text-anchor", "middle")
                        .set("fill", "white")
                        .set("font-weight", "bold");

                    doc = doc.add(label);
                }
            }

            current_angle = end_angle;
        }

        // Add legend
        let legend_x = self.config.width as f64 - self.config.margin_right as f64 + 10.0;
        let mut legend_y = self.config.margin_top as f64;

        for (i, data_point) in self.data.iter().enumerate() {
            let color = &self.colors[i % self.colors.len()];

            // Legend color box
            let legend_rect = SvgUtils::rect(legend_x, legend_y, 15.0, 15.0)
                .set("fill", color.as_str())
                .set("stroke", "black")
                .set("stroke-width", 1);

            // Legend text
            let legend_text = SvgUtils::text(legend_x + 20.0, legend_y + 12.0, &data_point.label)
                .set("class", "chart-axis-text");

            doc = doc.add(legend_rect).add(legend_text);
            legend_y += 25.0;
        }

        add_title_to_chart(doc, &self.config)
    }

    fn config(&self) -> &ChartConfig {
        &self.config
    }
}

impl Default for PieChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pie_chart_creation() {
        let chart = PieChart::new()
            .width(400)
            .height(300)
            .title("Test Chart")
            .inner_radius(50.0)
            .show_labels(false);

        assert_eq!(chart.config.width, 400);
        assert_eq!(chart.config.height, 300);
        assert_eq!(chart.config.title, Some("Test Chart".to_string()));
        assert_eq!(chart.inner_radius, 50.0);
        assert!(!chart.show_labels);
    }

    #[test]
    fn test_pie_chart_with_data() {
        let data = vec![
            DataPoint::new("A", 30.0),
            DataPoint::new("B", 20.0),
            DataPoint::new("C", 50.0),
        ];

        let chart = PieChart::new().data(data.clone());
        assert_eq!(chart.data.len(), 3);
        assert_eq!(chart.data[0].label, "A");
        assert_eq!(chart.data[0].value, 30.0);
    }

    #[test]
    fn test_pie_chart_render() {
        let data = vec![DataPoint::new("A", 30.0), DataPoint::new("B", 70.0)];

        let chart = PieChart::new().data(data).title("Test Pie Chart");

        let svg_string = chart.render().to_string();
        assert!(svg_string.contains("<path class=\"chart-pie\""));
        assert!(svg_string.contains("fill=\"#1f77b4\"")); // Default color for first slice
        assert!(svg_string.contains("fill=\"#ff7f0e\"")); // Default color for second slice
        assert!(svg_string.contains("30%")); // Check for label
        assert!(svg_string.contains("70%")); // Check for label
    }

    #[test]
    fn test_donut_chart_render() {
        let data = vec![DataPoint::new("A", 40.0), DataPoint::new("B", 60.0)];

        let chart = PieChart::new()
            .data(data)
            .inner_radius(50.0)
            .title("Test Donut Chart");

        let svg_string = chart.render().to_string();
        assert!(svg_string.contains("<path class=\"chart-pie\""));
        assert!(svg_string.contains("fill=\"#1f77b4\""));
        assert!(svg_string.contains("fill=\"#ff7f0e\""));
        assert!(svg_string.contains("40%"));
        assert!(svg_string.contains("60%"));
    }

    #[test]
    fn test_empty_pie_chart_render() {
        let chart = PieChart::new().title("Empty Chart");
        let svg_string = chart.render().to_string();
        assert!(svg_string.contains("Empty Chart"));
        assert!(!svg_string.contains("<path"));
        assert!(!svg_string.contains("<circle"));
    }

    #[test]
    fn test_effective_outer_radius() {
        let chart = PieChart::new().width(400).height(300);
        let radius = chart.effective_outer_radius();
        assert!(radius > 0.0);
        assert!(radius < 150.0); // Should be less than half the smaller dimension

        let chart_with_outer_radius = PieChart::new().outer_radius(100.0);
        assert_eq!(chart_with_outer_radius.effective_outer_radius(), 100.0);
    }

    #[test]
    fn test_pie_chart_colors() {
        let colors = vec!["red".to_string(), "blue".to_string()];
        let chart = PieChart::new().colors(colors.clone());
        assert_eq!(chart.colors, colors);
    }

    #[test]
    fn test_pie_chart_outer_radius() {
        let chart = PieChart::new().outer_radius(75.0);
        assert_eq!(chart.outer_radius, 75.0);
    }

    #[test]
    fn test_pie_chart_show_labels() {
        let data = vec![DataPoint::new("A", 100.0)];
        let chart_with_labels = PieChart::new().data(data.clone()).show_labels(true);
        let svg_string_with_labels = chart_with_labels.render().to_string();
        assert!(svg_string_with_labels.contains("100%"));

        let chart_without_labels = PieChart::new().data(data).show_labels(false);
        let svg_string_without_labels = chart_without_labels.render().to_string();
        assert!(!svg_string_without_labels.contains("100%"));
    }

    #[test]
    fn test_pie_chart_margins() {
        let chart = PieChart::new().margins(10, 20, 30, 40);
        assert_eq!(chart.config.margin_top, 10);
        assert_eq!(chart.config.margin_right, 20);
        assert_eq!(chart.config.margin_bottom, 30);
        assert_eq!(chart.config.margin_left, 40);
    }

    #[test]
    fn test_pie_chart_zero_total() {
        let data = vec![DataPoint::new("A", 0.0), DataPoint::new("B", 0.0)];
        let chart = PieChart::new().data(data).title("Zero Total Chart");
        let svg_string = chart.render().to_string();
        assert!(svg_string.contains("Zero Total Chart"));
        assert!(!svg_string.contains("<path"));
    }
}
