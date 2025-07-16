# Rust D3

This project is a Rust port of D3.js, aiming to provide data-driven document manipulation and visualization tools in Rust.

## Getting Started

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Build the project:
   ```sh
   cargo build
   ```
3. Run tests:
   ```sh
   cargo test
   ```

## Project Structure
- `src/lib.rs`: Main library code.
- `tests/`: Integration tests.

## Contributing
Contributions are welcome! Please open issues or pull requests for discussion.

## d3-time Parity (Time Intervals)

The `time` module provides D3.js-like time intervals, step intervals, custom week start intervals, and UTC intervals for flexible date/time manipulation and range generation.

- **Intervals**: `Second`, `Minute`, `Hour`, `Day`, `Week`, `Month`, `Year`
- **Step Intervals**: Use `.every(n)` or `time_every::<Interval>(n)` for intervals that advance by a custom step (e.g., every 2 days, every 3 weeks)
- **Custom Week Start Intervals**: `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`
- **UTC Intervals**: `UtcDay` (pattern for UtcWeek, UtcMonth, etc.)
- **Range Generation**: Generate Vecs of `chrono::NaiveDateTime` for any interval and step
- **Trait-based**: All intervals implement the `TimeInterval` trait for composability

### Example Usage

```rust
use rust_d3::time::{Day, Monday, UtcDay, time_every, TimeInterval};
use chrono::{NaiveDate, Utc};

let every_2_days = Day::every(2).unwrap();
let start = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
let stop = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(0, 0, 0).unwrap();
let range = every_2_days.range(start, stop, 1);
assert_eq!(range, vec![
    start,
    start + chrono::Duration::days(2),
    start + chrono::Duration::days(4),
    start + chrono::Duration::days(6),
]);

let monday = Monday;
let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
assert_eq!(monday.floor(d), NaiveDate::from_ymd_opt(2025, 7, 7).unwrap().and_hms_opt(0, 0, 0).unwrap());

let utc_day = UtcDay;
let d = Utc.with_ymd_and_hms(2025, 7, 8, 15, 30, 0).unwrap().naive_utc();
assert_eq!(utc_day.floor(d), Utc.with_ymd_and_hms(2025, 7, 8, 0, 0, 0).unwrap().naive_utc());

let every_3_weeks = time_every::<rust_d3::time::Week>(3).unwrap();
let start = NaiveDate::from_ymd_opt(2025, 7, 6).unwrap().and_hms_opt(0, 0, 0).unwrap();
let stop = NaiveDate::from_ymd_opt(2025, 8, 17).unwrap().and_hms_opt(0, 0, 0).unwrap();
let range = every_3_weeks.range(start, stop, 1);
assert_eq!(range, vec![
    start,
    start + chrono::Duration::days(21),
]);
```

See `src/time/README.md` for full API and more examples.

## D3 Selection API Parity Examples

The Rust D3 selection system provides full API parity with D3.js selection patterns. Below are example/test code snippets for all major D3 selection features:

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

See `src/selection/selection.rs` for full API and more advanced usage patterns.
