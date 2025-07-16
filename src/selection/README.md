# D3 Selection API Parity (Rust)

This module provides a Rust implementation of D3.js's selection system, supporting all major selection patterns and features.

## D3 Selection API Parity Matrix

### Core Selection Methods
| Feature | Status | Description |
|---------|--------|-------------|
| `select()` | ✅ | Select first matching descendant |
| `select_all()` | ✅ | Select all matching descendants |
| `select_child()` | ✅ | Select direct child by tag |
| `select_parent()` | ✅ | Select parent by tag |
| `select_by()` | ✅ | Select by CSS selector |
| `filter()` | ✅ | Filter selection by predicate |
| `find()` | ✅ | Find first matching node |
| `find_all()` | ✅ | Find all matching nodes |

### Data Binding
| Feature | Status | Description |
|---------|--------|-------------|
| `data()` | ✅ | Bind data to selection |
| `data_with_key()` | ✅ | Bind data with key function |
| `datum()` | ✅ | Set datum for all nodes |
| `enter()` | ✅ | Get enter selection |
| `update()` | ✅ | Get update selection |
| `exit()` | ✅ | Get exit selection |
| `join()` | ✅ | General data join pattern |

### DOM Manipulation
| Feature | Status | Description |
|---------|--------|-------------|
| `append()` | ✅ | Append new child element |
| `insert()` | ✅ | Insert before reference node |
| `remove()` | ✅ | Remove nodes from DOM |
| `clone()` | ✅ | Shallow clone selection |
| `deep_clone()` | ✅ | Deep clone with structure |

### Attributes & Properties
| Feature | Status | Description |
|---------|--------|-------------|
| `attr()` | ✅ | Set/get attribute |
| `attr_fn()` | ✅ | Set attribute with function |
| `style()` | ✅ | Set/get style property |
| `style_fn()` | ✅ | Set style with function |
| `property()` | ✅ | Set/get DOM property |
| `classed()` | ✅ | Add/remove CSS classes |
| `text()` | ✅ | Set/get text content |
| `html()` | ✅ | Set/get HTML content |

### Events & Iteration
| Feature | Status | Description |
|---------|--------|-------------|
| `on()` | ✅ | Attach event handler |
| `each()` | ✅ | Iterate over nodes |
| `call()` | ✅ | Call function on selection |
| `map()` | ✅ | Map nodes to values |

### Ordering & Hierarchy
| Feature | Status | Description |
|---------|--------|-------------|
| `order()` | ✅ | Reorder DOM elements |
| `sort_by()` | ✅ | Sort selection by comparator |
| `raise()` | ✅ | Move to end of parent |
| `lower()` | ✅ | Move to start of parent |
| `parent()` | ✅ | Get parent selection |
| `children()` | ✅ | Get children selection |

### Utilities
| Feature | Status | Description |
|---------|--------|-------------|
| `merge()` | ✅ | Merge two selections |
| `node()` | ✅ | Get first node |
| `nodes()` | ✅ | Get all nodes |
| `size()` | ✅ | Get selection size |
| `empty()` | ✅ | Check if selection is empty |

## Quick Start Examples

### Basic Selection and Manipulation
```rust
use rust_d3::selection::Selection;

// Create SVG and add elements
let mut svg = Selection::create("svg");
svg.attr("width", "400").attr("height", "300");

// Append group with styling
let mut group = svg.append("g");
group.attr("class", "chart").style("fill", "steelblue");
```

### Data Binding Pattern
```rust
use rust_d3::selection::Selection;

let mut svg = Selection::create("svg");
let data = vec![10, 20, 30, 40];

// Standard D3 data join
let join = svg.select_all(Some("rect")).data(&data);

// Handle enter selection
let mut enter = join.enter;
enter.join("rect")
    .attr_fn("x", |_, i, _| (i * 50).to_string())
    .attr_fn("height", |node, _, _| node.data.as_ref().unwrap().clone())
    .style("fill", "blue");

// Handle exit selection
join.exit.remove();
```

### Advanced Selectors
```rust
use rust_d3::selection::Selection;

let mut svg = Selection::create("svg");
svg.append("g").classed("axis", true).classed("major", true);

// Select by CSS selector
let axis_major = svg.select_by("g.axis.major");
axis_major.style("stroke", "black");

// Complex attribute functions
axis_major.attr_fn("transform", |node, i, prev| {
    format!("translate({}, {})", i * 100, node.tag.len() * 10)
});
```

### Corrected Behaviors

#### Empty String Attribute Removal
```rust
// Setting empty string removes attribute (D3 behavior)
selection.attr("width", "100");
selection.attr("width", ""); // Removes width attribute
```

#### Style Function with Previous Value
```rust
// Style functions receive previous value as third parameter
selection.style_fn("width", |node, index, prev_value| {
    let prev = prev_value.unwrap_or("0px".to_string());
    format!("calc({} + 10px)", prev)
});
```

#### Keyed Data Joins
```rust
// Key function for stable object constancy
let data = vec!["apple", "banana", "cherry"];
let join = selection.data_with_key(&data, |d, i| d.to_string());
// Elements are matched by key, not position
```

See `selection.rs` for complete API documentation and implementation details.
