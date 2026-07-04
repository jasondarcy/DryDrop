use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct NewArgs {
    name: String,
    #[arg(long, default_value = "barebone")]
    template: String,
    #[arg(long, default_value = ".")]
    output_dir: Option<PathBuf>,
}
