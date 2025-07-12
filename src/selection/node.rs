use crate::selection::NodeKey;
use std::collections::HashMap;

pub struct Node {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub properties: HashMap<String, String>,
    pub data: Option<String>,
    pub children: Vec<NodeKey>,
    pub parent: Option<NodeKey>,
    pub text: Option<String>,
    pub event_handlers: HashMap<String, Vec<Box<dyn FnMut(&mut Node)>>>,
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
            event_handlers: HashMap::new(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            tag: self.tag.clone(),
            attributes: self.attributes.clone(),
            properties: self.properties.clone(),
            data: self.data.clone(),
            children: self.children.clone(),
            parent: self.parent.clone(),
            text: self.text.clone(),
            event_handlers: HashMap::new(), // Do not clone event handlers
        }
    }
}
