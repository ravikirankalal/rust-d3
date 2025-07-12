# d3-quadtree (Rust)

Spatial index/quadtree inspired by d3-quadtree.

## Features
- Insert points with values
- Efficient spatial subdivision
- Find nearest point within a radius
- Remove points by location
- Visit all points
- Query all points in a bounding box (`query_range`)
- Clear the tree

## Usage
```rust
use rust_d3::Quadtree;

let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
qt.insert(1.0, 2.0, "a");
qt.insert(3.0, 4.0, "b");
qt.insert(8.0, 8.0, "c");

// Find nearest
let found = qt.find(1.0, 2.0, 0.1);
assert_eq!(found.unwrap().2, "a");

// Query all points in a bounding box
let found = qt.query_range((0.0, 0.0, 5.0, 5.0));
let vals: Vec<_> = found.iter().map(|p| p.2).collect();
assert!(vals.contains(&"a"));
assert!(vals.contains(&"b"));
assert!(!vals.contains(&"c"));

// Remove a point
let removed = qt.remove(1.0, 2.0, 0.1);
assert_eq!(removed, Some("a"));

// Visit all points
qt.visit(|(_, _, v)| println!("{}", v));

// Clear the tree
qt.clear();
assert_eq!(qt.len(), 0);
```

## Tests
See `tests.rs` for robust test coverage.
