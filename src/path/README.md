# d3-path (Rust Port)

This module provides a builder-style API for SVG path generation, similar to [d3-path](https://github.com/d3/d3-path).

## Example
```rust
use rust_d3::path::Path;
let mut p = Path::new();
// p.move_to(0.0, 0.0);
// p.line_to(10.0, 10.0);
// p.close_path();
println!("{}", p.to_string());
```

## Status
- [ ] move_to, line_to, arc, close_path, etc. (in progress)
- [ ] Full SVG path parity
- [ ] Robust tests
