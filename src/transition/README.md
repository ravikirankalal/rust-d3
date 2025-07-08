# d3-transition (Rust Port)

This module ports the [D3.js d3-transition](https://github.com/d3/d3-transition) API to Rust, enabling animated transitions for selections, attributes, and styles. Timing is simulated (no real DOM or animation frames), but the API is designed for deep D3.js parity and robust integration with other modules.

## Features
- **Transition struct**: Chainable API for animated changes to attributes and styles.
- **Timing**: Simulated using threads and sleep (no real DOM or animation frames).
- **Easing**: Built-in support for linear, quadratic, cubic, and bounce easing functions.
- **Event hooks**: Register handlers for `start` and `end` events (thread-safe, called at the right time).
- **Integration**: Works seamlessly with the `Selection` API. Any selection can start a transition via `.transition()`.
- **Chainable**: All methods return `Self` for fluent chaining.
- **Remove/Interrupt**: Stubs for `.remove()` and `.interrupt()` (parity, but not fully async/cancellable).

## Example Usage
```rust
use rust_d3::selection::Selection;
use rust_d3::transition::{Transition, ease_quad};
use std::thread;
use std::time::Duration;

let sel = Selection::select_all("rect");
let t = sel.transition()
    .duration(100)
    .delay(50)
    .ease(ease_quad)
    .attr("fill", "red")
    .style("stroke", "blue")
    .on("start", || println!("Transition started!"))
    .on("end", || println!("Transition ended!"));
thread::sleep(Duration::from_millis(200));
```

## API Parity
- `Transition::new(selection)`
- `.duration(ms)`, `.delay(ms)`, `.ease(fn)`, `.attr(name, value)`, `.style(name, value)`
- `.on(event, handler)` for `start` and `end` events
- `.remove()`, `.interrupt()`, `.transition()` (stubs for parity)
- Easing: `ease_linear`, `ease_quad`, `ease_cubic`, `ease_bounce`
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
- [d3-selection (Rust)](../selection/README.md)
- [D3.js d3-transition documentation](https://github.com/d3/d3-transition)
