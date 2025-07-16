use rust_d3::scale::ScalePoint;

fn main() {
    // Test case: 3 items, padding 0.25, range [0.0, 300.0]
    let scale = ScalePoint::new(
        vec!["First", "Second", "Third"], 
        [0.0, 300.0], 
        0.25 // padding
    );
    
    println!("Domain: {:?}", scale.domain);
    println!("Range: {:?}", scale.range);
    println!("Align (padding factor): {}", scale.align);
    
    // Current implementation calculations
    let range_size = scale.range[1] - scale.range[0]; // 300.0
    let padding = range_size * scale.align; // 300.0 * 0.25 = 75.0
    let available_space = range_size - 2.0 * padding; // 300.0 - 150.0 = 150.0
    let step = available_space / (3.0 - 1.0); // 150.0 / 2.0 = 75.0
    let start = scale.range[0] + padding; // 0.0 + 75.0 = 75.0
    
    println!("\nCurrent implementation:");
    println!("Range size: {}", range_size);
    println!("Padding: {}", padding);
    println!("Available space: {}", available_space);
    println!("Step: {}", step);
    println!("Start: {}", start);
    
    // Expected positions with current implementation
    println!("\nPositions (current):");
    for (i, item) in scale.domain.iter().enumerate() {
        let pos = scale.scale(item).unwrap();
        println!("{}: {}", item, pos);
    }
    
    // Expected formula from task:
    // step = (range_end - range_start) / max(1, domain.len() - 1 + 2*padding)
    // position = range_start + padding*step + index*step
    
    println!("\nExpected formula:");
    let denominator = (3.0 - 1.0 + 2.0 * 0.25_f64).max(1.0_f64);
    let expected_step = (scale.range[1] - scale.range[0]) / denominator;
    println!("Expected step: {} / {} = {}", (scale.range[1] - scale.range[0]), denominator, expected_step);
    
    println!("\nExpected positions:");
    for (i, item) in scale.domain.iter().enumerate() {
        let expected_pos = scale.range[0] + 0.25 * expected_step + i as f64 * expected_step;
        println!("{}: {}", item, expected_pos);
    }
}
