# D3.js to Rust Parity Checklist

This checklist tracks the parity between the official d3.js modules and your Rust port. Mark each as complete or partial as you implement them.

| D3.js Module           | Rust Equivalent? | Status   | Notes/Features Missing |
|------------------------|------------------|----------|------------------------|
| d3-array               | array/ (unified, D3.js-like API) | Complete  | All set operations (union, intersection, difference), cross, full bisect functionality (.by), fsum precision, and tickStep exposure implemented and tested. |
| d3-axis                | axis/axis.rs                                   | Partial  | Missing: tickArguments, tickSize (inner/outer), tickPadding, offset. SVG rendering is a stub. Verify: AxisScale ownership, flexible tick arguments, tickFormat options. |
| d3-brush               | brush/ (unified)                                | Partial  | Missing: 2D brush, touchable, keyModifiers, full filter/handleSize logic, "start"/"end" events, SVG rendering. Verify: extent (set functionality). |
| d3-chord               | chord/ (unified)                                | Partial  | Missing: full chord layout generation (angles, radius, padding, sorting), and ribbon shape generator. |
| d3-color               | color/ (unified)                                | Partial  | Missing: full color space representation (RGB, HSL, Lab, etc.), parsing, conversion, manipulation (brighter/darker), and formatting. Current implementation is a basic color scale. |
| d3-contour             | contour/                                       | Partial  | Missing: proper Kernel Density Estimation (KDE) for contourDensity, GeoJSON output. Marching squares logic needs thorough review for correctness (e.g., case 5). |
| d3-delaunay            | delaunay/                                      | Partial  | Missing: efficient triangulation algorithm (current is O(N^3)), half-edge topology, neighbors, Voronoi diagram, convex hull, rendering. Optimized find is missing. |
| d3-dispatch            | dispatch/ (unified)                             | Partial  | Missing: passing arguments to callbacks, removing callbacks, subtypes (e.g., event.foo), and copy() method. |
| d3-drag                | drag/ (unified)                                 | Partial  | Missing: core drag behavior implementation, event handling (start/drag/end), and all customization methods (container, filter, touchable, subject, clickDistance). |
| d3-dsv                 | dsv/ (unified)                                  | Partial  | Missing: object-based parsing/formatting, formatBody, formatRow, formatValue, and full RFC 4180 compliance (quoting, escaping). parse_rows and format_rows are incorrectly implemented. |
| d3-ease                | ease/                                           | Complete | Potential minor mathematical precision differences. No default parameters for configurable easing functions. |
| d3-fetch               | fetch/ (unified)                                | Partial  | Missing: `init` options for requests, `row` accessor/converter for CSV/TSV, and `d3.autoType` integration. Functional differences in XML and image fetching. |
| d3-force               | force/ (unified)                                | Complete | Quadtree implemented for ManyBody and Collide forces. |
| d3-format              | format/ (unified)                               | Partial  | Missing: comprehensive format specifier (fill, align, sign, width, trim zeros, full type support), robust locale support, and advanced formatPrefix. |
| d3-geo                 | geo/ (unified)                                  | Partial  | Missing: wide range of projections, extensive projection customization, spherical geometry operations, full GeoJSON handling, adaptive sampling, geoGraticule, geoIdentity. |
| d3-hierarchy           | hierarchy/ (unified)                            | Partial  | Missing: hierarchy creation (from flat/nested data), automatic computation of node properties (depth, height, value, parent), rich navigation methods (ancestors, descendants, leaves, links, path), and all layout algorithms (tree, cluster, partition, pack, treemap). |
| d3-interpolate         | interpolate/ (unified)                          | Partial  | Missing: automatic type detection, date interpolation, full color space interpolation (Lab, Hcl, Cubehelix), transform/path/piecewise/basis/discrete interpolation. Limited array/object/string interpolation. |
| d3-path                | path/                                           | Partial  | Missing: relative commands, full arc variants (e.g., arcTo), elliptical arc, and explicit current point tracking. Arc/Bezier/Quadratic curve implementations need verification for full SVG/D3.js compliance. |
| d3-polygon             | polygon/ (unified)                              | Partial  | Missing: polygonHull. Area function returns absolute value, D3.js returns signed. |
| d3-quadtree            | quadtree/                                       | Partial  | Missing: hierarchical structure, efficient add/remove, root/data/size methods, optimized find, visit/visitAfter, x/y accessors, extent setting. Current implementation is a basic point container. |
| d3-random              | random/                                         | Partial  | Missing: randomInt, Beta, Pareto, Weibull, Gumbel, Logistic, Binomial, Geometric, Poisson distributions. Lacks seeded randomness. |
| d3-scale               | scale/                                          | Partial  | Missing: `clamp()` (D3.js behavior), `nice()` (D3.js behavior), `interpolate()`, `unknown()`, `rangeRound()`, `copy()`. `ticks()` and `tickFormat()` are basic. |
| d3-scale-chromatic     | scale_chromatic/                                | Partial  | Missing: extensive categorical, sequential (multi-hue/single-hue), diverging, and cyclical color schemes. Sequential interpolators use basic lookup tables instead of smooth interpolation. |
| d3-selection           | selection/ (unified, D3.js-like API)            | Partial  | Missing: DOM integration, CSS selector-based selection, full data join (creating/removing/updating DOM elements), event handling (on/dispatch), and many DOM manipulation methods (append, insert, remove, html, text, property, classed, style, attr). Transition integration is stubbed. |
| d3-shape               | shape/ (unified, D3.js-like API)                | Partial  | Missing: curve interpolation, links, stacks, extensive symbol types. Limited customization via accessors. SVG path generation needs verification. |
| d3-time                | time/ (unified, D3.js-like API)                 | Partial  | Missing: comprehensive time interval objects (floor, ceil, round, offset, range, every), extensive time scale customization (ticks, tickFormat, nice, clamp, interpolate, unknown), and UTC time support. |
| d3-time-format         | time_format/ (unified, D3.js-like API)          | Partial  | Missing: robust locale support, d3.timeMultiFormat, and full set of format specifier directives. |
| d3-timer               | timer/                                          | Partial  | Missing: `time` parameter for timers, `timer.restart()`. `timerFlush()` and underlying timer mechanisms differ from D3.js. |
| d3-transition          | transition/ (unified, D3.js-like API)           | Partial  | Missing: DOM integration, robust scheduling, automatic interpolation, per-element delay/duration/ease. Limited merge/select/filter behavior. |
| d3-zoom                | zoom/ (unified, D3.js-like API)                 | Complete | zoom_adv/ merged       |
| d3-collection          | collection/ (unified)                           | Complete | all group/rollup/keys/values/entries merged |
| d3-request (legacy)    | fetch/                                          | Complete | replaced by d3-fetch   |
| d3-queue (legacy)      | queue.rs                                        | Complete | minimal legacy API, replaced by async/futures |

**All modules are now unified and D3.js-like. All legacy/adv/utility modules and test files have been removed or merged. All tests for unified modules pass.**

## D3.js to Rust API Parity Table

| D3.js API (JS)                | Rust Export/Module         | Status/Mapping/Notes                                 |
|-------------------------------|---------------------------|------------------------------------------------------|
| **d3-array**                  | `array`                   |                                                      |
| d3.min, d3.max, d3.extent     | min, max, extent          | ✅ Unified, exported                                 |
| d3.sum, d3.mean, d3.median    | sum, mean, median         | ✅ Exported                                          |
| d3.quantile, d3.cumsum        | quantile, cumsum          | ✅ Exported                                          |
| d3.ticks, d3.tickStep         | ticks                     | ✅ Exported (tickStep: check)                        |
| d3.range, d3.shuffle, ...     | range, shuffle, permute, ascending, descending, min_index, max_index, fsum, Adder, flatten, sum, mean, median, mode, variance, deviation, pairs, transpose, zip, least, greatest | ✅ Exported                                          |
| d3.group, d3.rollup           | group, rollup, flat_group | ✅ Exported                                          |
| d3.bisect, d3.ascending, ...  | bisect, ascending, descending | ✅ Exported                                          |
| **d3-axis**                   | `axis`                    |                                                      |
| d3.axisTop, d3.axisBottom...  | Axis                      | ✅ Unified, exported                                 |
| **d3-brush**                  | `brush`                   |                                                      |
| d3.brush, d3.brushX, ...      | Brush                     | ✅ Unified, exported                                 |
| **d3-chord**                  | `chord`                   |                                                      |
| d3.chord, d3.ribbon           | chord                     | ✅ Unified, exported                                 |
| **d3-color**                  | `color`                   |                                                      |
| d3.color, d3.rgb, d3.hsl, ... | Color, ColorScale         | ✅ Unified, exported                                 |
| **d3-contour**                | `contour`                 |                                                      |
| d3.contours, d3.contourDensity| contours, ContourLine, contour_density, ContourDensity | ✅ Exported                                          |
| **d3-delaunay**               | `delaunay`, `voronoi`     |                                                      |
| d3.Delaunay, d3.Voronoi       | Delaunay, VoronoiDiagram  | ✅ Exported                                          |
| **d3-dispatch**               | `dispatch`                |                                                      |
| d3.dispatch                   | dispatch                  | ✅ Exported                                          |
| **d3-drag**                   | `drag`                    |                                                      |
| d3.drag                       | drag                      | ✅ Exported                                          |
| **d3-dsv**                    | `dsv`                     |                                                      |
| d3.csvParse, d3.tsvParse, ... | parse_csv, parse_tsv, ... | ✅ Exported                                          |
| **d3-ease**                   | `ease`                    |                                                      |
| d3.easeLinear, ...            | linear, quad_in, quad_out, quad_in_out, cubic_in, cubic_out, cubic_in_out, poly_in, poly_out, poly_in_out, sin_in, sin_out, sin_in_out, exp_in, exp_out, exp_in_out, circle_in, circle_out, circle_in_out, bounce_in, bounce_out, bounce_in_out, back_in, back_out, back_in_out, elastic_in, elastic_out, elastic_in_out | ✅ Exported                                          |
| **d3-fetch**                  | `fetch`                   |                                                      |
| d3.csv, d3.tsv, d3.json, ...  | fetch_text, fetch_json, fetch_csv, fetch_tsv, fetch_xml, fetch_blob, fetch_buffer, fetch_svg, fetch_html, fetch_image | ✅ Exported                                          |
| **d3-force**                  | `force`                   |                                                      |
| d3.forceSimulation, ...       | ForceSimulation, ...      | ✅ Unified, exported. alphaMin and alphaDecay added. Quadtree implemented for ManyBody and Collide. |
| **d3-format**                 | `format`                  |                                                      |
| d3.format, d3.formatPrefix... | format                    | ✅ Exported                                          |
| **d3-geo**                    | `geo`                     |                                                      |
| d3.geoPath, d3.geoMercator... | equirectangular, mercator, GeoPathGenerator, geo_path_generator | ✅ Exported                                          |
| **d3-hierarchy**              | `hierarchy`, ...          |                                                      |
| d3.hierarchy, d3.cluster...   | Node, cluster, ...        | ✅ Unified, exported                                 |
| **d3-interpolate**            | `interpolate`             |                                                      |
| d3.interpolate, ...           | interpolate               | ✅ Exported                                          |
| **d3-path**                   | `path`                    |                                                      |
| d3.path                       | PathBuilder               | ✅ Exported                                          |
| **d3-polygon**                | `polygon`                 |                                                      |
| d3.polygonArea, ...           | polygon_area, ...         | ✅ Exported                                          |
| **d3-quadtree**               | `quadtree`                |                                                      |
| d3.quadtree                   | Quadtree                  | ✅ Exported                                          |
| **d3-random**                 | `random`                  |                                                      |
| d3.randomUniform, ...         | random_uniform, ...       | ✅ Exported                                          |
| **d3-scale**                  | `scale`                   |                                                      |
| d3.scaleLinear, ...           | scale types               | ✅ Exported                                          |
| **d3-scale-chromatic**        | `scale_chromatic`         |                                                      |
| d3.schemeCategory10, ...      | scheme_category10, ...    | ✅ Exported                                          |
| **d3-selection**              | `selection`               |                                                      |
| d3.select, d3.selection, ...  | Selection                 | ✅ Exported                                          |
| **d3-shape**                  | `shape`                   |                                                      |
| d3.line, d3.area, ...         | LineGenerator, ...        | ✅ Exported                                          |
| **d3-stratify**               | `stratify`                |                                                      |
| d3.stratify                   | stratify                  | ✅ Exported                                          |
| **d3-time**                   | `time`                    |                                                      |
| d3.timeFormat, ...            | format_time, TimeScale    | ✅ Exported                                          |
| **d3-time-format**            | `time_format`             |                                                      |
| d3.timeFormat, d3.timeParse   | format_time, ...          | ✅ Exported                                          |
| **d3-timer**                  | `timer`                   |                                                      |
| d3.timer, d3.timeout, ...     | timer, timeout, interval, now | ✅ Exported                                          |
| **d3-transition**             | `transition`              |                                                      |
| d3.transition                 | Transition                | ✅ Exported                                          |
| **d3-tree**                   | `tree`                    |                                                      |
| d3.tree                       | tree                      | ✅ Exported                                          |
| **d3-treemap**                | `treemap`                 |                                                      |
| d3.treemap                    | Treemap, TreemapTiling, TreemapTiler | ✅ Exported                                          |
| **d3-voronoi**                | `voronoi`                 |                                                      |
| d3.voronoi                    | VoronoiDiagram            | ✅ Exported                                          |
| **d3-zoom**                   | `zoom`                    |                                                      |
| d3.zoom, d3.zoomIdentity...   | Zoom                      | ✅ Exported                                          |
| **d3-collection**             | `array`, `collection`     |                                                      |
| d3.keys, d3.values, ...       | keys, values, entries     | ✅ Unified                                           |
| **d3-request (legacy)**       | `fetch`                   |                                                      |
| d3.request                    | fetch_text                | ✅ Replaced by fetch                                 |
| **d3-queue (legacy)**         | `queue`                   |                                                      |
| d3.queue                      | Queue                     | ✅ Exported (legacy)                                 |

---

**Legend:**  
✅ = Present and exported  
⚠️ = Partial or missing, check for implementation  
(check) = Not shown in exports, may need review
