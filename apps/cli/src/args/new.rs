use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
pub struct NewArgs {
    pub name: String,
    #[arg(long, default_value = "barebone")]
    pub template: String,
    #[arg(long, default_value = ".")]
    pub output_dir: Option<PathBuf>,
    #[arg(long, default_value = "tera")]
    pub engine: Option<String>,
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
    pub fn engine(&self) -> Option<&str> {
        self.engine.as_deref()
    }
}
