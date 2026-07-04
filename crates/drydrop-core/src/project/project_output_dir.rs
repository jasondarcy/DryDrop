use std::path::{Path, PathBuf};
use crate::error::{DryDropResult};
use crate::error::DryDropError::InvalidProjectOutputDir;

pub struct ProjectOutputDir(PathBuf);

impl ProjectOutputDir {
    pub fn new(output_dir: impl Into<String>) -> DryDropResult<Self> {
        let output_dir = output_dir.into();
        if Path::new(&output_dir).try_exists().is_err() {
            return Err(InvalidProjectOutputDir(output_dir));
        }
        Ok(Self(PathBuf::from(output_dir)))
    }
    pub fn value(&self) -> &PathBuf {
        &self.0
    }
}
