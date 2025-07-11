use crate::selection::NodeKey;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub properties: HashMap<String, String>,
    pub data: Option<String>,
    pub children: Vec<NodeKey>,
    pub parent: Option<NodeKey>,
    pub text: Option<String>,
}

impl Node {
    pub fn new(tag: &str) -> Self {
        Node {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            properties: HashMap::new(),
            data: None,
            children: vec![],
            parent: None,
            text: None,
        }
    }
}
