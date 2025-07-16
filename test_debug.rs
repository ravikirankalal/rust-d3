use rust_d3::selection::Selection;

fn main() {
    println!("Testing data join behavior...");
    
    // Test basic data join
    let mut svg = Selection::create("svg");
    let mut rects = svg.append("rect");
    let data = vec!["A", "B", "C"];
    let joined = rects.data(&data);
    
    println!("Enter keys: {:?}", joined.enter.keys);
    println!("Update keys: {:?}", joined.update.keys);
    println!("Exit keys: {:?}", joined.exit.keys);
    
    // Test the nodes
    let enter_nodes = joined.enter.nodes();
    println!("Enter nodes data: {:?}", enter_nodes.iter().map(|n| &n.data).collect::<Vec<_>>());
    
    let update_nodes = joined.update.nodes();
    println!("Update nodes data: {:?}", update_nodes.iter().map(|n| &n.data).collect::<Vec<_>>());
}
