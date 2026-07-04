use crate::file::file_content::FileContent;
use crate::file::file_name::FileName;

pub struct FileNode {
    pub name: FileName,
    pub content: FileContent,
}
