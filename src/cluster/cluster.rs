use crate::hierarchy::Node;

pub fn cluster<T: Clone>(root: &mut Node<T>, size: (f64, f64)) {
    let mut x = 0.0;
    let mut max_depth = 0;

    // First pass: assign x-coordinates and calculate max_depth
    assign_x_and_depth(root, &mut x, 0, &mut max_depth);

    // Second pass: assign y-coordinates based on depth
    assign_y(root, size.1, max_depth);

    // Third pass: scale x-coordinates to fit the width
    let max_x = get_max_x(root);
    scale_x(root, size.0, max_x);
}

fn assign_x_and_depth<T: Clone>(node: &mut Node<T>, x: &mut f64, depth: usize, max_depth: &mut usize) {
    node.y0 = depth as f64; // Temporarily use y0 for depth
    *max_depth = (*max_depth).max(depth);

    if node.children.is_empty() {
        node.x0 = *x; // Temporarily use x0 for initial x-position
        *x += 1.0; // Increment x for the next leaf node
    } else {
        for child in &mut node.children {
            assign_x_and_depth(child, x, depth + 1, max_depth);
        }
        // For internal nodes, x-coordinate is the average of its children's x-coordinates
        node.x0 = node.children.iter().map(|c| c.x0).sum::<f64>() / node.children.len() as f64;
    }
}

fn assign_y<T: Clone>(node: &mut Node<T>, height: f64, max_depth: usize) {
    node.y1 = node.y0 * height / max_depth as f64; // Assign y-coordinate based on depth
    for child in &mut node.children {
        assign_y(child, height, max_depth);
    }
}

fn scale_x<T: Clone>(node: &mut Node<T>, width: f64, max_x: f64) {
    node.x1 = node.x0 * width / max_x;
    for child in &mut node.children {
        scale_x(child, width, max_x);
    }
}

fn get_max_x<T: Clone>(node: &Node<T>) -> f64 {
    if node.children.is_empty() {
        node.x0
    } else {
        node.children.iter().map(|c| get_max_x(c)).fold(f64::NEG_INFINITY, f64::max)
    }
}