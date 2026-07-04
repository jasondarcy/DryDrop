use crate::directory::directory_node::DirectoryNode;
use crate::file::file_node::FileNode;

pub enum Node {
    File(FileNode),
    Directory(DirectoryNode),
    None,
}

impl Node {
    pub fn new() -> Self {
        Self::None
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::None
    }
}
