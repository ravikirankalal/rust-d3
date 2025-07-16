//! d3-interpolate README

This module provides Rust ports of d3-interpolate's core interpolators.

## Features
- Number interpolation: `interpolate_number(a, b, t)`
- Array interpolation: `interpolate_array(&[a], &[b], t)`
- String interpolation (numbers in strings): `interpolate_string(a, b, t)`
- RGB color interpolation: `interpolate_rgb(a, b, t)`

## Example
```rust
use rust_d3::interpolate::number::interpolate_number;
let x = interpolate_number(0.0, 10.0, 0.5); // 5.0

use rust_d3::interpolate::rgb::interpolate_rgb;
let color = interpolate_rgb("#ff0000", "#00ff00", 0.5); // "#808000"
```

## TODO
- Add HSL, LAB, and cubehelix color interpolation
- Add object interpolation (structs, maps)
- Add piecewise and quantize interpolators
- Add tests and more usage examples
