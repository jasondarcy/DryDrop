use std::path::{Path, PathBuf};
use clap::Parser;

#[derive(Parser)]
pub struct NewArgs {
    pub name: String,
    #[arg(long, default_value = "barebone")]
    pub template: String,
    #[arg(long, default_value = ".")]
    pub output_dir: Option<PathBuf>,
}

impl NewArgs {
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn template(&self) -> &str {
        &self.template
    }
    
    pub fn output_dir(&self) -> Option<&Path> {
        self.output_dir.as_deref()
    }
}
