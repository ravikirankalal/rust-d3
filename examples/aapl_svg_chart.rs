use chrono::{TimeZone, Utc};
use rust_d3::axis::{AxisRenderable, axis_bottom, axis_left};
use rust_d3::scale::ScaleLinear;
use rust_d3::selection::{Arena, Selection};
use rust_d3::shape::Area;
use rust_d3::time::format::time_parse;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read CSV data from a file (aapl.csv)
    let file = File::open("examples/aapl.csv").expect("Cannot open aapl.csv");
    let reader = BufReader::new(file);
    let mut dates = Vec::new();
    let mut closes = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        if i == 0 {
            continue;
        } // skip header
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue;
        }
        // println!("Line {}: {:?}", i, parts);
        let parsed = time_parse("%Y-%m-%d", parts[0].to_string().as_str());
        // println!("Parsed date: {:?}", parts[0].to_string().as_str());
        dates.push(Utc.from_utc_datetime(&parsed.unwrap()));
        closes.push(parts[1].parse::<f32>().unwrap_or(0.0));
    }
    println!("Dates: {:?}", dates);
    println!("Closes: {:?}", closes);
    // Create dimensions and margins for the chart
    let width = 600;
    let height = 300;
    let _margin = 40;
    let _margin_top = 20;
    let margin_right = 30;
    let margin_bottom = 30;
    let margin_left = 40;
    let n = closes.len();
    let min_close = closes.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_close = closes.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    // Create arena and root SVG node
    let mut arena = Arena {
        nodes: slotmap::SlotMap::with_key(),
    };
    let mut svg = Selection::root(&mut arena, "svg");
    svg.attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height))
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("style", "max-width: 100%; height: auto;");
    // Draw axes

    // X scale
    let x = ScaleLinear::new(
        [0.0, (n - 1) as f64],
        [margin_left as f64, (width - margin_right) as f64],
    );
    // Y scale (invert so higher values are at the top)
    let y = ScaleLinear::new(
        [min_close as f64, max_close as f64],
        [(height - margin_bottom) as f64, _margin_top as f64],
    );

    // Append x-axis
    let mut x_axis_group = svg.append("g");
    x_axis_group.attr(
        "transform",
        &format!("translate(0,{})", height - margin_bottom),
    );
    let x_axis = axis_bottom(x.clone())
        .tick_count(width / 80)
        .tick_size_inner(6.0)
        .tick_padding(3.0);
    x_axis.render(&mut x_axis_group);

    // Append y-axis
    let mut y_axis_group = svg.append("g");
    y_axis_group.attr("transform", &format!("translate({},0)", margin_left));
    let y_axis = axis_left(y.clone())
        .tick_count(height / 40)
        .tick_size_inner(6.0)
        .tick_padding(3.0);
    y_axis.render(&mut y_axis_group);

    // Area generator
    let area = Area::new()
        .x(|_d: &f32, i: usize| x.scale(i as f64))
        .y0(|_d: &f32, _| y.scale(min_close as f64))
        .y1(|d: &f32, _i| y.scale(*d as f64));

    svg.append("path")
        .attr("fill", "steelblue")
        .attr("d", &area.generate(&closes))
        .attr("stroke-width", "2");

    let root_key = svg.iter().next().copied().unwrap();
    let svg_str = Selection::render_node(&arena, root_key);
    println!("{}", svg_str);
}
