use crate::file::file_content::FileContent;
use crate::file::file_name::FileName;

pub struct FileNode {
    pub name: FileName,
    pub content: FileContent,
}

impl FileNode {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: FileName::new(name),
            content: FileContent::new(content),
        }
    }
}
