# D3 Axis Reference Fixtures

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
