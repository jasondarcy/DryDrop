use drydrop_error::DryDropError::FileAlreadyExists;
use drydrop_error::DryDropResult;
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
}
