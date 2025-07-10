use rust_d3::selection::{Arena, NodeKey, Selection};

fn main() {
    let data = vec![30, 80, 45, 60, 20, 90, 55];
    let width = 300;
    let height = 120;
    let bar_width = 30;
    let bar_gap = 10;

    // Create arena and root SVG node using Selection API
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");

    svg.attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height))
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("style", "max-width: 100%; height: auto;");

    // Add a rect for each bar.
    svg.append("g")
    .attr("fill", "steelblue")
    .select_all(None)
        .data(&data)
        .join("rect")
        .attr_fn("x", |_, i| (i * (bar_width + bar_gap)).to_string())
        .attr_fn("y", |n, _| {
            let d = n.data.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
            (height - d).to_string()
        })
        .attr("width", &bar_width.to_string())
        .attr_fn("height", |n, _| n.data.as_ref().cloned().unwrap_or_else(|| "0".to_string()))
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

    // Get the root key from the selection
    let root_key = svg.iter().next().copied().unwrap();
    let svg_str = render_node(&arena, root_key);
    println!("{}", svg_str);
}
