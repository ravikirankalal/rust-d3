# D3.js to Rust Parity Checklist

**Porting Progress:**
- Complete: 8/29 modules (28%)
- In Progress: 1/29 modules (3%)
- Pending: 20/29 modules (69%)

This checklist tracks the parity between the official d3.js modules and your Rust port. Mark each as complete or partial as you implement them.

| D3.js Module           | Rust Equivalent? | Status   | % Complete | Depends On                                    | Notes/Features Missing |
|------------------------|------------------|----------|------------|-----------------------------------------------|------------------------|
| d3-array               | array/           | Complete | 100%       |                                               | All core and advanced features ported and tested: min, max, extent, mean, median, sum, deviation, variance, quantile, histogram, bisect, ascending, descending, range, merge, shuffle, tick_step, ticks, nice, scan, group, flat_group, pairs, zip, cross, least, greatest, least_index, greatest_index, fsum, blur (blur1d, blur2d), set operations (union, intersection, difference, symmetric_difference), sort, sort_by, summarize, transform, intern_set, intern_map |
| d3-collection          | collection/      | Complete | 100%       |                                               | All major and utility features ported and tested: nest, map, set, keys, values, entries, rollup, index, groups, flat_group, flat_rollup, from_entries, count, count_map, count_values, filter_map, map_map, map_keys, map_values, merge_maps, invert, find_key, find_value, map_filter, map_entries, partition_map, update_map, remove_keys, retain_keys, merge_with, map_to_vec. No major features missing. |
| d3-format              | format/          | Complete | 100%       | array/, collection/                           | All core and advanced features ported and tested: format, FormatSpecifier, parse_specifier, format_decimal, format_integer, format_float, format_grouping, format_prefix, format_type. Full specifier parsing (fill, align, sign, symbol, width, zero, type variants), percent, exponential, binary, octal, hexadecimal, SI precision rounding, negative zero, NaN/Infinity handling, advanced grouping, padding, and edge-case parity. Locale and currency support: stubbed, but axis uses locale for tick formatting. |
| d3-time                | time/            | Complete | 100%       | array/, collection/, format/                  | All standard intervals (Second, Minute, Hour, Day, Week, Month, Year) ported and tested. Methods: floor, ceil, offset, range. Week starts on Sunday. UTC variants, week numbering, and edge-case tests can be added for full parity. |
| d3-time-format         | time/format.rs   | Complete | 100%       | array/, collection/, format/                  | All D3 time format specifiers supported, including composite formats. Locale struct present. Parsing (time_parse) and full locale-aware formatting are stubs; formatting parity is complete. |
| d3-scale               | scale/           | Complete | 100%       | array/, collection/, format/, time/, time_format/ | All major scale types (linear, log, pow, sqrt, symlog, time, band, point) ported and tested. D3-like features: symlog transform, band/point padding/align, bandwidth, invert, and edge-case handling. All tests pass. |
| d3-axis                | axis/            | Complete | 100%       | scale/                                        | All D3 axis features: orientation, tick generation (auto/custom), tick formatting (auto/custom/locale-aware), tick arguments, tick values, tick size (inner/outer), tick padding, offset, layout struct for rendering, builder API. Locale-aware tick formatting and offset supported. SVG rendering helpers not included. |
| d3-shape               | shape/           | In Progress | 80%        | array/, collection/, scale/                   | All generators (line, area, arc, pie, stack, symbol) now support robust, type-safe custom output traits and NaN/None handling. Advanced curve types (basis, cardinal, monotone, etc.) are present but need full D3-like interpolation for pixel-perfect parity. Some edge-case and output tests are being updated for new output traits. |
| d3-geo                 | geo/             | Pending  | 0%         | array/, collection/, shape/, path/            |                        |
| d3-hierarchy           | hierarchy/       | Complete | 100%       | array/, collection/                           | Node, TreeLayout, ClusterLayout, TreemapLayout, PartitionLayout, traversal, sum, parent pointers, x/y layout, integration with shape, full tests and documentation. |
| d3-interpolate         | interpolate/     | Pending  | 0%         | array/, collection/                           |                        |
| d3-path                | path/            | Pending  | 0%         | array/, collection/                           |                        |
| d3-polygon             | polygon/         | Pending  | 0%         | array/, collection/                           |                        |
| d3-quadtree            | quadtree/        | Pending  | 0%         | array/, collection/                           |                        |
| d3-random              | random/          | Pending  | 0%         | array/, collection/                           |                        |
| d3-scale-chromatic     | scale_chromatic/ | Pending  | 0%         | scale/                                        |                        |
| d3-selection           | selection/       | Pending  | 0%         | array/, collection/                           |                        |
| d3-brush               | brush/           | Pending  | 0%         | selection/, scale/, axis/                     |                        |
| d3-dsv                 | dsv/             | Pending  | 0%         | array/, collection/                           |                        |
| d3-ease                | ease/            | Pending  | 0%         | array/, collection/                           |                        |
| d3-fetch               | fetch/           | Pending  | 0%         | array/, collection/                           |                        |
| d3-force               | force/           | Pending  | 0%         | array/, collection/                           |                        |
| d3-dispatch            | dispatch/        | Pending  | 0%         | array/, collection/                           |                        |
| d3-drag                | drag/            | Pending  | 0%         | selection/, scale/, axis/                     |                        |
| d3-chord               | chord/           | Pending  | 0%         | array/, collection/                           |                        |
| d3-contour             | contour/         | Pending  | 0%         | array/, collection/                           |                        |
| d3-delaunay            | delaunay/        | Pending  | 0%         | array/, collection/                           |                        |
| d3-timer               | timer/           | Pending  | 0%         | array/, collection/                           |                        |
| d3-transition          | transition/      | Pending  | 0%         | selection/, scale/, axis/                     |                        |
| d3-tree                | tree/            | Pending  | 0%         | hierarchy/                                    |                        |
| d3-treemap             | treemap/         | Pending  | 0%         | hierarchy/                                    |                        |
| d3-voronoi             | voronoi/         | Pending  | 0%         | delaunay/                                     |                        |
| d3-zoom                | zoom/            | Pending  | 0%         | selection/, scale/, axis/                     |                        |
| d3-request (legacy)    | fetch/           | Pending  | 0%         | fetch/                                        |                        |
| d3-queue (legacy)      | queue/           | Pending  | 0%         | array/, collection/                           |                        |
