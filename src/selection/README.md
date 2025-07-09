# d3-selection Parity (Rust)

This module provides a Rust port of D3.js's d3-selection API, supporting DOM-like selection, data binding, and manipulation.

## Features
- `select`, `select_all`: Select nodes by tag.
- `attr`, `style`, `property`, `classed`, `text`, `html`, `datum`: Set/get attributes, styles, properties, classes, text, HTML, and data.
- `data`, `enter`, `exit`: Data binding and join.
- `append`, `insert`, `remove`: DOM-like node manipulation.
- `filter`, `merge`, `order`, `sort_by`, `call`, `each`, `map`, `empty`, `node`, `size`, `nodes`, `children`, `select_child`, `select_children`, `select_parent`, `select_parents`, `raise`, `lower`: Selection utilities.
- `on`, `dispatch`: Event handling.
- `interrupt`: Interrupt transitions (stub).
- `clone_selection`: Deep copy of selection.

## Usage Examples

### 1. Basic Selection and Attribute Manipulation
```rust
use rust_d3::Selection;
let mut svg = Selection::select("svg");
svg.attr("width", "400").attr("height", "300");
let mut rect = svg.append("rect");
rect.attr("x", "10").attr("y", "10").attr("width", "100").attr("height", "50");
let mut circle = svg.append("circle");
circle.attr("cx", "200").attr("cy", "150").attr("r", "40");
```

### 2. Data Join, Enter, Exit
```rust
let mut sel = Selection::select_all("rect");
sel.data(&[1, 2, 3]);
let enter = sel.enter();
let exit = sel.exit();
```

### 3. Filtering and Merging
```rust
let mut sel = Selection::select_all("rect");
let filtered = sel.filter(|n| n.tag == "rect");
let merged = sel.merge(&filtered);
```

### 4. Event Handling
```rust
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

### 5. Class, Style, and Property
```rust
let mut sel = Selection::select("circle");
sel.classed("highlight", true).style("fill", "red").property("checked", "true");
```

### 6. Text, HTML, and Datum
```rust
let mut sel = Selection::select("text");
sel.text("Hello").html("<b>Bold</b>").datum("42");
```

### 7. Children, Parent, and Node Utilities
```rust
let mut sel = Selection::select("g");
let _rect = sel.append("rect");
let _circle = sel.append("circle");
let child = sel.select_child();
let children = sel.children();
let parent = sel.select_parent();
let is_empty = sel.empty();
let size = sel.size();
let first_node = sel.node();
```

### 8. Sorting, Ordering, and Cloning
```rust
let mut sel = Selection::select_all("rect");
sel.sort_by(|a, b| a.tag.cmp(&b.tag));
sel.order();
let clone = sel.clone_selection();
```

### 9. Interrupt (stub)
```rust
let mut sel = Selection::select("rect");
sel.interrupt();
```

See the [tests/selection.rs](../../tests/selection.rs) for more examples.

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
