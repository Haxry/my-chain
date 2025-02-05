
use my_chain::error::Result;
use my_chain::cli::Cli;


fn main() -> Result<()>{
    let mut cli = Cli::new().unwrap();
    cli.run().unwrap();
    Ok(())
}