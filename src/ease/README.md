# d3-ease (Rust Port)

This module ports the [D3.js d3-ease](https://github.com/d3/d3-ease) API to Rust. It provides a comprehensive set of easing functions for use in transitions, animations, and interactive behaviors.

## Features
- **All D3.js easing functions**: linear, quad, cubic, sin, exp, circle, back, bounce, elastic
- **In, Out, InOut variants** for all major easings
- **No macro dependencies**: All functions are implemented directly for clarity and portability
- **Tested**: Includes tests for all major easings
- **Integration**: Designed for seamless use with the d3-transition module

## Example Usage
```rust
use rust_d3::ease::*;

// Basic usage
let t = quad_inout(0.5); // 0.5
let b = bounce_out(0.5); // ~0.7656
let e = elastic_out(0.5); // ~1.0

// All variants
let _ = quad_in(0.3);
let _ = quad_out(0.3);
let _ = quad_inout(0.3);
let _ = cubic_in(0.7);
let _ = cubic_out(0.7);
let _ = cubic_inout(0.7);
let _ = sin_in(0.2);
let _ = sin_out(0.2);
let _ = sin_inout(0.2);
let _ = exp_in(0.4);
let _ = exp_out(0.4);
let _ = exp_inout(0.4);
let _ = circle_in(0.6);
let _ = circle_out(0.6);
let _ = circle_inout(0.6);
let _ = back_in(0.8);
let _ = back_out(0.8);
let _ = back_inout(0.8);

// Integration with d3-transition
use rust_d3::selection::Selection;
use rust_d3::transition::Transition;

let sel = Selection::select_all("rect");
let t = sel.transition()
    .duration(100)
    .delay(50)
    .ease(cubic_inout)
    .attr("fill", "red")
    .style("stroke", "blue");
```

## API
- `linear(t)`
- `quad_in(t)`, `quad_out(t)`, `quad_inout(t)`
- `cubic_in(t)`, `cubic_out(t)`, `cubic_inout(t)`
- `sin_in(t)`, `sin_out(t)`, `sin_inout(t)`
- `exp_in(t)`, `exp_out(t)`, `exp_inout(t)`
- `circle_in(t)`, `circle_out(t)`, `circle_inout(t)`
- `back_in(t)`, `back_out(t)`, `back_inout(t)`
- `bounce_out(t)`
- `elastic_out(t)`

## Limitations
- Only `out` variant for bounce and elastic (most common in D3)
- No macro-based API (all functions are explicit)

## Tests
- All major easings are tested for correctness

## See Also
- [D3.js d3-ease documentation](https://github.com/d3/d3-ease)
- [d3-transition (Rust)](../transition/README.md)
