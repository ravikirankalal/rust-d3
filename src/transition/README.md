# d3-transition (Rust Port)

This module ports the [D3.js d3-transition](https://github.com/d3/d3-transition) API to Rust, enabling animated transitions for selections, attributes, and styles. Timing is simulated (no real DOM or animation frames), but the API is designed for deep D3.js parity and robust integration with other modules.

## Features
- **Transition struct**: Chainable API for animated changes to attributes and styles.
- **Timing**: Simulated using threads and sleep (no real DOM or animation frames).
- **Easing**: Built-in support for all D3.js easing functions via the `ease` module (see `src/ease/`).
- **Event hooks**: Register handlers for `start` and `end` events (thread-safe, called at the right time).
- **Integration**: Works seamlessly with the `Selection` API. Any selection can start a transition via `.transition()`.
- **Chainable**: All methods return `Self` for fluent chaining.
- **Remove/Interrupt**: Stubs for `.remove()` and `.interrupt()` (parity, but not fully async/cancellable).

## Example Usage
```rust
use rust_d3::selection::Selection;
use rust_d3::transition::Transition;
use rust_d3::ease::*;
use std::thread;
use std::time::Duration;

// Basic transition with quad easing
let sel = Selection::select_all("rect");
let t = sel.transition()
    .duration(100)
    .delay(50)
    .ease(quad_inout)
    .attr("fill", "red")
    .style("stroke", "blue");
thread::sleep(Duration::from_millis(200));

// Using bounce and elastic easings
let t2 = sel.transition()
    .duration(120)
    .ease(bounce_out)
    .attr("height", "40");
let t3 = sel.transition()
    .duration(150)
    .ease(elastic_out)
    .attr("width", "80");

// With event hooks
let t4 = sel.transition()
    .duration(80)
    .ease(cubic_in)
    .on("start", || println!("Started!"))
    .on("end", || println!("Ended!"))
    .attr("opacity", "0.5");
```

## API Parity
- `Transition::new(selection)`
- `.duration(ms)`, `.delay(ms)`, `.ease(fn)`, `.attr(name, value)`, `.style(name, value)`
- `.on(event, handler)` for `start` and `end` events
- `.remove()`, `.interrupt()`, `.transition()` (stubs for parity)
- Easing: All D3.js easings via `ease` module (linear, quad, cubic, sin, exp, circle, back, bounce, elastic, and in/out/inout variants)
- Integration: `Selection::transition()`

## Limitations
- No real DOM or animation frames (timing is simulated)
- No true async/cancellable transitions (interrupt is a stub)
- Event system is thread-based, not event-loop based
- No support for transition chaining or advanced D3.js features (yet)

## Tests
- Integration and API tests for all features
- Event hooks and chainability are tested

## See Also
- [d3-ease (Rust)](../ease/README.md)
- [d3-selection (Rust)](../selection/README.md)
- [D3.js d3-transition documentation](https://github.com/d3/d3-transition)
