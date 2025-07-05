use rust_d3::charts::{BarChart, LineChart, PieChart, Chart};
use rust_d3::data::{DataPoint, Point2D};
use std::fs;

fn main() {
    println!("Rust D3 - Examples");
    println!("==================");

    // Create sample data
    let bar_data = vec![
        DataPoint::new("Product A", 120.0),
        DataPoint::new("Product B", 85.0),
        DataPoint::new("Product C", 150.0),
        DataPoint::new("Product D", 95.0),
        DataPoint::new("Product E", 200.0),
    ];

    let line_data = vec![
        Point2D::new(0.0, 10.0),
        Point2D::new(1.0, 25.0),
        Point2D::new(2.0, 15.0),
        Point2D::new(3.0, 40.0),
        Point2D::new(4.0, 30.0),
        Point2D::new(5.0, 55.0),
    ];

    let pie_data = vec![
        DataPoint::new("Chrome", 65.0),
        DataPoint::new("Firefox", 15.0),
        DataPoint::new("Safari", 12.0),
        DataPoint::new("Edge", 5.0),
        DataPoint::new("Others", 3.0),
    ];

    // Generate bar chart
    println!("Generating bar chart...");
    let bar_chart = BarChart::new()
        .data(bar_data)
        .width(800)
        .height(600)
        .title("Product Sales")
        .color("#2E86AB");

    let bar_svg = bar_chart.render();
    fs::write("examples/bar_chart.svg", bar_svg.to_string()).expect("Failed to write bar chart");

    // Generate line chart
    println!("Generating line chart...");
    let line_chart = LineChart::new()
        .data(line_data)
        .width(800)
        .height(600)
        .title("Stock Price Over Time")
        .color("#A23B72")
        .show_points(true);

    let line_svg = line_chart.render();
    fs::write("examples/line_chart.svg", line_svg.to_string()).expect("Failed to write line chart");

    // Generate pie chart
    println!("Generating pie chart...");
    let pie_chart = PieChart::new()
        .data(pie_data.clone())
        .width(800)
        .height(600)
        .title("Browser Market Share")
        .show_labels(true);

    let pie_svg = pie_chart.render();
    fs::write("examples/pie_chart.svg", pie_svg.to_string()).expect("Failed to write pie chart");

    // Generate donut chart
    println!("Generating donut chart...");
    let donut_chart = PieChart::new()
        .data(pie_data)
        .width(800)
        .height(600)
        .title("Browser Market Share (Donut)")
        .inner_radius(80.0)
        .show_labels(true);

    let donut_svg = donut_chart.render();
    fs::write("examples/donut_chart.svg", donut_svg.to_string()).expect("Failed to write donut chart");

    println!("Charts generated successfully!");
    println!("Check the 'examples/' directory for SVG files.");
}
