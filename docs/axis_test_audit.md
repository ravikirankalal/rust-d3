# Axis Test Inventory Checklist

This temporary file documents the categorization of axis tests for deterministic deletion decisions.

## Tests in `tests/axis.rs`

### Fixture-powered Tests (use `load_axis_fixtures()` and `assert_tick_matches_expectation()`)
- [ ] `test_log_axis_ticks` (lines 20-34)
- [ ] `test_band_axis_ticks` (lines 58-80) 
- [ ] `test_point_axis_ticks` (lines 83-103)
- [ ] `test_linear_axis_custom_ticks` (lines 106-121)

### Legacy Tests (all others)
- [ ] `test_linear_axis_ticks` (lines 13-17) - marked as redundant, moved to axis_linear.rs
- [ ] `test_time_axis_ticks` (lines 37-55) - uses hardcoded values, no fixtures
- [ ] `test_axis_layout_linear` (lines 124-141) - layout test, no fixtures
- [ ] `test_fixture_based_ticks_linear_scale` (lines 144-160) - hardcoded fixture values
- [ ] `test_fixture_based_ticks_log_scale` (lines 163-174) - hardcoded fixture values  
- [ ] `test_fixture_based_ticks_time_scale` (lines 177-205) - hardcoded fixture values
- [ ] `test_axis_tick_size` (lines 208-219) - style test, no fixtures
- [ ] `test_axis_tick_size_chainable` (lines 222-240) - style test, no fixtures
- [ ] `test_axis_layout_with_offset_and_locale` (lines 243-257) - layout test, no fixtures
- [ ] `test_axis_grid_and_style` (lines 260-271) - style test, no fixtures
- [ ] `test_axis_title_and_style` (lines 274-285) - style test, no fixtures
- [ ] `test_axis_minor_ticks_and_size` (lines 288-295) - style test, no fixtures
- [ ] `test_axis_tick_label_angle_and_style` (lines 298-309) - style test, no fixtures
- [ ] `test_axis_line_style` (lines 312-321) - style test, no fixtures
- [ ] `test_axis_on_render_hook` (lines 324-337) - hook test, no fixtures
- [ ] `test_axis_empty_domain_range` (lines 340-345) - edge case test, no fixtures
- [ ] `test_axis_single_tick` (lines 348-353) - edge case test, no fixtures
- [ ] `test_axis_default_offset` (lines 356-366) - offset test, no fixtures
- [ ] `test_axis_custom_offset` (lines 369-379) - offset test, no fixtures
- [ ] `test_axis_zero_offset` (lines 382-392) - offset test, no fixtures
- [ ] `test_axis_offset_chaining` (lines 395-406) - offset test, no fixtures
- [ ] `test_axis_offset_in_layout` (lines 409-421) - offset test, no fixtures
- [ ] `test_band_axis_default_offset` (lines 424-430) - offset test, no fixtures
- [ ] `test_point_axis_default_offset` (lines 433-439) - offset test, no fixtures
- [ ] `test_log_axis_default_offset` (lines 442-448) - offset test, no fixtures
- [ ] `test_time_axis_default_offset` (lines 451-465) - offset test, no fixtures
- [ ] `test_axis_transform_application` (lines 468-505) - transform test, no fixtures
- [ ] `test_axis_tick_size_alias` (lines 508-523) - style test, no fixtures
- [ ] `test_axis_offset_half_visual_diff` (lines 526-545) - offset test, no fixtures
- [ ] `test_axis_custom_tick_format_string` (lines 548-568) - format test, no fixtures
- [ ] `test_axis_custom_offset_transform` (lines 571-593) - transform test, no fixtures
- [ ] `test_time_axis_seconds_interval` (lines 598-617) - time test, no fixtures
- [ ] `test_time_axis_minutes_interval` (lines 620-639) - time test, no fixtures
- [ ] `test_time_axis_hours_interval` (lines 642-661) - time test, no fixtures
- [ ] `test_time_axis_days_interval` (lines 664-683) - time test, no fixtures
- [ ] `test_time_axis_weeks_interval` (lines 686-706) - time test, no fixtures
- [ ] `test_time_axis_months_interval` (lines 709-729) - time test, no fixtures
- [ ] `test_time_axis_years_interval` (lines 732-752) - time test, no fixtures
- [ ] `test_time_axis_reverse_domain` (lines 755-776) - time test, no fixtures
- [ ] `test_time_axis_custom_tick_values` (lines 779-808) - time test, no fixtures
- [ ] `test_time_axis_custom_format` (lines 811-843) - time format test, no fixtures
- [ ] `test_linear_axis_negative_domain` (lines 848-860) - edge case test, no fixtures
- [ ] `test_linear_axis_fractional_domain` (lines 863-875) - edge case test, no fixtures
- [ ] `test_linear_axis_large_numbers` (lines 878-890) - edge case test, no fixtures
- [ ] `test_linear_axis_very_small_numbers` (lines 893-905) - edge case test, no fixtures
- [ ] `test_linear_axis_reverse_range` (lines 908-921) - edge case test, no fixtures
- [ ] `test_log_axis_multiple_decades` (lines 923-935) - log test, no fixtures
- [ ] `test_log_axis_base_2` (lines 938-950) - log test, no fixtures
- [ ] `test_band_axis_with_padding` (lines 953-972) - band test, no fixtures
- [ ] `test_point_axis_with_padding` (lines 975-992) - point test, no fixtures
- [ ] `test_axis_with_large_tick_count` (lines 995-1007) - edge case test, no fixtures
- [ ] `test_axis_with_minimal_tick_count` (lines 1010-1022) - edge case test, no fixtures
- [ ] `test_axis_domain_bounds_inclusion` (lines 1025-1039) - bounds test, no fixtures
- [ ] `test_axis_domain_bounds_only_for_auto_generated_ticks` (lines 1042-1063) - bounds test, no fixtures
- [ ] `test_axis_positioning_accuracy` (lines 1066-1084) - positioning test, no fixtures
- [ ] `test_d3_parity_linear_ticks_0_10_count_5` (lines 1091-1107) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_linear_ticks_minus5_5_count_4` (lines 1110-1127) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_linear_axis_orientations` (lines 1130-1165) - orientation test, hardcoded fixtures
- [ ] `test_d3_parity_log_scale_1_1000` (lines 1168-1190) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_time_scale_days_formatting` (lines 1193-1216) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_linear_scale_custom_ticks_positions` (lines 1219-1235) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_log_scale_custom_ticks_positions` (lines 1238-1254) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_time_scale_hours_formatting` (lines 1257-1278) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_band_scale_positioning` (lines 1281-1298) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_point_scale_positioning` (lines 1301-1317) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_linear_tick_label_formatting` (lines 1320-1339) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_log_tick_label_formatting` (lines 1342-1361) - D3 parity test, hardcoded fixtures
- [ ] `test_d3_parity_axis_domain_path_bounds` (lines 1364-1377) - D3 parity test, hardcoded fixtures

## Tests in `tests/axis_linear.rs`

### Fixture-powered Tests (use `load_axis_fixtures()` and `assert_tick_matches_expectation()`)
- [ ] `test_linear_axis_basic_ticks` (lines 8-22)
- [ ] `test_linear_axis_custom_ticks` (lines 25-40)
- [ ] `test_linear_axis_negative_domain` (lines 43-57)
- [ ] `test_linear_axis_fractional_domain` (lines 60-74)
- [ ] `test_linear_axis_large_numbers` (lines 77-91)
- [ ] `test_linear_axis_small_numbers` (lines 94-108)
- [ ] `test_linear_axis_reverse_range` (lines 111-125)
- [ ] `test_linear_axis_empty_domain` (lines 128-142)
- [ ] `test_linear_axis_single_value` (lines 145-160)

## Summary
- **Total Fixture-powered tests**: 13 tests (4 in axis.rs + 9 in axis_linear.rs)
- **Total Legacy tests**: 61 tests (all in axis.rs)

## Notes
- `test_linear_axis_ticks` in axis.rs is marked as redundant and moved to axis_linear.rs
- Many tests in axis.rs use hardcoded fixture values but don't use the fixture helper functions
- All tests in axis_linear.rs properly use the fixture loading and assertion helpers
