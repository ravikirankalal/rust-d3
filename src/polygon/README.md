# d3-polygon (Rust)

Polygon geometry utilities inspired by d3-polygon.

## Features
- Area calculation (`polygon_area`)
- Centroid calculation (`polygon_centroid`)
- Perimeter/length (`polygon_length`)
- Point-in-polygon test (`polygon_contains`)
- Convex hull (`polygon_hull`)

## Usage
```rust
use rust_d3::{polygon_area, polygon_centroid, polygon_length, polygon_contains, polygon_hull};

let pts = [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
let area = polygon_area(&pts); // 6.0
let (cx, cy) = polygon_centroid(&pts); // (1.333..., 1.0)
let len = polygon_length(&pts); // 12.0
let inside = polygon_contains(&pts, 1.0, 1.0); // true
let hull = polygon_hull(&pts); // returns the convex hull points
```

## Example: Convex Hull
```rust
let pts = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.5, 0.5)];
let hull = polygon_hull(&pts);
assert_eq!(hull.len(), 4);
```

## Tests
See `tests.rs` for robust test coverage.
