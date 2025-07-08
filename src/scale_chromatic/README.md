//! d3-scale-chromatic README

This module provides Rust ports of d3-scale-chromatic color schemes and interpolators.

## Features
- Categorical palettes: CATEGORY10, ACCENT, etc.
- Sequential palettes/interpolators: VIRIDIS, interpolate_viridis, etc.
- Diverging palettes/interpolators: RDYLBU, interpolate_rdyblu, etc.

## Example
```rust
use rust_d3::scale_chromatic::categorical::CATEGORY10;
let color = CATEGORY10[0]; // "#1f77b4"

use rust_d3::scale_chromatic::sequential::interpolate_viridis;
let color = interpolate_viridis(0.5); // e.g. "#31688e"
```

## TODO
- Add more palettes (Paired, Set1, Set2, Set3, etc.)
- Add perceptual and cubehelix interpolators
- Add color interpolation helpers for smooth gradients
