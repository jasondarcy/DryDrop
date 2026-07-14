use crate::args::new::NewArgs;
use eyre::Result;

pub fn run(args: NewArgs) -> Result<()> {
    println!("{:#?}", args);
    Ok(())
}
