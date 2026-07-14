use crate::models::vcs::Vcs;
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser, Clone)]
pub struct NewArgs {
    pub name: String,
    #[arg(long, default_value = "$HOME/.config/drydrop/config.toml")]
    pub config: Option<PathBuf>,
    #[arg(long, default_value = "barebone")]
    pub template: Option<String>,
    #[arg(long, default_value = ".")]
    pub destination: Option<PathBuf>,
    #[arg(long, default_value = "tera")]
    pub engine: Option<String>,
    #[arg(long, default_value = "git")]
    pub vcs: Vcs,
}

impl NewArgs {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn config(&self) -> Option<&Path> {
        self.config.as_deref()
    }
    pub fn template(&self) -> Option<&str> {
        self.template.as_deref()
    }
    pub fn destination(&self) -> Option<&Path> {
        self.destination.as_deref()
    }
    pub fn engine(&self) -> Option<&str> {
        self.engine.as_deref()
    }
    pub fn vcs(&self) -> &Vcs {
        &self.vcs
    }
}
