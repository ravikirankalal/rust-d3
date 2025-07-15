// Arena-based D3-like selection module for Rust
// This is a new module, not a drop-in replacement for the current mod.rs
// You can migrate your code to use this for true D3-like chaining and live selections

pub mod arena;
pub mod data_join;
pub mod node;
pub mod selection;
pub mod utils;

pub use arena::{Arena, NodeKey};
pub use data_join::DataJoin;
pub use node::Node;
pub use selection::Selection;
pub use utils::{Selector, parse_selector};
