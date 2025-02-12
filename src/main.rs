use my_chain::cli::Cli;
use my_chain::error::Result;

fn main() -> Result<()> {
    let mut cli = Cli::new().unwrap();
    cli.run().unwrap();
    Ok(())
}
