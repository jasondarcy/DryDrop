use clap::Parser;
pub(crate) use crate::commands::Commands;

pub mod new;

#[derive(Parser)]
#[command(name = "drydrop", about = "Composable fullstack project generator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
