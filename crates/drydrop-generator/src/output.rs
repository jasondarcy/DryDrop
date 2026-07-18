use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GenerationOutput {
    pub project_dir: PathBuf,
    pub template: String,
    pub files: Vec<PathBuf>,
}

impl GenerationOutput {
    pub fn new(project_dir: PathBuf, template: String, files: Vec<PathBuf>) -> Self {
        Self {
            project_dir,
            template,
            files,
        }
    }
}
