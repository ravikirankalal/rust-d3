# d3-transition

This module provides tools for animating changes to selections over time. Transitions are a powerful feature for creating dynamic and intuitive visualizations.

## Features

-   **`transition.duration(ms)`**: Set the duration of the transition in milliseconds.
-   **`transition.delay(ms)`**: Set the delay before the transition starts.
-   **`transition.ease(fn)`**: Specify a custom easing function.
-   **`transition.attr(name, value)`**: Animate an attribute to a new value.
-   **`transition.style(name, value)`**: Animate a style to a new value.
-   **`transition.on(event, handler)`**: Add event listeners for `start` and `end` of the transition.
-   **`transition.remove()`**: Remove the selected nodes at the end of the transition.
-   **`transition.filter(predicate)`**: Filter the transition to include only nodes that satisfy the predicate.
-   **`transition.select(selector)`**: Select the first descendant of each node.
-   **`transition.selectAll(selector)`**: Select all descendants of each node.
-   **`transition.selection()`**: Get the underlying selection.
-   **`transition.transition()`**: Chain transitions to run sequentially.

## Example

```rust
use rust_d3::selection::Selection;
use rust_d3::transition::Transition;
use std::time::Duration;

let sel = Selection::select("body");
let t1 = Transition::new(sel)
    .duration(1000)
    .attr("fill", "red");

let t2 = t1.transition()
    .duration(500)
    .attr("fill", "blue");

// t2 will start after t1 finishes.
```