// Arena-based D3-like selection module for Rust
// This is a new module, not a drop-in replacement for the current mod.rs
// You can migrate your code to use this for true D3-like chaining and live selections

pub mod node;
pub mod arena;
pub mod selection;
pub mod data_join;
pub mod utils;

pub use node::Node;
pub use arena::{Arena, NodeKey};
pub use selection::Selection;
pub use data_join::DataJoin;
