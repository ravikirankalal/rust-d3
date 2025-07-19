# Axis Utilities

This module provides utility functions for axis rendering, with a focus on crisp pixel rendering across different display environments.

## CrispPixel Utilities (`crisp.rs`)

The crisp pixel utilities provide device-aware offset calculations for optimal line rendering on both standard and high-DPI displays.

### Functions

#### `default_offset() -> f64`
- **WASM/Web**: Returns the current device pixel ratio from `web_sys::window()`
- **Other platforms**: Returns `1.0`
- **Fallback**: Returns `1.0` if window access fails

#### `crisp_offset(dpr: f64) -> f64`
- **High DPI** (`dpr > 1.0`): Returns `0.0` for crisp lines on retina displays
- **Standard DPI** (`dpr <= 1.0`): Returns `0.5` for crisp lines on standard displays

#### `effective_offset(custom: Option<f64>) -> f64`
- **With custom value**: Uses the provided offset directly
- **Without custom value**: Automatically calculates using device pixel ratio

### Usage

The axis constructors now automatically use `effective_offset(None)` instead of hard-coded `0.5` values:

```rust
use rust_d3::axis::{axis_bottom, effective_offset};
use rust_d3::scale::linear::ScaleLinear;

let scale = ScaleLinear::new([0.0, 100.0], [0.0, 500.0]);

// Automatic crisp pixel detection
let axis = axis_bottom(scale);

// Manual override if needed
let axis_manual = axis_bottom(scale).offset(0.25);

// Or use the utility directly
let crisp_value = effective_offset(None);
```

### Implementation Details

- **Centralized Logic**: Replaces hard-coded `0.5` offset values throughout all axis renderers
- **Platform Aware**: Automatically detects device pixel ratio on WASM targets
- **Backward Compatible**: Still allows manual offset override via `.offset()` method
- **Well Tested**: Comprehensive unit tests ensure correct behavior across scenarios

### Migration

All existing axis code continues to work unchanged. The only difference is that new axes now automatically get optimal offset values instead of the hard-coded `0.5`.

If you need the old behavior, explicitly set the offset:

```rust
let axis = axis_bottom(scale).offset(0.5);
```
