pub(crate) use crate::commands::Commands;
use clap::Parser;

pub mod new;

#[derive(Parser)]
#[command(name = "drydrop", about = "Composable fullstack project generator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
