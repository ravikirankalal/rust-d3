# Testing Documentation

This directory contains the test suite for the rust-d3 project, which provides D3.js-compatible data visualization components in Rust.

## Test Organization

The test suite is organized into several categories:

- **Unit Tests**: Individual component testing (array, color, dispatch, etc.)
- **Integration Tests**: Cross-component functionality testing 
- **Fixture-Based Tests**: Reference implementation validation using D3.js fixtures
- **Golden Tests**: SVG output validation against known good outputs

## Axis Testing Policy

**IMPORTANT**: All axis tests MUST be powered by D3.js fixtures to ensure complete compatibility with the D3.js reference implementation.

### Why D3.js Fixtures Are Required

The axis module is one of the most complex and behavior-rich components in D3.js, with numerous edge cases, scaling scenarios, and rendering behaviors. To ensure our Rust implementation maintains perfect parity with D3.js:

1. **Reference Accuracy**: D3.js fixtures provide the authoritative source of truth for expected behavior
2. **Edge Case Coverage**: D3.js naturally handles many edge cases that might be missed in manual test creation
3. **Behavioral Consistency**: Complex interactions between scales, ticks, formatting, and rendering are preserved
4. **Future Compatibility**: Changes in D3.js behavior can be detected through fixture regeneration

### Fixture Generation

All axis test fixtures are generated using the script in `tests/fixtures/generate_reference.js`. This script:

- Uses the official D3.js axis and scale implementations
- Generates comprehensive test data covering all scale types (linear, time, log, band, point)
- Includes edge cases, styling options, and complex configurations
- Outputs JSON fixtures that can be consumed by Rust tests

To regenerate fixtures:

```bash
cd tests/fixtures
npm install
node generate_reference.js
```

### Test Implementation

Axis tests should:

1. Load the appropriate D3.js fixture data
2. Configure the Rust axis implementation with the same parameters
3. Compare outputs (tick values, positions, labels, SVG structure)
4. Validate against the D3.js reference behavior

See `tests/fixtures/README.md` for detailed information about:
- Available fixture data
- Coverage matrix of tested scenarios
- Implementation priorities for missing behaviors

## Running Tests

### All Tests
```bash
cargo test
```

### Axis Tests Only
```bash
cargo test axis
```

### Specific Test Modules
```bash
cargo test color
cargo test selection
```

### Documentation Tests
```bash
cargo test --doc
```

## CI Integration

The continuous integration pipeline automatically:
- Runs all tests using `cargo test --all` (auto-discovery)
- Validates formatting and linting
- Tests across multiple platforms (Linux, Windows, macOS)
- Generates example artifacts for visual validation

No specific test files need to be configured in CI - Rust's test runner automatically discovers and executes all `#[test]` functions.

## Test Guidelines

1. **Axis Tests**: Must use D3.js fixtures (see policy above)
2. **Unit Tests**: Should test individual functions and edge cases
3. **Integration Tests**: Should validate cross-component interactions
4. **Golden Tests**: Should generate SVG outputs for visual regression testing
5. **Performance Tests**: Should be marked with `#[ignore]` for optional execution

## Coverage

The test suite aims for comprehensive coverage of:
- All D3.js scale types and configurations
- Edge cases and error conditions
- SVG output validation
- Cross-browser/platform compatibility
- Performance characteristics

See the detailed coverage matrix in `tests/fixtures/README.md` for axis-specific testing status.
