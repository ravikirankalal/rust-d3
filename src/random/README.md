# d3-random (Rust Port)

This module provides seeded and unseeded random number generators for common distributions, similar to [d3-random](https://github.com/d3/d3-random).

## Example
```rust
use rust_d3::random::random_uniform;
let x = random_uniform();
println!("{}", x);
```

## Status
- [x] Uniform random (0, 1)
- [ ] Normal, logNormal, exponential, etc. (to be added)
- [ ] Seeding support
- [ ] Robust tests
