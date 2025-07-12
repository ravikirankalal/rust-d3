# Color Module

This module provides color manipulation and conversion utilities, inspired by D3.js's color module. It includes support for RGB, HSL, and Lab color spaces, as well as color conversion functions.

## Features
- RGB, HSL, and Lab color structs and conversions
- Color parsing and formatting
- Utilities for color interpolation and manipulation

## Modules
- `rgb.rs`: RGB color representation and utilities
- `hsl.rs`: HSL color representation and utilities
- `lab.rs`: Lab color representation and utilities
- `convert.rs`: Color space conversion functions

## Usage Examples
```rust
use rust_d3::color::{Rgb, Hsl, Lab};

// Create an RGB color
let rgb = Rgb::new(255, 0, 0);

// Convert RGB to HSL
let hsl = Hsl::from(&rgb);

// Convert RGB to Lab
let lab = Lab::from(&rgb);

// Parse a color from a hex string
let rgb2 = Rgb::from_hex("#00ff00").unwrap();

// Format a color as a hex string
let hex = rgb2.to_hex();

// Interpolate between two colors
let c1 = Rgb::new(255, 0, 0);
let c2 = Rgb::new(0, 0, 255);
let mid = c1.interpolate(&c2, 0.5); // Should be purple

// Manipulate color lightness
let lighter = hsl.lighter(0.2);
let darker = hsl.darker(0.2);
```

## Parity Checklist
See `../../D3_RUST_PARITY_CHECKLIST.md` for parity status with D3.js color module.
