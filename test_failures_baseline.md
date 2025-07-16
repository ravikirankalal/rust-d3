# Test Failures Baseline - cargo test --test axis

## Summary
Total tests: 55
Passed: 46
Failed: 9

## Failing Tests Analysis

### 1. test_axis_domain_bounds_inclusion
- **File**: tests/axis.rs
- **Line**: 922
- **Error**: `assertion failed: ticks[0].value <= 2.3 + 0.1`
- **Issue**: Domain bounds inclusion logic - first tick value exceeds expected upper bound
- **Affected Component**: Axis tick generation logic

### 2. test_axis_custom_offset_transform
- **File**: tests/axis.rs
- **Line**: 493
- **Error**: `assertion `left == right` failed: left: None, right: Some("translate(0,0)")`
- **Issue**: Custom offset transform not being applied correctly
- **Affected Component**: Axis transform/offset handling
- **Debug Info**: Shows offset calculations but transform not set properly

### 3. test_linear_axis_large_numbers
- **File**: tests/axis.rs
- **Line**: 778
- **Error**: `assertion failed: ticks.iter().any(|t| t.label.len() > 4)`
- **Issue**: Large number formatting - labels should be longer than 4 characters but aren't
- **Affected Component**: Linear axis tick label formatting

### 4. test_log_axis_base_2
- **File**: tests/axis.rs
- **Line**: 835
- **Error**: `assertion failed: ticks.iter().any(|t| t.value == 2.0)`
- **Issue**: Log axis with base 2 not generating expected tick at value 2.0
- **Affected Component**: Log axis tick generation for non-base-10

### 5. test_log_axis_multiple_decades
- **File**: tests/axis.rs
- **Line**: 820
- **Error**: `assertion failed: ticks.iter().any(|t| t.value == 10.0)`
- **Issue**: Log axis spanning multiple decades not generating tick at value 10.0
- **Affected Component**: Log axis tick generation for multi-decade ranges

### 6. test_point_axis_with_padding
- **File**: tests/axis.rs
- **Line**: 879
- **Error**: `assertion failed: ticks[0].position > 0.0`
- **Issue**: Point axis with padding - first tick position should be > 0.0 but isn't
- **Affected Component**: Point axis positioning with padding

### 7. test_time_axis_minutes_interval
- **File**: tests/axis.rs
- **Line**: 536
- **Error**: `assertion failed: ticks[0].label.contains("12:00")`
- **Issue**: Time axis minute interval - first tick label doesn't contain expected time "12:00"
- **Affected Component**: Time axis tick labeling for minute intervals
- **Debug Info**: Shows extensive tick interval selection logic, chose Minute(5) interval

### 8. test_time_axis_seconds_interval
- **File**: tests/axis.rs
- **Line**: 515
- **Error**: `assertion failed: ticks[0].label.contains("12:00:00")`
- **Issue**: Time axis second interval - first tick label doesn't contain expected time "12:00:00"
- **Affected Component**: Time axis tick labeling for second intervals
- **Debug Info**: Shows tick interval selection, chose Second(5) interval

### 9. test_time_axis_hours_interval
- **File**: tests/axis.rs
- **Line**: 556
- **Error**: `assertion `left == right` failed: left: 5, right: 7`
- **Issue**: Time axis hour interval - expected 7 ticks but got 5
- **Affected Component**: Time axis tick count for hour intervals
- **Debug Info**: Shows tick interval selection, chose Hour(3) interval

## Categories of Issues

### Axis Transform/Offset (1 failure)
- test_axis_custom_offset_transform

### Domain/Bounds Logic (1 failure)
- test_axis_domain_bounds_inclusion

### Linear Axis (1 failure)
- test_linear_axis_large_numbers

### Log Axis (2 failures)
- test_log_axis_base_2
- test_log_axis_multiple_decades

### Point Axis (1 failure)
- test_point_axis_with_padding

### Time Axis (3 failures)
- test_time_axis_minutes_interval
- test_time_axis_seconds_interval
- test_time_axis_hours_interval

## Key Files That Need Attention

1. **Axis implementation** - Core axis logic for transforms, offsets, and bounds
2. **Linear scale implementation** - Number formatting for large values
3. **Log scale implementation** - Tick generation for non-base-10 and multi-decade ranges
4. **Point scale implementation** - Positioning with padding
5. **Time scale implementation** - Tick labeling and count generation for time intervals

## Verification Strategy
After each fix, run `cargo test --test axis` to verify the specific test passes and ensure no regressions in other tests.
