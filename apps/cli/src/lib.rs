use crate::args::Cli;
use eyre::Result;

pub mod args;
pub mod commands;
pub mod models;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        args::Commands::New(args) => commands::new::run(args),
    }
}
