use crate::directory::directory_node::DirectoryNode;
use crate::file::file_node::FileNode;
use crate::node::Node;
use drydrop_error::{DryDropError, DryDropResult};
use std::path::{Component, Path, PathBuf};

pub struct VfsTree {
    root: DirectoryNode,
}

impl VfsTree {
    pub fn new() -> Self {
        Self {
            root: DirectoryNode::new(),
        }
    }

    pub fn insert_file(&mut self, path: impl AsRef<Path>, content: impl Into<String>) -> DryDropResult<()> {
        let components = path_components(path.as_ref())?;
        insert_file_at(&mut self.root, &components, content.into())
    }

    pub fn file_count(&self) -> usize {
        let mut count = 0;
        collect_files(&self.root, PathBuf::new(), &mut |_, _| count += 1);
        count
    }

    pub fn files(&self) -> Vec<(PathBuf, String)> {
        let mut files = Vec::new();
        collect_files(&self.root, PathBuf::new(), &mut |path, content| {
            files.push((path, content.to_string()));
        });
        files
    }

    pub fn root(&self) -> &DirectoryNode {
        &self.root
    }
}

impl Default for VfsTree {
    fn default() -> Self {
        Self::new()
    }
}

fn path_components(path: &Path) -> DryDropResult<Vec<String>> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(DryDropError::InvalidPath(format!(
            "parent directory segments are not allowed: {}",
            path.display()
        )));
    }

    let mut components = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(name) => {
                let Some(segment) = name.to_str() else {
                    return Err(DryDropError::InvalidPath(format!(
                        "path contains non-UTF-8 segment: {}",
                        path.display()
                    )));
                };
                components.push(segment.to_string());
            }
            Component::RootDir | Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) => {}
        }
    }

    if components.is_empty() {
        return Err(DryDropError::InvalidPath(format!(
            "path must contain at least one segment: {}",
            path.display()
        )));
    }

    Ok(components)
}

fn insert_file_at(
    node: &mut DirectoryNode,
    components: &[String],
    content: String,
) -> DryDropResult<()> {
    if components.len() == 1 {
        let name = &components[0];
        if node.children.contains_key(name) {
            return Err(DryDropError::DuplicatePath(name.clone()));
        }

        node.children
            .insert(name.clone(), Node::File(FileNode::new(name, content)));
        return Ok(());
    }

    let dir_name = &components[0];
    if !node.children.contains_key(dir_name) {
        node.children.insert(
            dir_name.clone(),
            Node::Directory(DirectoryNode::with_name(dir_name)),
        );
    }

    match node.children.get_mut(dir_name) {
        Some(Node::Directory(dir)) => insert_file_at(dir, &components[1..], content),
        Some(Node::File(_)) => Err(DryDropError::PathConflict(format!(
            "cannot create directory '{dir_name}' because a file already exists at that path"
        ))),
        None => unreachable!("directory entry was just inserted"),
    }
}

fn collect_files<F>(node: &DirectoryNode, prefix: PathBuf, visitor: &mut F)
where
    F: FnMut(PathBuf, &str),
{
    for (name, child) in &node.children {
        let path = prefix.join(name);
        match child {
            Node::File(file) => visitor(path, file.content.value()),
            Node::Directory(dir) => collect_files(dir, path, visitor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_nested_files() {
        let mut tree = VfsTree::new();
        tree.insert_file("Cargo.toml", "[package]\nname = \"demo\"\n")
            .expect("Cargo.toml should insert");
        tree.insert_file("src/main.rs", "fn main() {}")
            .expect("main.rs should insert");

        assert_eq!(tree.file_count(), 2);

        let files = tree.files();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].0, PathBuf::from("Cargo.toml"));
        assert_eq!(files[1].0, PathBuf::from("src/main.rs"));
    }

    #[test]
    fn rejects_duplicate_paths() {
        let mut tree = VfsTree::new();
        tree.insert_file("src/main.rs", "first")
            .expect("first insert should succeed");

        let err = tree
            .insert_file("src/main.rs", "second")
            .expect_err("duplicate insert should fail");
        assert!(matches!(err, DryDropError::DuplicatePath(_)));
    }

    #[test]
    fn rejects_file_directory_conflicts() {
        let mut tree = VfsTree::new();
        tree.insert_file("src/main.rs", "fn main() {}")
            .expect("file insert should succeed");

        let err = tree
            .insert_file("src/main.rs/extra.txt", "nope")
            .expect_err("file-as-directory conflict should fail");
        assert!(matches!(err, DryDropError::PathConflict(_)));
    }

    #[test]
    fn rejects_parent_dir_segments() {
        let mut tree = VfsTree::new();
        let err = tree
            .insert_file("../escape.txt", "nope")
            .expect_err("parent dir segments should fail");
        assert!(matches!(err, DryDropError::InvalidPath(_)));
    }
}
