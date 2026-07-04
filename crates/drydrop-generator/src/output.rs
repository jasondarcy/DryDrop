use std::path::PathBuf;

pub struct GenerationOutput {
    pub project_dir: PathBuf,
}

impl GenerationOutput {
    pub fn new(project_dir: PathBuf) -> Self {
        Self {
            project_dir: project_dir,
        }
    }
}
