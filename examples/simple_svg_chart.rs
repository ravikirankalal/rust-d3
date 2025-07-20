use rust_d3::selection::Selection;

fn main() {
    let data = [30, 80, 45, 60, 20, 90, 55];
    let width = 300;
    let height = 120;
    let bar_width = 30;
    let bar_gap = 10;

    // Create SVG selection
    let mut svg = Selection::create("svg");

    svg.attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height))
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("style", "max-width: 100%; height: auto;");

    // Add a rect for each bar.
    let _data_join = svg
        .append("g")
        .attr("fill", "steelblue")
        .select_all(None)
        .data(&data)
        .enter()
        .exit()
        .append("rect")
        .attr_fn("x", |_, i, _| (i * (bar_width + bar_gap)).to_string())
        .attr_fn("y", |n, _, _| {
            let d = n
                .data
                .as_ref()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);
            (height - d).to_string()
        })
        .attr("width", &bar_width.to_string())
        .attr_fn("height", |n, _, _| {
            n.data.as_ref().cloned().unwrap_or_else(|| "0".to_string())
        })
        .attr("fill", "steelblue");

    svg.append("line")
        .attr("x1", "0")
        .attr("y1", &height.to_string())
        .attr("x2", &width.to_string())
        .attr("y2", &height.to_string())
        .attr("stroke", "black");

    svg.append("line")
        .attr("x1", "0")
        .attr("y1", "0")
        .attr("x2", "0")
        .attr("y2", &height.to_string())
        .attr("stroke", "black");

    // Render SVG as string using Selection's render method
    let svg_str = svg.render();
    println!("{}", svg_str);
}
