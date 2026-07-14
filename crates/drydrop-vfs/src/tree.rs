use crate::node::Node;

pub struct VfsTree {
    pub root: Node,
}

impl VfsTree {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }
}
