use slotmap::{SlotMap, new_key_type};
use crate::selection::node::Node;

new_key_type! { pub struct NodeKey; }

pub struct Arena {
    pub nodes: SlotMap<NodeKey, Node>,
}
