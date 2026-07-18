use drydrop_error::DryDropError::FileAlreadyExists;
use drydrop_error::DryDropResult;
use drydrop_vfs::VfsTree;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FsWriter;

impl FsWriter {
    pub fn write_file(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> DryDropResult<PathBuf> {
        let path = path.as_ref();

        if path.exists() {
            return Err(FileAlreadyExists(path.display().to_string()));
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, content.as_ref())?;

        Ok(path.to_path_buf())
    }

    pub fn write_tree(base: impl AsRef<Path>, tree: &VfsTree) -> DryDropResult<Vec<PathBuf>> {
        let base = base.as_ref();
        let mut written = Vec::new();

        for (relative, content) in tree.files() {
            written.push(Self::write_file(base.join(relative), content)?);
        }

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}"))
    }

    #[test]
    fn writes_vfs_tree_to_disk() {
        let base = unique_dir("drydrop-vfs-write");
        let mut tree = VfsTree::new();
        tree.insert_file("Cargo.toml", "[package]\nname = \"demo\"\n")
            .expect("Cargo.toml should insert");
        tree.insert_file("src/main.rs", "fn main() {}")
            .expect("main.rs should insert");

        let written = FsWriter::write_tree(&base, &tree).expect("tree should write");
        assert_eq!(written.len(), 2);
        assert!(base.join("Cargo.toml").is_file());
        assert!(base.join("src/main.rs").is_file());

        let _ = fs::remove_dir_all(base);
    }
}
