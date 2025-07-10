use rust_d3::array::extent::extent;
use rust_d3::array::max::max;
use rust_d3::selection::{Arena, NodeKey, Selection};
use rust_d3::shape::Area;
use serde::de;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rust_d3::scale::{ScaleLinear, ScaleUtc};
use chrono::{Utc, TimeZone};
use rust_d3::time::format::time_parse;
use rust_d3::axis::axis_bottom;
use chrono::{DateTime, NaiveDateTime};


fn main() {
    // Read CSV data from a file (aapl.csv)
    let file = File::open("examples/aapl.csv").expect("Cannot open aapl.csv");
    let reader = BufReader::new(file);
    let mut dates = Vec::new();
    let mut closes = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        if i == 0 { continue; } // skip header
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 { continue; }
        println!("Line {}: {:?}", i, parts);
        let parsed = time_parse("%Y-%m-%d", parts[0].to_string().as_str());
        println!("Parsed date: {:?}", parts[0].to_string().as_str());
        // let naive_datetime = NaiveDateTime::parse_from_str(parts[0].to_string().as_str(), "%Y-%m-%d").unwrap();
            // Ok(Utc.from_utc_datetime(&naive_datetime))

        dates.push(Utc.from_utc_datetime(&parsed.unwrap()));
        closes.push(parts[1].parse::<f32>().unwrap_or(0.0));
    }
    println!("Dates: {:?}", dates);
    println!("Closes: {:?}", closes);
    // Create dimensions and margins for the chart
    let width = 600;
    let height = 300;
    let margin = 40;
    let marginTop = 20;
    let marginRight = 30;
    let marginBottom = 30;
    let marginLeft = 40;
    let n = closes.len();
    let min_close = closes.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_close = closes.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    // Create arena and root SVG node
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");
    svg.attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height))
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("style", "max-width: 100%; height: auto;");
    // Draw axes

    //scaleUtc
    let x = ScaleLinear::new([0.0,max(&closes).unwrap_or(0.0) as f64], [marginLeft as f64, (width - marginRight) as f64]);
    let y = ScaleUtc::new(extent(&dates).unwrap(), [marginLeft as f64, (width - marginRight) as f64]);

    svg.append("g")
      .attr("transform", &format!("translate(0,{})",height - marginBottom))
      .call( |_| {axis_bottom(x).ticks_count(width / 80).tick_size_outer(0.0);});

    svg.append("line")
        .attr("x1", &margin.to_string())
        .attr("y1", &(height - margin).to_string())
        .attr("x2", &(width - margin).to_string())
        .attr("y2", &(height - margin).to_string())
        .attr("stroke", "black");

    svg.append("line")
        .attr("x1", &margin.to_string())
        .attr("y1", &margin.to_string())
        .attr("x2", &margin.to_string())
        .attr("y2", &(height - margin).to_string())
        .attr("stroke", "black");
    
    // Declare the area generator.
    let area  = Area::new()
        .x(|_d: &f32, i: usize| margin as f64 + (i as f64) * ((width - 2 * margin) as f64) / ((n - 1) as f64))
        .y0(|_d: &f32, _| margin as f64 + (max_close - min_close) as f64 * ((height - 2 * margin) as f64) / (max_close - min_close) as f64)
        .y1(|d: &f32, _| margin as f64 + (max_close - (*d as f32)) as f64 * ((height - 2 * margin) as f64) / (max_close - min_close) as f64);
    
    svg.append("path")
      .attr("fill", "steelblue")
      .attr("d", &area.generate(&closes))
      .attr("stroke-width", "2");

    
    // svg.append("path")
    //     .attr("d", &path.to_string())
    //     .attr("fill", "none")
    //     .attr("stroke", "steelblue")
    //     .attr("stroke-width", "2");
    // Render SVG as string
    fn render_node(arena: &Arena, key: NodeKey) -> String {
        let node = &arena.nodes[key];
        let mut s = String::new();
        s.push('<');
        s.push_str(&node.tag);
        for (k, v) in &node.attributes {
            s.push(' ');
            s.push_str(k);
            s.push_str("=\"");
            s.push_str(v);
            s.push('"');
        }
        if node.children.is_empty() {
            s.push_str("/>");
            return s;
        }
        s.push('>');
        for &child in &node.children {
            s.push_str(&render_node(arena, child));
        }
        s.push_str(&format!("</{}>", node.tag));
        s
    }
    let root_key = svg.iter().next().copied().unwrap();
    let svg_str = render_node(&arena, root_key);
    println!("{}", svg_str);
}
