// Only export the array module for isolated testing
pub mod array;
pub mod collection;
pub mod format; 
pub mod time;
pub mod scale;
pub mod axis;
pub mod shape;
pub mod hierarchy;

pub use axis::Axis;
pub use shape::{Symbol, SymbolType};

// Example integration: use hierarchy with shape for tree visualization
// use crate::hierarchy::{Node, TreeLayout};
// use crate::shape::{Line, LinearCurve};
//
// let mut root = Node::new((0.0, 0.0));
// root.add_child(Node::new((1.0, 1.0)));
// let tree = TreeLayout::new();
// tree.layout(&mut root);
// let mut line = Line::new()
//     .x(|d, _| d.0)
//     .y(|d, _| d.1)
//     .curve(LinearCurve::default());
// let path = line.generate(&[root.data, root.children[0].data]);
// println!("SVG Path: {}", path);