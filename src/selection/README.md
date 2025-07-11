# D3 Selection API Parity (Rust)

This module provides a Rust implementation of D3.js's selection system, supporting all major selection patterns and features.

## Example/Test Code for All D3 Patterns

```rust
use rust_d3::selection::Selection;

// Create a root SVG node
let mut sel = Selection::create("svg");

// Append a group and set attributes
sel.append("g").attr("class", "group");

// Select all groups and bind data
sel.select_all(Some("g")).data(&["a", "b", "c"]);

// Set attributes and styles using functions
sel.attr_fn("id", |node, i| format!("node-{}", i));
sel.style("fill", "red");
sel.style_fn("stroke", |node, _| format!("#{}", node.tag.len()));

// Class manipulation
sel.classed("active", true);

// Set text and HTML
sel.text("Hello");
sel.html("<span>World</span>");

// Event handling
sel.on("click", |node| { /* ... */ });

// Iteration and chaining
sel.each(|node| { /* ... */ });
sel.call(|s| { s.attr("foo", "bar"); });

// Merging selections
let other_sel = Selection::create("rect");
sel.merge(&other_sel);

// Ordering
sel.order().raise().lower();

// Access nodes and properties
let node = sel.node();
let nodes = sel.nodes();
let size = sel.size();
let is_empty = sel.empty();
let parent = sel.parent();
let children = sel.children();
let cloned = sel.clone();
let deep_cloned = sel.clone_selection();

// Find and select
let found = sel.find_all("rect");
let selected = sel.select("g");
let selected_by = sel.select_by("g.active");

// Sorting and mapping
sel.sort_by(|a, b| a.tag.cmp(&b.tag));
let mapped: Vec<_> = sel.map(|node| node.tag.clone());

// Child and parent selection
let child_sel = sel.select_child("rect");
let parent_sel = sel.select_parent("svg");

// Animation/event stubs
sel.transition().interrupt().dispatch("custom");

// Data join pattern
let update = sel.update();
let enter = sel.enter();
let exit = sel.exit();
sel.join(|enter| { /* ... */ }, |update| { /* ... */ }, |exit| { /* ... */ });
sel.join_nodes("g");
```

See `selection.rs` for full API and advanced usage patterns.
