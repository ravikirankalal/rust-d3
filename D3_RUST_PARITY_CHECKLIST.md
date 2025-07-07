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
