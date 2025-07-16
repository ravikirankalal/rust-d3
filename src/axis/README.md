# d3-axis

The axis component renders human-readable reference marks for [scales](../scale/README.md). This alleviates one of the more tedious tasks in visualizing data.

## API Reference

Regardless of orientation, axes are always rendered at the origin. To change the position of the axis with respect to the chart, specify a transform attribute on the containing element.

The elements created by the axis are considered part of its public API. You can apply external stylesheets or modify the generated axis elements to customize the axis appearance.

An axis consists of a domain path representing the extent of the scale's domain, followed by tick elements representing each of the scale's ticks. Each tick has a tick line and a tick label.

The orientation of an axis is fixed; to change the orientation, remove the old axis and create a new axis.

### Constructor Functions

<a name="axis_top" href="#axis_top">#</a> **axis_top**(*scale*) 

Constructs a new top-oriented axis generator for the given [scale](../scale/README.md), with a [tick count](#axis_tick_count) of 10, a [tick size](#axis_tick_size) of 6 and [padding](#axis_tick_padding) of 3. In this orientation, ticks are drawn above the horizontal domain path.

```rust
use d3_rust::axis::axis_top;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_top(scale);
```

<a name="axis_right" href="#axis_right">#</a> **axis_right**(*scale*)

Constructs a new right-oriented axis generator for the given [scale](../scale/README.md), with a [tick count](#axis_tick_count) of 10, a [tick size](#axis_tick_size) of 6 and [padding](#axis_tick_padding) of 3. In this orientation, ticks are drawn to the right of the vertical domain path.

```rust
use d3_rust::axis::axis_right;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_right(scale);
```

<a name="axis_bottom" href="#axis_bottom">#</a> **axis_bottom**(*scale*)

Constructs a new bottom-oriented axis generator for the given [scale](../scale/README.md), with a [tick count](#axis_tick_count) of 10, a [tick size](#axis_tick_size) of 6 and [padding](#axis_tick_padding) of 3. In this orientation, ticks are drawn below the horizontal domain path.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale);
```

<a name="axis_left" href="#axis_left">#</a> **axis_left**(*scale*)

Constructs a new left-oriented axis generator for the given [scale](../scale/README.md), with a [tick count](#axis_tick_count) of 10, a [tick size](#axis_tick_size) of 6 and [padding](#axis_tick_padding) of 3. In this orientation, ticks are drawn to the left of the vertical domain path.

```rust
use d3_rust::axis::axis_left;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_left(scale);
```

### Configuration Methods

<a name="axis_tick_count" href="#axis_tick_count">#</a> *axis*.**tick_count**(*count*)

Sets the approximate number of ticks to be generated and returns the axis. The actual number of ticks may be different depending on the scale's domain and the tick generation algorithm.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_count(5); // Request approximately 5 ticks
```

<a name="axis_tick_arguments" href="#axis_tick_arguments">#</a> *axis*.**tick_arguments**(*arguments*)

Sets the *arguments* that will be passed to the scale's tick generation when the axis is rendered, and returns the axis generator. The meaning of the *arguments* depends on the axis' scale type: most commonly, the arguments are a suggested *count* for the number of ticks, and an optional format *specifier* to customize how the tick values are formatted.

This method has no effect if the scale does not implement tick generation, as with [band](../scale/README.md#band-scales) and [point](../scale/README.md#point-scales) scales. To set the tick values explicitly, use [*axis*.tick_values](#axis_tick_values). To set the tick format explicitly, use [*axis*.tick_format](#axis_tick_format).

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);

// Generate approximately 20 ticks
let axis = axis_bottom(scale.clone())
    .tick_arguments(vec![20.0]);

// Generate ticks with custom count and formatting hint
let axis_with_format = axis_bottom(scale)
    .tick_arguments(vec![10.0, 1.0]); // Count and format specifier hint
```

<a name="axis_tick_values" href="#axis_tick_values">#</a> *axis*.**tick_values**(*values*)

Sets the specified values to be used for ticks rather than using the scale's automatic tick generator. If *values* is empty, clears any previously-set explicit tick values and reverts back to the scale's tick generator.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_values(vec![1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0]);
```

The explicit tick values take precedence over the tick arguments set by [*axis*.tick_arguments](#axis_tick_arguments). However, any tick arguments will still be passed to the scale's tick formatting function if a tick format is not also set.

<a name="axis_tick_format" href="#axis_tick_format">#</a> *axis*.**tick_format**(*format*)

Sets the tick format function and returns the axis. The format function takes a numeric value and returns a formatted string. A format function of `None` indicates that the scale's default formatter should be used.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);

// Custom formatter for currency
let axis = axis_bottom(scale)
    .tick_format(|value| format!("${:.2}", value));
```

See [d3-format](../format/README.md) for help creating formatters.

<a name="axis_tick_size" href="#axis_tick_size">#</a> *axis*.**tick_size**(*size*)

Sets the [inner](#axis_tick_size_inner) and [outer](#axis_tick_size_outer) tick size to the specified value and returns the axis.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_size(10.0); // Sets both inner and outer tick size to 10 pixels
```

<a name="axis_tick_size_inner" href="#axis_tick_size_inner">#</a> *axis*.**tick_size_inner**(*size*)

Sets the inner tick size to the specified value and returns the axis. The inner tick size controls the length of the tick lines, offset from the native position of the axis.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_size_inner(8.0); // Inner tick lines are 8 pixels long
```

<a name="axis_tick_size_outer" href="#axis_tick_size_outer">#</a> *axis*.**tick_size_outer**(*size*)

Sets the outer tick size to the specified value and returns the axis. The outer tick size controls the length of the square ends of the domain path, offset from the native position of the axis. Thus, the "outer ticks" are not actually ticks but part of the domain path. An outer tick size of 0 suppresses the square ends of the domain path, instead producing a straight line.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_size_outer(0.0); // No square ends on domain path
```

<a name="axis_tick_padding" href="#axis_tick_padding">#</a> *axis*.**tick_padding**(*padding*)

Sets the padding to the specified value in pixels and returns the axis. The padding is the distance between the tick line and the tick label.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_padding(5.0); // 5 pixels between tick line and label
```

<a name="axis_offset" href="#axis_offset">#</a> *axis*.**offset**(*offset*)

Sets the offset to the specified value in pixels and returns the axis. The offset ensures crisp edges on different resolution devices.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .offset(0.5); // Half-pixel offset for crisp edges
```

<a name="axis_locale" href="#axis_locale">#</a> *axis*.**locale**(*locale*)

Sets the locale string for number formatting and returns the axis. This affects how numbers are formatted in tick labels.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .locale("en-US"); // Use US English locale for formatting
```

### Tick Generation Methods

<a name="axis_ticks" href="#axis_ticks">#</a> *axis*.**ticks**()

Returns the tick values that would be generated for this axis based on the current configuration. This method is useful for inspecting the ticks without rendering the axis.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale);
let ticks = axis.ticks();

for tick in ticks {
    println!("Tick value: {}, label: {}, position: {}", 
             tick.value, tick.label, tick.position);
}
```

<a name="axis_ticks_with" href="#axis_ticks_with">#</a> *axis*.**ticks_with**(*tick_values*)

Returns the tick values using the specified values instead of the scale's automatic tick generator. This is useful for generating ticks with custom values.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale);
let custom_values = vec![0.0, 25.0, 50.0, 75.0, 100.0];
let ticks = axis.ticks_with(Some(&custom_values));
```

### Layout Generation

<a name="axis_layout" href="#axis_layout">#</a> *axis*.**layout**(*axis_start*, *axis_end*, *ticks*)

Generates an `AxisLayout` containing all the information needed to render the axis. This includes the ticks, their positions, and styling information.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale);
let ticks = axis.ticks();
let layout = axis.layout(0.0, 500.0, ticks);

// Use layout for rendering
println!("Axis orientation: {:?}", layout.orientation);
println!("Number of ticks: {}", layout.ticks.len());
```

### Extended Configuration Methods

<a name="axis_grid" href="#axis_grid">#</a> *axis*.**grid**(*show*)

Enables or disables grid lines for the axis and returns the axis.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .grid(true); // Enable grid lines
```

<a name="axis_grid_style" href="#axis_grid_style">#</a> *axis*.**grid_style**(*style*)

Sets the styling for grid lines and returns the axis.

```rust
use d3_rust::axis::{axis_bottom, GridStyle};
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let grid_style = GridStyle {
    color: "#e0e0e0".to_string(),
    width: 1.0,
    dasharray: Some("2,2".to_string()),
};
let axis = axis_bottom(scale)
    .grid(true)
    .grid_style(grid_style);
```

<a name="axis_title" href="#axis_title">#</a> *axis*.**title**(*title*)

Sets the axis title and returns the axis.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .title("Value"); // Set axis title
```

<a name="axis_title_style" href="#axis_title_style">#</a> *axis*.**title_style**(*style*)

Sets the styling for the axis title and returns the axis.

```rust
use d3_rust::axis::{axis_bottom, TitleStyle};
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let title_style = TitleStyle {
    font: "14px Arial".to_string(),
    color: "#333".to_string(),
    position: Some((250.0, 40.0)),
};
let axis = axis_bottom(scale)
    .title("Value")
    .title_style(title_style);
```

<a name="axis_minor_ticks" href="#axis_minor_ticks">#</a> *axis*.**minor_ticks**(*ticks*)

Sets the values for minor ticks and returns the axis.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .minor_ticks(vec![5.0, 15.0, 25.0, 35.0, 45.0, 55.0, 65.0, 75.0, 85.0, 95.0]);
```

<a name="axis_minor_tick_size" href="#axis_minor_tick_size">#</a> *axis*.**minor_tick_size**(*size*)

Sets the size of minor ticks and returns the axis.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .minor_tick_size(3.0); // Minor ticks are 3 pixels long
```

<a name="axis_tick_label_angle" href="#axis_tick_label_angle">#</a> *axis*.**tick_label_angle**(*angle*)

Sets the rotation angle for tick labels in degrees and returns the axis.

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_label_angle(45.0); // Rotate labels 45 degrees
```

<a name="axis_tick_label_style" href="#axis_tick_label_style">#</a> *axis*.**tick_label_style**(*style*)

Sets the styling for tick labels and returns the axis.

```rust
use d3_rust::axis::{axis_bottom, TickLabelStyle};
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let label_style = TickLabelStyle {
    font: "12px Arial".to_string(),
    color: "#666".to_string(),
    padding: Some(2.0),
};
let axis = axis_bottom(scale)
    .tick_label_style(label_style);
```

<a name="axis_axis_line_style" href="#axis_axis_line_style">#</a> *axis*.**axis_line_style**(*style*)

Sets the styling for the axis domain line and returns the axis.

```rust
use d3_rust::axis::{axis_bottom, AxisLineStyle};
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let line_style = AxisLineStyle {
    color: "#333".to_string(),
    width: 2.0,
    dasharray: None,
};
let axis = axis_bottom(scale)
    .axis_line_style(line_style);
```

### Working with Different Scale Types

#### Linear Scales

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_count(10)
    .tick_format(|d| format!("{:.1}", d));
```

#### Logarithmic Scales

```rust
use d3_rust::axis::axis_left;
use d3_rust::scale::log::ScaleLog;

let scale = ScaleLog::new([1.0, 1000.0], [0.0, 300.0]);
let axis = axis_left(scale)
    .tick_count(5)
    .tick_format(|d| format!("{:.0}", d));
```

#### Time Scales

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::time::ScaleTime;
use chrono::NaiveDate;

let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
let end = NaiveDate::from_ymd_opt(2020, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
let scale = ScaleTime::new([start, end], [0.0, 500.0]);
let axis = axis_bottom(scale)
    .tick_count(12); // Approximately monthly ticks
```

#### Band Scales

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::band::ScaleBand;

let scale = ScaleBand::new(vec!["A", "B", "C", "D", "E"], [0.0, 500.0]);
let axis = axis_bottom(scale);
// Note: tick_count has no effect on band scales
```

#### Point Scales

```rust
use d3_rust::axis::axis_bottom;
use d3_rust::scale::point::ScalePoint;

let scale = ScalePoint::new(vec!["Small", "Medium", "Large"], [0.0, 500.0]);
let axis = axis_bottom(scale);
// Note: tick_count has no effect on point scales
```

### Complete Example

```rust
use d3_rust::axis::{axis_bottom, axis_left, GridStyle, TitleStyle};
use d3_rust::scale::linear::ScaleLinear;

// Create scales
let x_scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);
let y_scale = ScaleLinear::new([0.0, 50.0], [300.0, 0.0]);

// Create styled grid
let grid_style = GridStyle {
    color: "#e0e0e0".to_string(),
    width: 1.0,
    dasharray: Some("2,2".to_string()),
};

// Create axes with full configuration
let x_axis = axis_bottom(x_scale)
    .tick_count(10)
    .tick_format(|d| format!("{:.0}%", d))
    .tick_padding(5.0)
    .grid(true)
    .grid_style(grid_style.clone())
    .title("Percentage");

let y_axis = axis_left(y_scale)
    .tick_count(5)
    .tick_format(|d| format!("${:.0}", d))
    .tick_padding(5.0)
    .grid(true)
    .grid_style(grid_style)
    .title("Value");

// Generate ticks and layout
let x_ticks = x_axis.ticks();
let y_ticks = y_axis.ticks();
let x_layout = x_axis.layout(0.0, 500.0, x_ticks);
let y_layout = y_axis.layout(0.0, 300.0, y_ticks);

// Use layouts for rendering...
```

## Data Structures

### Tick

Represents a single tick mark on an axis.

```rust
pub struct Tick {
    pub value: f64,     // The data value
    pub label: String,  // The formatted label
    pub position: f64,  // The pixel position on the axis
}
```

### AxisLayout

Contains all information needed to render an axis.

```rust
pub struct AxisLayout {
    pub orientation: AxisOrientation,
    pub ticks: Vec<Tick>,
    pub tick_size_inner: f64,
    pub tick_size_outer: f64,
    pub tick_padding: f64,
    pub axis_start: f64,
    pub axis_end: f64,
    pub offset: f64,
}
```

### Styling Structures

#### GridStyle

```rust
pub struct GridStyle {
    pub color: String,
    pub width: f64,
    pub dasharray: Option<String>,
}
```

#### TitleStyle

```rust
pub struct TitleStyle {
    pub font: String,
    pub color: String,
    pub position: Option<(f64, f64)>,
}
```

#### TickLabelStyle

```rust
pub struct TickLabelStyle {
    pub font: String,
    pub color: String,
    pub padding: Option<f64>,
}
```

#### AxisLineStyle

```rust
pub struct AxisLineStyle {
    pub color: String,
    pub width: f64,
    pub dasharray: Option<String>,
}
```

## Backward Compatibility

All existing code using the axis module will continue to work unchanged. The API is designed to be additive, meaning:

1. All existing method signatures remain the same
2. New methods have sensible defaults
3. The core `Axis` struct maintains all existing fields
4. Constructor functions (`axis_bottom`, `axis_left`, etc.) retain their original behavior

Migration from older versions requires no code changes - all existing functionality is preserved while new features are opt-in through additional method calls.
