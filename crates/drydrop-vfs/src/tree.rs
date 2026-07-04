use crate::node::Node;

pub struct VfsTree {
    pub root: Node,
}

impl VfsTree {
    pub fn new() -> Self {
        Self {
            root: Node::new(),
        }
    }
}

impl Default for VfsTree {
    fn default() -> Self {
        Self {
            root: Node::new(),
        }
    }
}
