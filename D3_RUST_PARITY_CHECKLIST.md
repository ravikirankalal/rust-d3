# D3.js to Rust Parity Checklist

This checklist tracks the parity between the official d3.js modules and your Rust port. Mark each as complete or partial as you implement them.

| D3.js Module           | Rust Equivalent? | Status   | Depends On                                    | Notes/Features Missing |
|------------------------|------------------|----------|-----------------------------------------------|------------------------|
| d3-array               | array/           | Complete |                                               | All core and advanced features ported and tested: min, max, extent, mean, median, sum, deviation, variance, quantile, histogram, bisect, ascending, descending, range, merge, shuffle, tick_step, ticks, nice, scan, group, flat_group, pairs, zip, cross, least, greatest, least_index, greatest_index, fsum, blur (blur1d, blur2d), set operations (union, intersection, difference, symmetric_difference), sort, sort_by, summarize, transform, intern_set, intern_map |
| d3-collection          | collection/      | Complete |                                               | All major and utility features ported and tested: nest, map, set, keys, values, entries, rollup, index, groups, flat_group, flat_rollup, from_entries, count, count_map, count_values, filter_map, map_map, map_keys, map_values, merge_maps, invert, find_key, find_value, map_filter, map_entries, partition_map, update_map, remove_keys, retain_keys, merge_with, map_to_vec. No major features missing. |
| d3-format              | format/          | Complete | array/, collection/                           | All core and advanced features ported and tested: format, FormatSpecifier, parse_specifier, format_decimal, format_integer, format_float, format_grouping, format_prefix, format_type. Full specifier parsing (fill, align, sign, symbol, width, zero, type variants), percent, exponential, binary, octal, hexadecimal, SI precision rounding, negative zero, NaN/Infinity handling, advanced grouping, padding, and edge-case parity. Locale and currency support: pending. |
| d3-time                | time/            | Pending  | array/, collection/, format/                  |                        |
| d3-time-format         | time_format/     | Pending  | array/, collection/, format/                  |                        |
| d3-scale               | scale/           | Pending  | array/, collection/, format/, time/, time_format/ |                        |
| d3-axis                | axis/            | Pending  | scale/                                        |                        |
| d3-shape               | shape/           | Pending  | array/, collection/, scale/                   |                        |
| d3-geo                 | geo/             | Pending  | array/, collection/, shape/, path/            |                        |
| d3-hierarchy           | hierarchy/       | Pending  | array/, collection/                           |                        |
| d3-interpolate         | interpolate/     | Pending  | array/, collection/                           |                        |
| d3-path                | path/            | Pending  | array/, collection/                           |                        |
| d3-polygon             | polygon/         | Pending  | array/, collection/                           |                        |
| d3-quadtree            | quadtree/        | Pending  | array/, collection/                           |                        |
| d3-random              | random/          | Pending  | array/, collection/                           |                        |
| d3-scale-chromatic     | scale_chromatic/ | Pending  | scale/                                        |                        |
| d3-selection           | selection/       | Pending  | array/, collection/                           |                        |
| d3-brush               | brush/           | Pending  | selection/, scale/, axis/                     |                        |
| d3-dsv                 | dsv/             | Pending  | array/, collection/                           |                        |
| d3-ease                | ease/            | Pending  | array/, collection/                           |                        |
| d3-fetch               | fetch/           | Pending  | array/, collection/                           |                        |
| d3-force               | force/           | Pending  | array/, collection/                           |                        |
| d3-dispatch            | dispatch/        | Pending  | array/, collection/                           |                        |
| d3-drag                | drag/            | Pending  | selection/, scale/, axis/                     |                        |
| d3-chord               | chord/           | Pending  | array/, collection/                           |                        |
| d3-contour             | contour/         | Pending  | array/, collection/                           |                        |
| d3-delaunay            | delaunay/        | Pending  | array/, collection/                           |                        |
| d3-timer               | timer/           | Pending  | array/, collection/                           |                        |
| d3-transition          | transition/      | Pending  | selection/, scale/, axis/                     |                        |
| d3-tree                | tree/            | Pending  | hierarchy/                                    |                        |
| d3-treemap             | treemap/         | Pending  | hierarchy/                                    |                        |
| d3-voronoi             | voronoi/         | Pending  | delaunay/                                     |                        |
| d3-zoom                | zoom/            | Pending  | selection/, scale/, axis/                     |                        |
| d3-request (legacy)    | fetch/           | Pending  | fetch/                                        |                        |
| d3-queue (legacy)      | queue/           | Pending  | array/, collection/                           |                        |
