//! Integration tests that generate chart images for documentation

use rust_d3::charts::{BarChart, Chart, LineChart, PieChart};
use rust_d3::data::{DataPoint, Point2D};
use std::fs;

#[test]
fn test_generate_bar_chart_for_readme() {
    let data = vec![
        DataPoint::new("Q1 2023", 120.0),
        DataPoint::new("Q2 2023", 85.0),
        DataPoint::new("Q3 2023", 150.0),
        DataPoint::new("Q4 2023", 95.0),
        DataPoint::new("Q1 2024", 200.0),
    ];

    let chart = BarChart::new()
        .data(data)
        .width(600)
        .height(400)
        .title("Quarterly Revenue ($k)")
        .color("#3498db");

    let svg = chart.render();
    fs::write("target/bar_chart_example.svg", svg.to_string()).expect("Failed to write bar chart");
}

#[test]
fn test_generate_line_chart_for_readme() {
    let data = vec![
        Point2D::new(0.0, 50.0),
        Point2D::new(1.0, 75.0),
        Point2D::new(2.0, 65.0),
        Point2D::new(3.0, 90.0),
        Point2D::new(4.0, 85.0),
        Point2D::new(5.0, 110.0),
        Point2D::new(6.0, 95.0),
    ];

    let chart = LineChart::new()
        .data(data)
        .width(600)
        .height(400)
        .title("Stock Price Over Time")
        .color("#e74c3c")
        .show_points(true);

    let svg = chart.render();
    fs::write("target/line_chart_example.svg", svg.to_string())
        .expect("Failed to write line chart");
}

#[test]
fn test_generate_pie_chart_for_readme() {
    let data = vec![
        DataPoint::new("Chrome", 65.0),
        DataPoint::new("Firefox", 15.0),
        DataPoint::new("Safari", 12.0),
        DataPoint::new("Edge", 5.0),
        DataPoint::new("Others", 3.0),
    ];

    let chart = PieChart::new()
        .data(data)
        .width(600)
        .height(400)
        .title("Browser Market Share")
        .show_labels(true);

    let svg = chart.render();
    fs::write("target/pie_chart_example.svg", svg.to_string()).expect("Failed to write pie chart");
}

#[test]
fn test_generate_donut_chart_for_readme() {
    let data = vec![
        DataPoint::new("Desktop", 45.0),
        DataPoint::new("Mobile", 40.0),
        DataPoint::new("Tablet", 15.0),
    ];

    let chart = PieChart::new()
        .data(data)
        .width(600)
        .height(400)
        .title("Device Usage Distribution")
        .inner_radius(60.0)
        .show_labels(true);

    let svg = chart.render();
    fs::write("target/donut_chart_example.svg", svg.to_string())
        .expect("Failed to write donut chart");
}

#[test]
fn test_custom_styled_charts() {
    // Test bar chart with custom styling
    let data = vec![
        DataPoint::new("A", 10.0),
        DataPoint::new("B", 25.0),
        DataPoint::new("C", 15.0),
        DataPoint::new("D", 30.0),
    ];

    let chart = BarChart::new()
        .data(data)
        .width(400)
        .height(300)
        .title("Custom Bar Chart")
        .color("#9b59b6")
        .margins(40, 30, 60, 70);

    let svg = chart.render();
    assert!(svg.to_string().contains("Custom Bar Chart"));
    assert!(svg.to_string().contains("#9b59b6"));

    // Test line chart without points
    let line_data = vec![
        Point2D::new(0.0, 10.0),
        Point2D::new(1.0, 20.0),
        Point2D::new(2.0, 15.0),
    ];

    let line_chart = LineChart::new()
        .data(line_data)
        .width(400)
        .height(300)
        .title("Line Without Points")
        .color("#f39c12")
        .show_points(false)
        .line_width(3.0);

    let line_svg = line_chart.render();
    assert!(line_svg.to_string().contains("Line Without Points"));
    assert!(line_svg.to_string().contains("#f39c12"));
}

#[test]
fn test_edge_cases() {
    // Test empty charts
    let empty_bar = BarChart::new().title("Empty Bar Chart");
    let empty_svg = empty_bar.render();
    assert!(empty_svg.to_string().contains("Empty Bar Chart"));

    let empty_line = LineChart::new().title("Empty Line Chart");
    let empty_line_svg = empty_line.render();
    assert!(empty_line_svg.to_string().contains("Empty Line Chart"));

    let empty_pie = PieChart::new().title("Empty Pie Chart");
    let empty_pie_svg = empty_pie.render();
    assert!(empty_pie_svg.to_string().contains("Empty Pie Chart"));

    // Test single data point
    let single_data = vec![DataPoint::new("Single", 42.0)];
    let single_bar = BarChart::new().data(single_data).title("Single Bar");
    let single_svg = single_bar.render();
    assert!(single_svg.to_string().contains("Single Bar"));
    assert!(single_svg.to_string().contains("42.0"));
}
