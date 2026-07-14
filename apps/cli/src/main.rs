use clap::Parser;
use cli::args::Cli;
use cli::run;
use eyre::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}
