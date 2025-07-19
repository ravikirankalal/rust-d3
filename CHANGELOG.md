# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Crisp-Pixel Strategy**: Axis components now automatically adjust for high-DPR displays using crisp-pixel rendering
- `Axis::set_device_pixel_ratio(dpr)` method for customizing device pixel ratio
- `Axis::device_pixel_ratio()` getter method for retrieving current device pixel ratio
- Enhanced documentation with examples for high-DPR canvas usage
- Crisp-pixel strategy documentation explaining offset behavior

### Changed
- **BREAKING**: Default axis offset computation now uses device pixel ratio to determine crisp edges
  - Previous behavior: Always used 0.5 pixel offset
  - New behavior: Uses 0.5 for standard DPR (≤1.0), 0.0 for high DPR (>1.0)
  - **Migration**: If your code depends on a specific offset value, explicitly set it using `.offset(value)`
  - **Impact**: Visual appearance of axis lines may change on high-DPR displays (usually improved sharpness)

### Technical Details

The crisp-pixel strategy ensures that lines render sharply on different display densities:

```rust
// Old behavior (before this change)
let axis = axis_bottom(scale); // Always offset by 0.5

// New behavior (automatic DPR detection)
let axis = axis_bottom(scale); // Offset by 0.5 or 0.0 based on DPR

// Explicit control (recommended for consistent behavior)
let axis = axis_bottom(scale).offset(0.5); // Force specific offset
let axis = axis_bottom(scale).set_device_pixel_ratio(2.0); // Set custom DPR
```

For applications requiring consistent rendering across different displays, we recommend explicitly setting the offset value.
