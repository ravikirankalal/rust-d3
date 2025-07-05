//! Bar chart implementation

use super::{add_styles_to_chart, add_title_to_chart, Chart, ChartConfig};
use crate::data::{DataPoint, DataUtils};
use crate::scales::{BandScale, LinearScale};
use crate::svg_utils::SvgUtils;
use svg::Document;

/// Bar chart for visualizing categorical data
#[derive(Debug, Clone)]
pub struct BarChart {
    data: Vec<DataPoint>,
    config: ChartConfig,
    color: String,
}

impl BarChart {
    /// Create a new bar chart
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            config: ChartConfig::default(),
            color: "steelblue".to_string(),
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

    /// Set the bar color
    pub fn color(mut self, color: &str) -> Self {
        self.color = color.to_string();
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

impl Chart for BarChart {
    fn render(&self) -> Document {
        let mut doc = SvgUtils::document(self.config.width, self.config.height);
        doc = add_styles_to_chart(doc);

        if self.data.is_empty() {
            return add_title_to_chart(doc, &self.config);
        }

        // Create scales
        let x_scale = BandScale::new()
            .domain(self.data.iter().map(|d| d.label.clone()).collect())
            .range(
                self.config.margin_left as f64,
                (self.config.width - self.config.margin_right) as f64,
            )
            .padding(0.2);

        let max_value = DataUtils::max(&self.data).unwrap_or(0.0);
        let y_scale = LinearScale::new().domain(0.0, max_value).range(
            (self.config.height - self.config.margin_bottom) as f64,
            self.config.margin_top as f64,
        );

        // Draw bars
        for data_point in &self.data {
            if let Some(x) = x_scale.scale(&data_point.label) {
                let bar_height = y_scale.scale(0.0) - y_scale.scale(data_point.value);
                let bar = SvgUtils::rect(
                    x,
                    y_scale.scale(data_point.value),
                    x_scale.bandwidth(),
                    bar_height,
                )
                .set("class", "chart-bar")
                .set("fill", self.color.as_str());

                doc = doc.add(bar);

                // Add value labels on top of bars
                let label = SvgUtils::text(
                    x + x_scale.bandwidth() / 2.0,
                    y_scale.scale(data_point.value) - 5.0,
                    &format!("{:.1}", data_point.value),
                )
                .set("class", "chart-axis-text")
                .set("text-anchor", "middle");

                doc = doc.add(label);
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

        // Add x-axis labels
        for data_point in &self.data {
            if let Some(x) = x_scale.scale(&data_point.label) {
                let label = SvgUtils::text(
                    x + x_scale.bandwidth() / 2.0,
                    (self.config.height - self.config.margin_bottom + 18) as f64,
                    &data_point.label,
                )
                .set("class", "chart-axis-text")
                .set("text-anchor", "middle");

                doc = doc.add(label);
            }
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

impl Default for BarChart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_chart_creation() {
        let chart = BarChart::new().width(400).height(300).title("Test Chart");

        assert_eq!(chart.config.width, 400);
        assert_eq!(chart.config.height, 300);
        assert_eq!(chart.config.title, Some("Test Chart".to_string()));
    }

    #[test]
    fn test_bar_chart_with_data() {
        let data = vec![
            DataPoint::new("A", 10.0),
            DataPoint::new("B", 20.0),
            DataPoint::new("C", 15.0),
        ];

        let chart = BarChart::new().data(data.clone());
        assert_eq!(chart.data.len(), 3);
        assert_eq!(chart.data[0].label, "A");
        assert_eq!(chart.data[0].value, 10.0);
    }

    #[test]
    fn test_bar_chart_render() {
        let data = vec![DataPoint::new("A", 10.0), DataPoint::new("B", 20.0)];

        let chart = BarChart::new().data(data).title("Test Bar Chart");

        let _svg = chart.render();
        // Test that rendering doesn't panic
    }

    #[test]
    fn test_empty_bar_chart_render() {
        let chart = BarChart::new().title("Empty Chart");
        let _svg = chart.render();
        // Test that rendering empty chart doesn't panic
    }
}
