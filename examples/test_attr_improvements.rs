use rust_d3::selection::Selection;
use rust_d3::px;

fn main() {
    // Test 1: Demonstrate that attr() skips setting identical values
    let mut svg = Selection::create("svg");
    let mut circle = svg.append("circle");
    
    // First time setting the attribute
    println!("Setting radius to '10' for the first time");
    circle.attr("r", "10");
    let node = circle.node().unwrap();
    println!("Radius after first set: {:?}", node.attributes.get("r"));
    
    // Setting the same value again - this should be skipped internally (micro-perf optimization)
    println!("Setting radius to '10' again (should be optimized to skip)");
    circle.attr("r", "10");
    let node = circle.node().unwrap();
    println!("Radius after second set (same value): {:?}", node.attributes.get("r"));
    
    // Setting a different value - this should update
    println!("Setting radius to '20' (should update)");
    circle.attr("r", "20");
    let node = circle.node().unwrap();
    println!("Radius after changing to '20': {:?}", node.attributes.get("r"));
    
    println!();
    
    // Test 2: Demonstrate the new attr_px() convenience method
    let mut rect = svg.append("rect");
    
    println!("Using new attr_px() method:");
    rect.attr_px("width", 100.5)
        .attr_px("height", 50.0)
        .attr_px("x", 10.333333)
        .attr_px("y", 0.0);
    
    let rect_node = rect.node().unwrap();
    println!("Width: {:?}", rect_node.attributes.get("width"));
    println!("Height: {:?}", rect_node.attributes.get("height"));
    println!("X: {:?}", rect_node.attributes.get("x"));
    println!("Y: {:?}", rect_node.attributes.get("y"));
    
    println!();
    
    // Test 3: Compare attr_px() vs manual px() conversion
    let mut line = svg.append("line");
    
    // Manual approach (old way)
    line.attr("x1", &px(25.75))
        .attr("y1", &px(30.0));
    
    // New convenience approach
    line.attr_px("x2", 125.75)
        .attr_px("y2", 130.0);
    
    let line_node = line.node().unwrap();
    println!("Line coordinates using both approaches:");
    println!("X1 (manual px): {:?}", line_node.attributes.get("x1"));
    println!("Y1 (manual px): {:?}", line_node.attributes.get("y1"));
    println!("X2 (attr_px): {:?}", line_node.attributes.get("x2"));
    println!("Y2 (attr_px): {:?}", line_node.attributes.get("y2"));
    
    println!();
    println!("Final SVG structure:");
    println!("{}", svg.render());
}
