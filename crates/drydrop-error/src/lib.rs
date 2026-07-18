pub type DryDropResult<T> = Result<T, DryDropError>;

#[derive(Debug, thiserror::Error)]
pub enum DryDropError {
    #[error("Invalid project name: {0}")]
    InvalidProjectName(String),
    #[error("Invalid project output directory: {0}")]
    InvalidProjectOutputDir(String),
    #[error("Unsupported template: {0}")]
    UnsupportedTemplate(String),
    #[error("Template render error: {0}")]
    TemplateRender(String),
    #[error("File already exists: {0}, refuse to overwrite it")]
    FileAlreadyExists(String),
    #[error("Duplicate path in VFS: {0}")]
    DuplicatePath(String),
    #[error("Path conflict in VFS: {0}")]
    PathConflict(String),
    #[error("Invalid VFS path: {0}")]
    InvalidPath(String),
    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error),
}
