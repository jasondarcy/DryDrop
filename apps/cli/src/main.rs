use clap::Parser;
use eyre::Result;
use cli::args::Cli;
use cli::run;

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}
