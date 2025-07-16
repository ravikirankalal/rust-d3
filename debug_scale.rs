// Manual inline implementation to test the new formula
fn test_scale_point_new_implementation() {
    let domain = vec!["First", "Second", "Third"];
    let range = [0.0, 300.0];
    let align = 0.25;
    
    println!("Testing new ScalePoint implementation:");
    println!("Domain: {:?}", domain);
    println!("Range: {:?}", range);
    println!("Align (padding): {}", align);
    
    let n = domain.len();
    let range_size = range[1] - range[0];
    let padding = align;
    let denominator = ((n - 1) as f64 + 2.0 * padding).max(1.0);
    let step = range_size / denominator;
    let start = range[0] + padding * step;
    
    println!("\nCalculations:");
    println!("n = {}", n);
    println!("range_size = {}", range_size);
    println!("padding = {}", padding);
    println!("denominator = max(1, {} - 1 + 2 * {}) = {}", n, padding, denominator);
    println!("step = {} / {} = {}", range_size, denominator, step);
    println!("start = {} + {} * {} = {}", range[0], padding, step, start);
    
    for (i, item) in domain.iter().enumerate() {
        let position = start + i as f64 * step;
        println!("  {}: {} + {} * {} = {}", item, start, i, step, position);
    }
    
    println!("\nTest conditions:");
    let first_pos = start + 0.0 * step;
    let third_pos = start + 2.0 * step;
    println!("first_pos > 0.0: {} > 0.0 = {}", first_pos, first_pos > 0.0);
    println!("third_pos < 300.0: {} < 300.0 = {}", third_pos, third_pos < 300.0);
}

fn main() {
    test_scale_point_new_implementation();
}
