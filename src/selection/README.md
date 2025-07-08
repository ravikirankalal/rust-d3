# d3-selection (Rust port)

This module aims to port the [d3-selection](https://github.com/d3/d3-selection) API to Rust.

## Parity Goals
- `select`, `select_all`
- `attr`, `style`
- `data`, `enter`, `exit`
- `append`, `remove`
- Event handling (`on`, `dispatch`)
- Integration with other d3 modules

## Status
- **Stub**: API surface is present, but no real DOM or data binding yet.

## Example Usage (planned)
```rust
use rust_d3::selection::Selection;

let mut sel = Selection::select("svg");
sel.attr("width", "400").attr("height", "300");
sel.append("circle").attr("cx", "100").attr("cy", "100").attr("r", "50");
```

## Parity Tracking
See `D3_RUST_PARITY_CHECKLIST.md` for progress.
