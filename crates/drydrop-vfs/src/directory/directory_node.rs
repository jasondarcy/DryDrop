use std::collections::BTreeMap;
use crate::directory::directory_name::DirectoryName;
use crate::node::Node;

pub struct DirectoryNode {
    pub name: DirectoryName,
    pub children: BTreeMap<String, Node>,
}

impl DirectoryNode {
    pub fn new() -> Self {
        Self {
            name: DirectoryName::new(),
            children: BTreeMap::new(),
        }
    }
}
