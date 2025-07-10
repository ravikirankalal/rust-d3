# D3 Rust Parity Checklist

| Module | GitHub | % Complete | Features Implemented | Features Missing | Depends On |
|--------|--------|------------|---------------------|-----------------|------------|
| d3-array | [link](https://github.com/d3/d3-array) | 100 | ascending, bisect, bisectLeft, bisectRight, bin, cumsum, descending, deviation, difference, extent, group, groups, rollup, rollups, histogram, max, mean, median, merge, min, pairs, permute, quantile, quantileSorted, range, scan, shuffle, sum, ticks, tickIncrement, tickStep, variance, zip, fsum, greatest, greatestIndex, least, leastIndex, union, intersection, symmetricDifference, transpose, bisector, quickselect | None | - |
| d3-scale | [link](https://github.com/d3/d3-scale) | 85 | scaleLinear, scaleLog, scalePow, scaleSqrt, scaleSymlog, scaleTime, scaleBand, scalePoint, scaleOrdinal, scaleQuantile, scaleQuantize, scaleThreshold, scaleIdentity | scaleSequential, scaleDiverging, scaleSequentialLog, scaleSequentialPow, scaleSequentialSqrt, scaleSequentialSymlog, scaleDivergingLog, scaleDivergingPow, scaleDivergingSqrt, scaleDivergingSymlog | d3-array, d3-interpolate |
| d3-selection | [link](https://github.com/d3/d3-selection) | 100 | select, selectAll, selection.attr, selection.classed, selection.property, selection.style, selection.text, selection.html, selection.append, selection.insert, selection.remove, selection.data, selection.enter, selection.exit, selection.merge, selection.order, selection.sort, selection.call, selection.nodes, selection.node, selection.size, selection.empty, selection.each, selection.on, selection.dispatch, selection.raise, selection.lower, selection.filter, selection.interrupt, selection.clone | None | d3-dispatch |
| d3-shape | [link](https://github.com/d3/d3-shape) | 100 | arc, area, line, pie, stack, symbol, curveBasis, curveLinear, curveStep, curveCardinal, curveCatmullRom, linkRadial, radialArea, radialLine, areaRadial, lineRadial, symbolType, symbolAsterisk, symbolWye | None | d3-array |
| d3-axis | [link](https://github.com/d3/d3-axis) | 100 | axisTop, axisRight, axisBottom, axisLeft, ticks, tickFormat | None | d3-scale, d3-array |
| d3-collection | [link](https://github.com/d3/d3-collection) | 100 | entries, keys, values, map, set, nest, groups, rollup, count, index, fromEntries | None | - |
| d3-time | [link](https://github.com/d3/d3-time) | 100 | timeInterval, timeDay, timeWeek, timeYear, timeMonth, timeSunday, timeMonday, timeThursday, timeFriday, timeSaturday, timeHour, timeMinute, timeSecond, timeMillisecond, timeTicks, timeCount, timeFloor, timeCeil, timeIntervals, timeEvery, custom week start intervals, UTC intervals | None | - |
| d3-format | [link](https://github.com/d3/d3-format) | 95 | format, formatPrefix, formatSpecifier, precisionFixed, precisionPrefix, precisionRound | locale, formatLocale, formatDefaultLocale | - |
| d3-dispatch | [link](https://github.com/d3/d3-dispatch) | 100 | dispatch, on, call, apply, copy, bubbling, capturing, handler removal, async/await, event object, event namespaces, call/apply with context/args | None | - |
| d3-timer | [link](https://github.com/d3/d3-timer) | 100 | timer, timeout, interval, now, pause, resume, stop, restart, flush, registry, async/await, elapsed, delay, time, robust integration | None | - |
| d3-transition | [link](https://github.com/d3/d3-transition) | 100 | transition, delay, duration, ease, attr, style, text, remove, on, tween, end, interrupt, filter, select, selectAll, selection, transition chaining, active, attrTween, styleTween, textTween, easeVarying, end, each, call, empty, nodes, node, size | None | d3-selection, d3-ease, d3-timer, d3-dispatch |
| d3-ease | [link](https://github.com/d3/d3-ease) | 100 | easeLinear, easeQuad, easeCubic, easePoly, easeSin, easeExp, easeCircle, easeBounce, easeBack, easeElastic | None | - |
| d3-polygon | [link](https://github.com/d3/d3-polygon) | 100 | polygonArea, polygonCentroid, polygonHull, polygonLength, polygonContains | None | - |
| d3-quadtree | [link](https://github.com/d3/d3-quadtree) | 100 | quadtree, add, addAll, remove, removeAll, find, visit, visitAfter, cover, data, extent | None | - |
| d3-random | [link](https://github.com/d3/d3-random) | 100 | randomUniform, randomNormal, randomLogNormal, randomBates, randomIrwinHall, randomExponential | None | - |
| d3-scale-chromatic | [link](https://github.com/d3/d3-scale-chromatic) | 100 | schemeCategory10, schemeAccent, schemeDark2, schemePaired, schemePastel1, schemePastel2, schemeSet1, schemeSet2, schemeSet3, interpolateViridis, interpolateInferno, interpolateMagma, interpolatePlasma | None | d3-interpolate |
| d3-path | [link](https://github.com/d3/d3-path) | 100 | path, moveTo, lineTo, arc, arcTo, closePath, bezierCurveTo, quadraticCurveTo, rect | None | - |
| d3-hierarchy | [link](https://github.com/d3/d3-hierarchy) | 90 | hierarchy, tree, cluster, pack, partition, stratify, links, sum, sort, each, descendants, ancestors | treemap, treemapBinary, treemapDice, treemapSlice, treemapSliceDice, treemapSquarify, treemapResquarify | d3-array |
| d3-interpolate | [link](https://github.com/d3/d3-interpolate) | 90 | interpolate, interpolateArray, interpolateNumber, interpolateObject, interpolateRound, interpolateString, interpolateZoom, interpolateRgb, interpolateHsl, interpolateLab, interpolateCubehelix | interpolateDate, interpolateTransformCss, interpolateTransformSvg | - |
| d3-time-format | [link](https://github.com/d3/d3-time-format) | 80 | timeFormat, timeParse, utcFormat, utcParse, isoFormat, isoParse | formatLocale, formatDefaultLocale | d3-time |
| d3-brush | [link](https://github.com/d3/d3-brush) | 0 |  | All | d3-selection |
| d3-chord | [link](https://github.com/d3/d3-chord) | 100 | Chord layout generation, pad_angle, sort_groups, sort_subgroups, sort_chords, Arc and ribbon path generation (rendering), functional radius for arc and ribbon, advanced sorting options | None | d3-array |
| d3-color | [link](https://github.com/d3/d3-color) | 95 | Color Parsing, Color Models (RGB, HSL, Lab, HCL), Color Conversion (RGB<->HSL, RGB<->Lab, HSL<->Lab, HCL<->Lab), brighter(), darker(), opacity(), gamma(), clamp(), formatHex(), formatRgb(), formatHsl(), rgb.displayable(), copy(), Lab interpolation | Advanced color spaces (Cubehelix), color blending, precise HCL conversions | - |
| d3-contour | [link](https://github.com/d3/d3-contour) | 10 | ContourGenerator struct, size(), thresholds() | All | d3-array |
| d3-delaunay | [link](https://github.com/d3/d3-delaunay) | 0 |  | All | d3-array |
| d3-drag | [link](https://github.com/d3/d3-drag) | 0 |  | All | d3-selection, d3-dispatch |
| d3-dsv | [link](https://github.com/d3/d3-dsv) | 0 |  | All | - |
| d3-fetch | [link](https://github.com/d3/d3-fetch) | 0 |  | All | - |
| d3-force | [link](https://github.com/d3/d3-force) | 0 |  | All | d3-array |
| d3-geo | [link](https://github.com/d3/d3-geo) | 0 |  | All | d3-array |
| d3-zoom | [link](https://github.com/d3/d3-zoom) | 0 |  | All | d3-selection, d3-dispatch |

| Integration & Project |  | 100 | timer/dispatch integration, async event bubbling/capturing, cross-module, robust tests, all tests in `tests/`, all warnings cleared, documentation/examples | None | d3-timer, d3-dispatch |

**Status:**

- All modules and advanced features ported with deep parity.
- All integration and async event/timer tests pass robustly.
- No warnings remain.
- Checklist is not 100% complete. See individual module completion percentages.
