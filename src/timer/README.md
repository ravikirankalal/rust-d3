# d3-timer (Rust port)

A Rust port of [d3-timer](https://github.com/d3/d3-timer). Provides timer utilities for animation, scheduling, and repeated callbacks.

## Features
- Timer struct for repeated callbacks
- Start/stop control
- Threaded implementation (no async required)

## Usage Example
```rust
use rust_d3::timer::Timer;
let mut t = Timer::new(|| println!("tick"), 10);
t.start();
t.stop();
```
