pub mod new;

use crate::args::new::NewArgs;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    New(NewArgs),
}
