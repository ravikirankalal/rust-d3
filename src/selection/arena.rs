use crate::selection::node::Node;
use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct NodeKey; }

pub struct Arena {
    pub nodes: SlotMap<NodeKey, Node>,
}
