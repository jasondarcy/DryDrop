use crate::directory::directory_name::DirectoryName;
use crate::node::Node;
use std::collections::BTreeMap;

pub struct DirectoryNode {
    pub name: DirectoryName,
    pub children: BTreeMap<String, Node>,
}

impl DirectoryNode {
    pub fn new() -> Self {
        Self {
            name: DirectoryName::root(),
            children: BTreeMap::new(),
        }
    }

    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: DirectoryName::new(name),
            children: BTreeMap::new(),
        }
    }
}

impl Default for DirectoryNode {
    fn default() -> Self {
        Self::new()
    }
}
