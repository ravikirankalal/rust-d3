# D3.js to Rust Parity Checklist

This checklist tracks the parity between the official d3.js modules and your Rust port. Mark each as complete or partial as you implement them.

| D3.js Module           | Rust Equivalent? | Status   | Depends On                                    | Notes/Features Missing |
|------------------------|------------------|----------|-----------------------------------------------|------------------------|
| d3-array               | array/           | Complete |                                               | Missing: blur, intern, set ops, sort, summarize, transform |
| d3-collection          | collection/      | Pending  |                                               |                        |
| d3-format              | format/          | Pending  | array/, collection/                           |                        |
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
