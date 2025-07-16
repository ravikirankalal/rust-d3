# d3-dispatch (Rust port)

A Rust port of [d3-dispatch](https://github.com/d3/d3-dispatch). Provides event dispatching and listener registration.

## Features
- Register multiple listeners for named events
- Call all listeners for an event
- Thread-safe (Arc/Mutex)

## Usage Example
```rust
use rust_d3::dispatch::Dispatch;
let mut d = Dispatch::new();
d.on("foo", || println!("foo event"));
d.call("foo");
```
