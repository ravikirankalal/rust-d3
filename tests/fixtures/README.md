# D3 Axis Reference Fixtures

**MANDATORY**: All axis tests in this project MUST be powered by D3.js fixtures from this directory to ensure complete D3.js compatibility.

This directory contains reference data generated from the official D3.js axis implementation to validate the Rust implementation.

## Files

- `generate_reference.js` - JavaScript script that generates reference data using D3.js
- `package.json` - Node.js dependencies for the reference generator
- `linear_reference.json` - Linear scale axis reference data
- `time_reference.json` - Time scale axis reference data  
- `d3_axis_reference.json` - Combined reference data
- `test_comparison.rs` - Rust program to compare Rust implementation with D3 reference
- `README.md` - This file

## Reference Data Structure

Each reference file contains tick data with the following structure:

```json
{
  "value": "tick value (number for linear, ISO string for time)",
  "position": "scaled position (0-100 range)",
  "label": "formatted label string"
}
```

## Linear Scale Test Cases

- **Normal domain [0, 10]**: Standard linear scale with 5 ticks
- **Zero span domain [0, 0]**: Edge case where min equals max
- **Single value domain [5, 5]**: Same as zero span but with non-zero value

## Time Scale Test Cases

- **Seconds**: 4-second span (2020-01-01 00:00:00 to 00:00:04)
- **Minutes**: 5-minute span (2020-01-01 00:00:00 to 00:05:00)
- **Hours**: 4-hour span (2020-01-01 00:00:00 to 04:00:00)
- **Days**: 4-day span (2020-01-01 to 2020-01-05)
- **Months**: 4-month span (2020-01-01 to 2020-05-01)
- **Years**: 4-year span (2020-01-01 to 2024-01-01)

## Usage

### Regenerate Reference Data

```bash
cd tests/fixtures
npm install
node generate_reference.js
```

### Compare with Rust Implementation

```bash
cd tests/fixtures
cargo run --bin test_comparison
```

### Run Test Suite

```bash
cargo test axis
```

## Notes

- The JavaScript generator uses D3.js version 4+ axis and scale modules
- Time values are stored as ISO strings for JSON serialization
- Position values are scaled to 0-100 range for consistency
- Labels reflect D3's default formatting behavior
- These fixtures help ensure the Rust implementation maintains D3 parity

# Coverage Matrix

This section defines which **D3 scenarios** must be added to fixtures so every behavior is covered, based on the gap analysis between D3.js reference implementation and Rust implementation.

## Scale Type Coverage

### Linear Scale Scenarios
| Scenario | Domain | Range | Tick Count | Purpose | Status |
|----------|--------|-------|------------|---------|--------|
| **Basic Ticks** | [0, 10] | [0, 100] | 5 | Standard linear progression | ✅ Covered |
| **Custom Ticks** | [0, 10] | [0, 100] | explicit [2,5,8] | Custom tick value override | ✅ Covered |
| **Negative Domain** | [-10, 10] | [0, 200] | 10 | Zero-crossing scenarios | ✅ Covered |
| **Fractional Domain** | [0.1, 0.9] | [0, 800] | 8 | Sub-integer precision handling | ✅ Covered |
| **Large Numbers** | [1e6, 1e7] | [0, 1000] | 10 | Scientific notation formatting | ✅ Covered |
| **Small Numbers** | [0.001, 0.009] | [0, 900] | 9 | High decimal precision | ✅ Covered |
| **Reversed Range** | [0, 100] | [500, 0] | 10 | Inverted axis rendering | ✅ Covered |
| **Empty Domain** | [0, 0] | [0, 0] | 5 | Zero-span edge case | ✅ Covered |
| **Single Value** | [5, 5] | [10, 10] | 1 | Single-point domain | ✅ Covered |
| **Huge Range** | [0, 1] | [0, 1e8] | 10 | Extreme scaling scenarios | ⚠️ Need to add |
| **Reversed Domain** | [100, 0] | [0, 500] | 10 | Inverted domain logic | ⚠️ Need to add |

### Logarithmic Scale Scenarios
| Scenario | Domain | Range | Base | Tick Count | Purpose | Status |
|----------|--------|-------|------|------------|---------|--------|
| **Basic Decades** | [1, 1000] | [0, 100] | 10 | Standard log decades | ✅ Covered |
| **Multiple Decades** | [1, 10000] | [0, 400] | 10 | 4+ decade span | ✅ Covered |
| **Base 2** | [1, 256] | [0, 800] | 2 | Binary logarithm | ✅ Covered |
| **Custom Values** | [1, 1000] | [0, 200] | 10 | explicit [1,10,100,1000] | ✅ Covered |
| **Fractional Start** | [0.01, 1000] | [0, 300] | 10 | Sub-unit domain start | ⚠️ Need to add |
| **Base E (Natural)** | [1, 100] | [0, 500] | e | Natural logarithm | ⚠️ Need to add |
| **Large Base** | [1, 1000000] | [0, 600] | 1000 | Unusual base values | ⚠️ Need to add |

### Time Scale Scenarios
| Scenario | Domain | Range | Tick Count | Interval | Purpose | Status |
|----------|--------|-------|------------|----------|---------|--------|
| **Seconds** | [2023-01-01 12:00:00, 12:00:30] | [0, 300] | 6 | sub-minute | ✅ Covered |
| **Minutes** | [2023-01-01 12:00:00, 12:30:00] | [0, 300] | 6 | sub-hour | ✅ Covered |
| **Hours** | [2023-01-01 08:00:00, 20:00:00] | [0, 600] | 6 | sub-day | ✅ Covered |
| **Days** | [2023-01-01, 2023-01-15] | [0, 700] | 7 | sub-month | ✅ Covered |
| **Months** | [2023-01-01, 2023-12-01] | [0, 1100] | 12 | sub-year | ✅ Covered |
| **Years** | [2020-01-01, 2030-01-01] | [0, 1000] | 10 | multi-year | ✅ Covered |
| **Reverse Domain** | [2023-12-31, 2023-01-01] | [0, 365] | 12 | backwards time | ✅ Covered |
| **Custom Values** | [2023-01-01, 2023-01-10] | [0, 900] | explicit dates | custom intervals | ✅ Covered |
| **Milliseconds** | [2023-01-01 12:00:00.000, 12:00:00.500] | [0, 500] | 10 | sub-second | ⚠️ Need to add |
| **Weeks** | [2023-01-01, 2023-03-01] | [0, 800] | 8 | weekly intervals | ⚠️ Need to add |
| **DST Transition** | [2023-03-12, 2023-03-13] | [0, 240] | 24 | daylight saving | ⚠️ Need to add |
| **Leap Year** | [2024-02-28, 2024-03-01] | [0, 200] | 4 | February 29 handling | ⚠️ Need to add |

### Band Scale Scenarios
| Scenario | Domain | Range | Inner Padding | Outer Padding | Purpose | Status |
|----------|--------|-------|---------------|---------------|---------|--------|
| **Basic** | ["a", "b", "c"] | [0, 120] | 0.1 | 0.1 | Standard band layout | ✅ Covered |
| **With Padding** | ["Alpha", "Beta", "Gamma", "Delta"] | [0, 400] | 0.2 | 0.1 | Complex padding | ✅ Covered |
| **No Padding** | ["x", "y", "z"] | [0, 300] | 0.0 | 0.0 | No gaps between bands | ⚠️ Need to add |
| **Large Count** | ["item1"..."item50"] | [0, 1000] | 0.05 | 0.02 | Many categories | ⚠️ Need to add |
| **Align Variants** | ["A", "B", "C"] | [0, 100] | 0.1 | 0.1 | align = 0.0, 0.5, 1.0 | ⚠️ Need to add |

### Point Scale Scenarios
| Scenario | Domain | Range | Padding | Purpose | Status |
|----------|--------|-------|---------|---------|--------|
| **Basic** | ["x", "y", "z"] | [0, 100] | 0.5 | Standard point layout | ✅ Covered |
| **With Padding** | ["First", "Second", "Third"] | [0, 300] | 0.25 | Custom padding | ✅ Covered |
| **No Padding** | ["A", "B", "C"] | [0, 200] | 0.0 | Points at range edges | ⚠️ Need to add |
| **Single Point** | ["only"] | [0, 100] | 0.5 | Edge case handling | ⚠️ Need to add |
| **Max Padding** | ["p1", "p2"] | [0, 100] | 1.0 | Extreme padding | ⚠️ Need to add |

## Layout and Offset Coverage

### Orientation Scenarios
| Orientation | Tick Size Inner | Tick Size Outer | Tick Padding | Offset | Purpose | Status |
|-------------|------------------|------------------|--------------|--------|---------|--------|
| **Bottom** | 8.0 | 12.0 | 5.0 | 0.5 | Standard bottom axis | ✅ Covered |
| **Left** | 6.0 | 6.0 | 3.0 | 0.5 | Standard left axis | ✅ Covered |
| **Top** | 5.0 | 7.0 | 2.0 | 0.5 | Standard top axis | ✅ Covered |
| **Right** | 4.0 | 8.0 | 4.0 | 0.5 | Standard right axis | ✅ Covered |

### Critical Offset Scenarios (devicePixelRatio Awareness)
| Scenario | Offset Value | Device Pixel Ratio | Expected Transform | Purpose | Status |
|----------|--------------|-------------------|-------------------|---------|--------|
| **Standard Crisp** | 0.5 | 1.0 | translate(0.5,0) or (0,0.5) | Crisp 1px lines | ✅ Covered |
| **High DPI** | 0.0 | 2.0+ | translate(0,0) | No half-pixel on Retina | ❌ **Missing** |
| **Custom Offset** | 1.25 | 1.0 | translate(1.25,0) or (0,1.25) | Custom offset values | ✅ Covered |
| **Zero Offset** | 0.0 | 1.0 | translate(0,0) | Disable crisp rendering | ✅ Covered |

### Transform String Scenarios
| Orientation | Offset | Expected Transform | Transform Composition | Status |
|-------------|--------|-------------------|----------------------|--------|
| **Bottom/Top** | 0.5 | "translate(0.5,0)" | Horizontal offset | ✅ Covered |
| **Left/Right** | 0.5 | "translate(0,0.5)" | Vertical offset | ✅ Covered |
| **With Existing** | 0.5 | "existing translate(0.5,0)" | Chained transforms | ⚠️ **Gap: Not tested** |
| **Conditional Apply** | 0.0 | Should skip offset | Time renderer bug | ❌ **Known Bug** |

## Tick and Axis Sizing Coverage

### Tick Size Scenarios
| Scenario | Inner Size | Outer Size | Purpose | Visual Impact | Status |
|----------|------------|------------|---------|---------------|--------|
| **Standard** | 6.0 | 6.0 | Default D3 sizing | Standard ticks | ✅ Covered |
| **Inner Only** | 8.0 | 0.0 | No outer ticks | Clean domain line | ⚠️ Need to add |
| **Outer Only** | 0.0 | 12.0 | No inner ticks | Domain endpoint emphasis | ⚠️ Need to add |
| **Different Sizes** | 4.0 | 10.0 | Asymmetric sizing | Visual hierarchy | ✅ Covered |
| **Large Ticks** | 20.0 | 25.0 | Prominent ticks | High visibility | ⚠️ Need to add |
| **Negative Ticks** | -5.0 | -8.0 | Inward-facing ticks | Alternative styling | ⚠️ Need to add |

### Minor Tick Scenarios
| Scenario | Major Ticks | Minor Tick Values | Minor Size | Purpose | Status |
|----------|-------------|-------------------|------------|---------|--------|
| **Linear Minor** | [0,5,10] | [1,2,3,4,6,7,8,9] | 3.0 | Sub-divisions | ✅ Covered |
| **Log Minor** | [1,10,100] | [2,3,4,5,6,7,8,9,20,30...] | 2.0 | Log subdivisions | ⚠️ Need to add |
| **Time Minor** | hourly | [15,30,45] minutes | 2.5 | Time subdivisions | ⚠️ Need to add |
| **No Minor** | [0,10,20] | [] | 0.0 | Major ticks only | ✅ Covered |

### Padding and Spacing
| Scenario | Tick Padding | Text Offset | Purpose | Status |
|----------|--------------|-------------|---------|--------|
| **Standard** | 3.0 | 3px from tick line | Default spacing | ✅ Covered |
| **Tight** | 1.0 | 1px from tick line | Compact layout | ⚠️ Need to add |
| **Wide** | 10.0 | 10px from tick line | Spacious layout | ⚠️ Need to add |
| **Zero** | 0.0 | Text on tick line | Overlapping text | ⚠️ Need to add |

## Style Options Coverage (JSON-Serializable)

### Grid Style Scenarios
| Property | Values to Test | Purpose | JSON Structure | Status |
|----------|----------------|---------|----------------|--------|
| **Color** | ["#f00", "currentColor", "rgba(0,0,0,0.1)"] | Various color formats | `{"color": "#f00"}` | ✅ Covered |
| **Width** | [0.5, 1.0, 2.0, 3.5] | Line thickness | `{"width": 2.0}` | ✅ Covered |
| **Dasharray** | [null, "2,2", "5,5", "10,5,2,5"] | Line patterns | `{"dasharray": "2,2"}` | ✅ Covered |
| **Opacity** | [0.0, 0.3, 0.7, 1.0] | Transparency levels | `{"opacity": 0.7}` | ⚠️ Need to add |

### Title Style Scenarios
| Property | Values to Test | Purpose | JSON Structure | Status |
|----------|----------------|---------|----------------|--------|
| **Font** | ["Arial", "serif", "16px Helvetica"] | Font specifications | `{"font": "Arial"}` | ✅ Covered |
| **Color** | ["#00f", "currentColor", "inherit"] | Title colors | `{"color": "#00f"}` | ✅ Covered |
| **Position** | [[5,5], [0,0], [-10,20]] | Title positioning | `{"position": [5.0, 5.0]}` | ✅ Covered |
| **Anchor** | ["start", "middle", "end"] | Text alignment | `{"anchor": "middle"}` | ⚠️ Need to add |
| **Angle** | [0, 90, -45, 180] | Text rotation | `{"angle": 90}` | ⚠️ Need to add |

### Tick Label Style Scenarios
| Property | Values to Test | Purpose | JSON Structure | Status |
|----------|----------------|---------|----------------|--------|
| **Font** | ["Verdana", "12px monospace"] | Label fonts | `{"font": "Verdana"}` | ✅ Covered |
| **Color** | ["#333", "inherit", "currentColor"] | Label colors | `{"color": "#333"}` | ✅ Covered |
| **Padding** | [0.0, 2.0, 5.0, 10.0] | Label spacing | `{"padding": 2.0}` | ✅ Covered |
| **Angle** | [0, 45, 90, -30] | Rotated labels | `{"angle": 45}` | ✅ Covered |
| **Anchor** | ["start", "middle", "end"] | Label alignment | `{"anchor": "end"}` | ⚠️ Need to add |

### Axis Line Style Scenarios
| Property | Values to Test | Purpose | JSON Structure | Status |
|----------|----------------|---------|----------------|--------|
| **Color** | ["#abc", "black", "currentColor"] | Domain line colors | `{"color": "#abc"}` | ✅ Covered |
| **Width** | [0.5, 1.0, 1.5, 3.0] | Domain line thickness | `{"width": 1.5}` | ✅ Covered |
| **Dasharray** | [null, "3,1", "5,5"] | Domain line patterns | `{"dasharray": null}` | ✅ Covered |

## Critical Gap Analysis-Based Scenarios

### Priority 1 - Critical Missing Behaviors
| Issue | D3 Behavior | Rust Current | Required Test | Status |
|-------|-------------|--------------|---------------|--------|
| **devicePixelRatio Offset** | `offset = devicePixelRatio > 1 ? 0 : 0.5` | Always 0.5 | High-DPI scenarios | ❌ **Missing** |
| **Domain Path vs Line** | SVG `<path>` with outer ticks | Simple `<line>` element | Path command tests | ❌ **Missing** |
| **Element Order** | Domain before ticks | Domain after ticks | DOM structure tests | ❌ **Missing** |
| **Selection Lifecycle** | enter/update/exit pattern | Direct manipulation | Animation/transition tests | ❌ **Missing** |

### Priority 2 - Important Behaviors
| Issue | D3 Behavior | Rust Current | Required Test | Status |
|-------|-------------|--------------|---------------|--------|
| **Time Offset Logic** | Always applies offset | Conditional application | Time renderer transform | ❌ **Missing** |
| **Transition Support** | Smooth animations | Immediate updates | Transition scenarios | ❌ **Missing** |
| **Element Structure** | `<g class="tick">` grouping | Flat structure | Proper nesting tests | ❌ **Missing** |

### Priority 3 - Consistency Issues
| Issue | D3 Behavior | Rust Current | Required Test | Status |
|-------|-------------|--------------|---------------|--------|
| **Color Consistency** | "currentColor" everywhere | Mixed hardcoded colors | Color attribute tests | ⚠️ **Partial** |
| **Transform Chaining** | Proper composition | Inconsistent patterns | Chain transform tests | ⚠️ **Partial** |

## Implementation Priority

### Immediate (Priority 1)
1. **High-DPI Offset Detection**: Add devicePixelRatio-aware offset scenarios
2. **Domain Path Commands**: Test SVG path generation with outer ticks vs simple lines  
3. **Element Ordering**: Verify domain-before-ticks DOM structure
4. **Transform String Generation**: Test all orientation + offset combinations

### Near-term (Priority 2)
5. **Minor Tick Coverage**: Add log and time scale minor tick scenarios
6. **Edge Case Domains**: Huge ranges, reversed domains, fractional starts
7. **Style Completeness**: Add missing opacity, anchor, angle properties
8. **Size Variations**: Add negative tick sizes, zero sizes, asymmetric sizing

### Long-term (Priority 3)
9. **Transition Support**: Add animation and lifecycle test scenarios
10. **Performance**: Large tick counts, many categories
11. **Locale Support**: Multiple time/number format locales
12. **Accessibility**: ARIA labels, semantic structure tests

## Test File Organization

### Suggested New Fixture Files
```
tests/fixtures/
├── axis_comprehensive.json          # ✅ Exists
├── axis_edge_cases.json             # ⚠️ Need to create  
├── axis_transforms.json             # ⚠️ Need to create
├── axis_styles_complete.json        # ⚠️ Need to create
├── axis_dpi_awareness.json          # ❌ Need to create
├── axis_dom_structure.json          # ❌ Need to create
└── axis_transitions.json            # ❌ Need to create (future)
```

### Test Coverage Goals
- **Scale Types**: 100% coverage of linear, log, time, band, point scenarios
- **Layout Variants**: All orientations × offset values × sizing combinations
- **Transform Strings**: Proper D3-compatible transform generation
- **Style Options**: Complete JSON-serializable style property coverage
- **Critical Gaps**: Address all Priority 1 D3 parity issues
- **Edge Cases**: Domain boundaries, extreme values, error conditions

This coverage matrix ensures every identified behavior gap has corresponding test scenarios to validate D3.js parity.
