use crate::directory::directory_node::DirectoryNode;
use crate::file::file_node::FileNode;

pub enum Node {
    File(FileNode),
    Directory(DirectoryNode),
}

impl Node {
    pub fn as_directory(&self) -> Option<&DirectoryNode> {
        match self {
            Self::Directory(dir) => Some(dir),
            Self::File(_) => None,
        }
    }

    pub fn as_directory_mut(&mut self) -> Option<&mut DirectoryNode> {
        match self {
            Self::Directory(dir) => Some(dir),
            Self::File(_) => None,
        }
    }
}
