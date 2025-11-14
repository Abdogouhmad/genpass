use anyhow::Error;
use clap::Parser;
use genpass_lib::cli::GenCli;

fn main() -> Result<(), Error> {
    let cli = GenCli::parse();

    cli.run()?;
    Ok(())
}
