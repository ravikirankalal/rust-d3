use std::sync::Arc;

use rust_d3::selection::Selection;

fn main() {
    // Example data
    let data = vec![30, 80, 45, 60, 20, 90, 55];
    let width = 300;
    let height = 120;
    let bar_width = 30;
    let bar_gap = 10;

    // Create SVG root
    let mut svg = Selection::create("svg");
    let value = format!("0 0 {} {}", width, height);
    svg.attr("width", width.to_string().as_str())
    .attr("height", height.to_string().as_str())
    .attr("viewBox", value.as_str())
    .attr("xmlns", "http://www.w3.org/2000/svg")
    .attr("style", "max-width: 100%; height: auto;");


    // svg.append("g")
    //         .attr("fill", "steelblue");
    // svg.select_all(None)
    //         .data(&data)
    //         .join("rect")
    //         .attr("x", |_, i| (i * (bar_width + bar_gap)) as i32)
    //         .attr("y", |d: &i32, _| (height - d) as i32)
    //         .attr("width", bar_width)
    //         .attr("height", |d: &i32, _| *d as i32)
    //         .attr("fill", "steelblue");
    // Add bars
    // for (i, value) in data.iter().enumerate() {
    //     let mut rect = Selection::create("rect");
    //     rect.nodes[0].attributes.insert("x".to_string(), (i as i32 * (bar_width + bar_gap)).to_string());
    //     rect.nodes[0].attributes.insert("y".to_string(), (height - value).to_string());
    //     rect.nodes[0].attributes.insert("width".to_string(), bar_width.to_string());
    //     rect.nodes[0].attributes.insert("height".to_string(), value.to_string());
    //     rect.nodes[0].attributes.insert("fill".to_string(), "steelblue".to_string());
    //     svg.nodes[0].children.push(rect.nodes[0].clone());
    // }

    // Add x-axis
    let mut x_axis = Selection::create("line");
    x_axis.nodes[0].attributes.insert("x1".to_string(), "0".to_string());
    x_axis.nodes[0].attributes.insert("y1".to_string(), height.to_string());
    x_axis.nodes[0].attributes.insert("x2".to_string(), width.to_string());
    x_axis.nodes[0].attributes.insert("y2".to_string(), height.to_string());
    x_axis.nodes[0].attributes.insert("stroke".to_string(), "black".to_string());
    svg.nodes[0].children.push(x_axis.nodes[0].clone());

    // Add y-axis
    let mut y_axis = Selection::create("line");
    y_axis.nodes[0].attributes.insert("x1".to_string(), "0".to_string());
    y_axis.nodes[0].attributes.insert("y1".to_string(), "0".to_string());
    y_axis.nodes[0].attributes.insert("x2".to_string(), "0".to_string());
    y_axis.nodes[0].attributes.insert("y2".to_string(), height.to_string());
    y_axis.nodes[0].attributes.insert("stroke".to_string(), "black".to_string());
    svg.nodes[0].children.push(y_axis.nodes[0].clone());

    // Render SVG as string
    fn render_node(node: &rust_d3::selection::Node) -> String {
        let mut s = String::new();
        s.push('<');
        s.push_str(&node.tag);
        for (k, v) in &node.attributes {
            if k == "textContent" || k == "innerHTML" { continue; }
            s.push(' ');
            s.push_str(k);
            s.push_str("=\"");
            s.push_str(v);
            s.push('"');
        }
        if node.children.is_empty() && !node.attributes.contains_key("textContent") {
            s.push_str("/>");
            return s;
        }
        s.push('>');
        if let Some(txt) = node.attributes.get("textContent") {
            s.push_str(txt);
        }
        for child in &node.children {
            s.push_str(&render_node(child));
        }
        s.push_str(&format!("</{}>", node.tag));
        s
    }

    let svg_str = render_node(&svg.nodes[0]);
    println!("{}", svg_str);
}
