//! Chart implementations for various visualization types

use crate::scales::LinearScale;
use crate::svg_utils::SvgUtils;
use svg::node::element::{Group, Style};
use svg::Document;

pub mod bar_chart;
pub mod line_chart;
pub mod pie_chart;

pub use bar_chart::BarChart;
pub use line_chart::LineChart;
pub use pie_chart::PieChart;

/// Common chart configuration
#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub width: u32,
    pub height: u32,
    pub margin_top: u32,
    pub margin_right: u32,
    pub margin_bottom: u32,
    pub margin_left: u32,
    pub title: Option<String>,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            margin_top: 50,
            margin_right: 50,
            margin_bottom: 50,
            margin_left: 50,
            title: None,
        }
    }
}

impl ChartConfig {
    /// Get the inner width (excluding margins)
    pub fn inner_width(&self) -> u32 {
        self.width
            .saturating_sub(self.margin_left + self.margin_right)
    }

    /// Get the inner height (excluding margins)
    pub fn inner_height(&self) -> u32 {
        self.height
            .saturating_sub(self.margin_top + self.margin_bottom)
    }
}

/// Trait for all chart types
pub trait Chart {
    /// Render the chart to an SVG document
    fn render(&self) -> Document;

    /// Get the chart configuration
    fn config(&self) -> &ChartConfig;
}

/// Helper function to add title to chart
pub fn add_title_to_chart(doc: Document, config: &ChartConfig) -> Document {
    if let Some(title) = &config.title {
        let title_element =
            SvgUtils::text(config.width as f64 / 2.0, 25.0, title).set("class", "chart-title");

        doc.add(title_element)
    } else {
        doc
    }
}

/// Helper function to create axes for charts
pub fn create_axes(x_scale: &LinearScale, y_scale: &LinearScale, config: &ChartConfig) -> Group {
    let mut group = SvgUtils::group();

    let inner_width = config.inner_width() as f64;
    let inner_height = config.inner_height() as f64;
    let margin_left = config.margin_left as f64;
    let margin_top = config.margin_top as f64;

    // X axis
    let x_axis = SvgUtils::line(
        margin_left,
        margin_top + inner_height,
        margin_left + inner_width,
        margin_top + inner_height,
    )
    .set("class", "chart-axis");
    group = group.add(x_axis);

    // Y axis
    let y_axis = SvgUtils::line(
        margin_left,
        margin_top,
        margin_left,
        margin_top + inner_height,
    )
    .set("class", "chart-axis");
    group = group.add(y_axis);

    // X axis ticks and labels
    let x_ticks = x_scale.ticks(5);
    for tick in x_ticks {
        let x = margin_left + x_scale.scale(tick);
        let tick_line = SvgUtils::line(
            x,
            margin_top + inner_height,
            x,
            margin_top + inner_height + 5.0,
        )
        .set("class", "chart-axis");

        let tick_label = SvgUtils::text(x, margin_top + inner_height + 18.0, &format!("{tick:.1}"))
            .set("class", "chart-axis-text")
            .set("text-anchor", "middle");

        group = group.add(tick_line).add(tick_label);
    }

    // Y axis ticks and labels
    let y_ticks = y_scale.ticks(5);
    for tick in y_ticks {
        let y = margin_top + y_scale.scale(tick);
        let tick_line =
            SvgUtils::line(margin_left - 5.0, y, margin_left, y).set("class", "chart-axis");

        let tick_label = SvgUtils::text(margin_left - 8.0, y + 4.0, &format!("{tick:.1}"))
            .set("class", "chart-axis-text")
            .set("text-anchor", "end");

        group = group.add(tick_line).add(tick_label);
    }

    group
}

/// Helper function to add default styles to a chart
pub fn add_styles_to_chart(doc: Document) -> Document {
    let style = Style::new(SvgUtils::default_styles());
    doc.add(style)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_config() {
        let config = ChartConfig::default();
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.inner_width(), 700);
        assert_eq!(config.inner_height(), 500);
    }

    #[test]
    fn test_chart_config_with_custom_margins() {
        let config = ChartConfig {
            width: 400,
            height: 300,
            margin_left: 60,
            margin_right: 40,
            margin_top: 30,
            margin_bottom: 50,
            title: None,
        };

        assert_eq!(config.inner_width(), 300);
        assert_eq!(config.inner_height(), 220);
    }

    #[test]
    fn test_add_title_to_chart_with_title() {
        let config = ChartConfig {
            title: Some("My Chart Title".to_string()),
            ..Default::default()
        };
        let doc = Document::new();
        let doc_with_title = add_title_to_chart(doc, &config);
        let svg_string = doc_with_title.to_string();
        assert!(svg_string.contains("<text"));
        assert!(svg_string.contains("class=\"chart-title\""));
        assert!(svg_string.contains("My Chart Title"));
    }

    #[test]
    fn test_add_title_to_chart_without_title() {
        let config = ChartConfig {
            title: None,
            ..Default::default()
        };
        let doc = Document::new();
        let doc_without_title = add_title_to_chart(doc, &config);
        let svg_string = doc_without_title.to_string();
        assert!(!svg_string.contains("<text"));
    }

    #[test]
    fn test_create_axes() {
        let x_scale = LinearScale::new().domain(0.0, 10.0).range(0.0, 100.0);
        let y_scale = LinearScale::new().domain(0.0, 100.0).range(200.0, 0.0);
        let config = ChartConfig::default();
        let axes_group = create_axes(&x_scale, &y_scale, &config);
        let svg_string = Document::new().add(axes_group).to_string();

        // Check for x and y axis lines
        assert!(svg_string.matches("<line").count() >= 2);
        // Check for x and y ticks
        assert!(svg_string.matches("<text").count() >= 10);
    }

    #[test]
    fn test_add_styles_to_chart() {
        let doc = Document::new();
        let doc_with_style = add_styles_to_chart(doc);
        let svg_string = doc_with_style.to_string();
        assert!(svg_string.contains("<style>"));
        assert!(svg_string.contains(".chart-title"));
    }
}
