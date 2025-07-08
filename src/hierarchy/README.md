//! d3-hierarchy README

This module provides Rust implementations of the core d3-hierarchy layouts:
- General-purpose `Node` for tree structures
- `TreeLayout` for tidy trees
- `ClusterLayout` for dendrograms
- `TreemapLayout` for space-filling treemaps
- `PartitionLayout` for sunburst/partition diagrams

All layouts operate on a generic `Node<T>` structure and compute positions or values in-place.

## Example

```rust
use rust_d3::hierarchy::{Node, TreeLayout};

let mut root = Node::new("root");
root.add_child(Node::new("child1"));
root.add_child(Node::new("child2"));
let tree = TreeLayout::new();
tree.layout(&mut root);
```

## Integration Example: Hierarchy + Shape

You can combine `hierarchy` with the `shape` module to visualize trees as SVG paths:

```rust
use rust_d3::hierarchy::{Node, TreeLayout};
use rust_d3::shape::{Line, LinearCurve};

let mut root = Node::new((0.0, 0.0));
root.add_child(Node::new((1.0, 1.0)));
let tree = TreeLayout::new();
tree.layout(&mut root);
let mut line = Line::new()
    .x(|d, _| d.0)
    .y(|d, _| d.1)
    .curve(LinearCurve::default());
let path = line.generate(&[root.data, root.children[0].data]);
println!("SVG Path: {}", path);
```

## Features
- Traversal (`each`), value aggregation (`sum`), and parent pointers in `Node`
- All layouts assign x/y for visualization
- Designed for easy integration with other modules

## More Examples

### Aggregating values
```rust
use rust_d3::hierarchy::Node;
let mut root = Node::new(1);
root.add_child(Node::new(2));
root.add_child(Node::new(3));
let sum = root.sum(&|v| *v as f64);
assert_eq!(sum, 6.0);
```

### Traversing the tree
```rust
use rust_d3::hierarchy::Node;
let mut root = Node::new(0);
root.add_child(Node::new(1));
root.add_child(Node::new(2));
let mut count = 0;
root.each(|n| { count += 1; });
assert_eq!(count, 3);
```
