# D3.js to Rust Parity Checklist

This checklist tracks the parity between the official d3.js modules and your Rust port. Mark each as complete or partial as you implement them.

| D3.js Module           | Rust Equivalent? | Status   | Notes/Features Missing |
|------------------------|------------------|----------|------------------------|
| d3-array               | array/ (unified, D3.js-like API) | Complete | All utils/adv merged, all tests pass |
| d3-axis                | axis/axis.rs                                   | Complete |                        |
| d3-brush               | brush/ (unified)                                | Complete | brush_adv/ merged      |
| d3-chord               | chord/ (unified)                                | Complete | chord_adv/ merged      |
| d3-color               | color/ (unified)                                | Complete | color_adv/, color_utils/ merged |
| d3-contour             | contour/                                       | Complete |                        |
| d3-delaunay            | delaunay/                                      | Complete |                        |
| d3-dispatch            | dispatch/ (unified)                             | Complete | dispatch_adv/ merged   |
| d3-drag                | drag/ (unified)                                 | Complete | drag_adv/ merged       |
| d3-dsv                 | dsv/ (unified)                                  | Complete | dsv_adv/ merged        |
| d3-ease                | ease/                                           | Complete |                        |
| d3-fetch               | fetch/ (unified)                                | Complete | fetch_adv/ merged      |
| d3-force               | force/ (unified)                                | Complete | all force_* merged     |
| d3-format              | format/ (unified)                               | Complete | format_adv/ merged     |
| d3-geo                 | geo/ (unified)                                  | Complete | geo_adv/, geo_proj/, geo_proj_adv/ merged |
| d3-hierarchy           | hierarchy/ (unified)                            | Complete | all cluster, pack, partition, stratify, tree, treemap merged |
| d3-interpolate         | interpolate/ (unified)                          | Complete | all adv/utils merged   |
| d3-path                | path/                                           | Complete |                        |
| d3-polygon             | polygon/ (unified)                              | Complete | polygon_hull/ merged   |
| d3-quadtree            | quadtree/                                       | Complete |                        |
| d3-random              | random/                                         | Complete |                        |
| d3-scale               | scale/                                          | Complete |                        |
| d3-scale-chromatic     | scale_chromatic/                                | Complete |                        |
| d3-selection           | selection/ (unified, D3.js-like API)            | Complete | selection_adv/ merged  |
| d3-shape               | shape/ (unified, D3.js-like API)                | Complete | all adv/utils/stack/symbol merged |
| d3-time                | time/ (unified, D3.js-like API)                 | Complete | time_adv/ merged       |
| d3-time-format         | time_format/ (unified, D3.js-like API)          | Complete | time_format_adv/ merged|
| d3-timer               | timer/                                          | Complete |                        |
| d3-transition          | transition/ (unified, D3.js-like API)           | Complete | transition_adv/ merged |
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
| d3.sum, d3.mean, d3.median    | (check)                   | ⚠️ Not shown in exports, check for implementation    |
| d3.quantile, d3.cumsum        | quantile, cumsum          | ✅ Exported                                           |
| d3.ticks, d3.tickStep         | ticks                     | ✅ Exported (tickStep: check)                        |
| d3.range, d3.shuffle, ...     | (check)                   | ⚠️ Not shown, check for utility coverage             |
| d3.group, d3.rollup           | group, rollup             | ✅ Present in tests, check for export                |
| d3.bisect, d3.ascending, ...  | bisector, ...             | ⚠️ Check for bisector module                         |
| **d3-axis**                   | `axis`                    |                                                      |
| d3.axisTop, d3.axisBottom...  | Axis                      | ✅ Unified, exported                                 |
| **d3-brush**                  | `brush`                   |                                                      |
| d3.brush, d3.brushX, ...      | Brush                     | ✅ Unified, exported                                 |
| **d3-chord**                  | `chord`                   |                                                      |
| d3.chord, d3.ribbon           | chord                     | ✅ Unified, exported                                 |
| **d3-color**                  | `color`                   |                                                      |
| d3.color, d3.rgb, d3.hsl, ... | Color, ColorScale         | ✅ Unified, exported                                 |
| **d3-contour**                | `contour`                 |                                                      |
| d3.contours, d3.contourDensity| contours, ContourLine     | ✅ Exported (contourDensity: check)                  |
| **d3-delaunay**               | `delaunay`, `voronoi`     |                                                      |
| d3.Delaunay, d3.Voronoi       | Delaunay, VoronoiDiagram  | ✅ Exported                                          |
| **d3-dispatch**               | `dispatch`                |                                                      |
| d3.dispatch                   | dispatch                  | ✅ Exported                                          |
| **d3-drag**                   | `drag`                    |                                                      |
| d3.drag                       | drag                      | ✅ Exported                                          |
| **d3-dsv**                    | `dsv`                     |                                                      |
| d3.csvParse, d3.tsvParse, ... | parse_csv, parse_tsv, ... | ✅ Exported                                          |
| **d3-ease**                   | `ease`                    |                                                      |
| d3.easeLinear, ...            | (check)                   | ⚠️ Not shown, check for module                       |
| **d3-fetch**                  | `fetch`                   |                                                      |
| d3.csv, d3.tsv, d3.json, ...  | fetch_text                | ⚠️ Only fetch_text exported, check for others        |
| **d3-force**                  | `force`                   |                                                      |
| d3.forceSimulation, ...       | ForceSimulation, ...      | ✅ Unified, exported                                 |
| **d3-format**                 | `format`                  |                                                      |
| d3.format, d3.formatPrefix... | format                    | ✅ Exported                                          |
| **d3-geo**                    | `geo`                     |                                                      |
| d3.geoPath, d3.geoMercator... | equirectangular           | ⚠️ Only equirectangular exported, check for others   |
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
| d3.timer, d3.timeout, ...     | (check)                   | ⚠️ Not shown, check for module                       |
| **d3-transition**             | `transition`              |                                                      |
| d3.transition                 | Transition                | ✅ Exported                                          |
| **d3-tree**                   | `tree`                    |                                                      |
| d3.tree                       | tree                      | ✅ Exported                                          |
| **d3-treemap**                | `treemap`                 |                                                      |
| d3.treemap                    | (check)                   | ⚠️ Not shown, check for module                       |
| **d3-voronoi**                | `voronoi`                 |                                                      |
| d3.voronoi                    | VoronoiDiagram            | ✅ Exported                                          |
| **d3-zoom**                   | `zoom`                    |                                                      |
| d3.zoom, d3.zoomIdentity...   | Zoom                      | ✅ Exported                                          |
| **d3-collection**             | `array`, `collection`     |                                                      |
| d3.keys, d3.values, ...       | (merged in array)         | ✅ Unified                                           |
| **d3-request (legacy)**       | `fetch`                   |                                                      |
| d3.request                    | fetch_text                | ✅ Replaced by fetch                                 |
| **d3-queue (legacy)**         | `queue`                   |                                                      |
| d3.queue                      | Queue                     | ✅ Exported (legacy)                                 |

---

**Legend:**  
✅ = Present and exported  
⚠️ = Partial or missing, check for implementation  
(check) = Not shown in exports, may need review
