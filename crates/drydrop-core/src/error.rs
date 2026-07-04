pub type DryDropResult<T> = Result<T, DryDropError>;

#[derive(Debug, thiserror::Error)]
pub enum DryDropError {
    #[error("Invalid project name: {0}")]
    InvalidProjectName(String),
    #[error("Invalid project output directory: {0}")]
    InvalidProjectOutputDir(String),
    #[error("File already exists: {0}, refuse to overwrite it")]
    FileAlreadyExists(String),
    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error),
}
