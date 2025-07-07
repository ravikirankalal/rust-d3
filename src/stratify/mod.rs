// D3 hierarchy stratify for Rust D3
// Provides a simple stratify utility for flat data to tree conversion.

use crate::hierarchy::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FlatNode<T> {
    pub id: String,
    pub parent_id: Option<String>,
    pub value: T,
}

pub fn stratify<T: Clone>(data: Vec<FlatNode<T>>) -> Option<Node<T>> {
    let mut nodes: HashMap<String, Node<T>> = HashMap::new();
    let mut root_id = None;
    for item in &data {
        nodes.insert(item.id.clone(), Node::new(item.value.clone()));
        if item.parent_id.is_none() {
            root_id = Some(item.id.clone());
        }
    }
    // Collect child-parent pairs to avoid double mutable borrow
    let mut moves = Vec::new();
    for item in &data {
        if let Some(pid) = &item.parent_id {
            moves.push((pid.clone(), item.id.clone()));
        }
    }
    for (pid, cid) in moves {
        // Remove the child first
        if let Some(child) = nodes.remove(&cid) {
            // Then get a mutable reference to the parent
            if let Some(parent) = nodes.get_mut(&pid) {
                parent.add_child(child);
            }
        }
    }
    root_id.and_then(|id| nodes.remove(&id))
}

/// Builder for stratify with custom id/parent_id accessors
pub struct StratifyBuilder<T, FId, FParentId>
where
    FId: Fn(&T) -> String,
    FParentId: Fn(&T) -> Option<String>,
{
    id: FId,
    parent_id: FParentId,
    _marker: std::marker::PhantomData<T>,
}

impl<T, FId, FParentId> StratifyBuilder<T, FId, FParentId>
where
    FId: Fn(&T) -> String,
    FParentId: Fn(&T) -> Option<String>,
{
    pub fn build(self, data: &[T]) -> Option<Node<T>>
    where
        T: Clone,
    {
        let mut nodes: HashMap<String, Node<T>> = HashMap::new();
        let mut root_id = None;
        for item in data {
            let id = (self.id)(item);
            let parent_id = (self.parent_id)(item);
            nodes.insert(id.clone(), Node::new(item.clone()));
            if parent_id.is_none() {
                root_id = Some(id.clone());
            }
        }
        let mut moves = Vec::new();
        for item in data {
            let id = (self.id)(item);
            if let Some(pid) = (self.parent_id)(item) {
                moves.push((pid, id));
            }
        }
        for (pid, cid) in moves {
            if let Some(child) = nodes.remove(&cid) {
                if let Some(parent) = nodes.get_mut(&pid) {
                    parent.add_child(child);
                }
            }
        }
        root_id.and_then(|id| nodes.remove(&id))
    }
}

/// Start a stratify builder with a custom id accessor
pub fn stratify_id<T, FId>(_id: FId) -> impl FnOnce(FId) -> StratifyBuilder<T, FId, fn(&T) -> Option<String>>
where
    FId: Fn(&T) -> String,
{
    move |id_fn| StratifyBuilder {
        id: id_fn,
        parent_id: |_| None,
        _marker: std::marker::PhantomData,
    }
}

/// Set a custom parent_id accessor on a stratify builder
pub fn stratify_parent_id<T, FId, FParentId>(id: FId, parent_id: FParentId) -> StratifyBuilder<T, FId, FParentId>
where
    FId: Fn(&T) -> String,
    FParentId: Fn(&T) -> Option<String>,
{
    StratifyBuilder {
        id,
        parent_id,
        _marker: std::marker::PhantomData,
    }
}
