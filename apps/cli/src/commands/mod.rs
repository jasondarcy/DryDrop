pub mod new;

use clap::Subcommand;
use crate::args::new::NewArgs;

#[derive(Subcommand)]
pub enum Commands {
    New(NewArgs)
}
