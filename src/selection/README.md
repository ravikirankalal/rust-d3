# d3-selection (Rust)

This module provides a D3.js-like selection API for Rust, enabling DOM-like data binding, manipulation, and event handling. It is designed for deep parity with D3.js, supporting robust, chainable, and testable workflows for SVG, data visualization, and more.

## Features
- `select`, `select_all`: Create selections by tag.
- `attr`, `style`, `property`, `classed`: Set attributes, styles, properties, and classes.
- `data`, `datum`, `enter`, `exit`: Data binding and data join.
- `append`, `insert`, `remove`: Manipulate children.
- `on`, `dispatch`: Event system (simulated).
- `filter`, `merge`, `each`, `map`, `children`, `select_child`, `select_children`, `select_parent`, `select_parents`.
- `raise`, `lower`, `sort_by`, `order`: Simulated DOM order manipulation.
- Chainable API, robust edge-case and integration tests.

## Usage Example
```rust
use rust_d3::Selection;

// Create a root selection and append SVG elements
let mut svg = Selection::select("svg");
svg.attr("width", "400").attr("height", "300");
let mut rect = svg.append("rect");
rect.attr("x", "10").attr("y", "10").attr("width", "100").attr("height", "50");
let mut circle = svg.append("circle");
circle.attr("cx", "200").attr("cy", "150").attr("r", "40");

// Data join
let mut sel = Selection::select_all("rect");
sel.data(&[1, 2, 3]);
let enter = sel.enter();
let exit = sel.exit();

// Event handling
let mut sel = Selection::select("rect");
let called = std::sync::Arc::new(std::sync::Mutex::new(0));
let called2 = called.clone();
sel.on("click", move || {
    let mut n = called2.lock().unwrap();
    *n += 1;
});
sel.dispatch("click");
assert_eq!(*called.lock().unwrap(), 1);
```

## Integration
- Works seamlessly with other rust-d3 modules (array, collection, format, scale, axis, shape, etc.).
- Use `.attr()`/`.text()` with formatted numbers, scales, or path data from other modules.

## Parity Notes
- All major D3.js selection methods are present.
- Real DOM integration and advanced event propagation are not supported (simulated only).
- All methods are chainable and tested for edge cases.

## See Also
- [d3-array (Rust)](../array/README.md)
- [d3-collection (Rust)](../collection/README.md)
- [d3-format (Rust)](../format/README.md)
- [d3-scale (Rust)](../scale/README.md)
- [d3-shape (Rust)](../shape/README.md)

---
For full API details, see the source code and tests.
