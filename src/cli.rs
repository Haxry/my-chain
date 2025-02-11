use crate::blockchain::Blockchain;
use crate::error::Result;
use clap::Command;
use clap::arg;

pub struct Cli{
    bc: Blockchain,
}

impl Cli{
 pub fn new() -> Result<Cli>{
    Ok(
        Cli{
            bc: Blockchain::new()?,
        }
    )
 }

 pub fn run(&mut self) -> Result<()>{
    let matches = Command::new("blockchain-rust-demo")
    .version("0.1.0")
    .author("haxry")
    .about("A simple blockchain implementation in Rust")
    .subcommand(
        Command::new("printchain")
        .about("Print all the blocks in the blockchain"),
    )
    .subcommand(
        Command::new("addblock")
        .about("Add a block to the blockchain")
        .arg(arg!(<DATA>"'the blockchain data'")),
    )
    .get_matches();
   if let Some(ref matches) = matches.subcommand_matches("addblock") {
    if let Some(c) = matches.get_one::<String>("DATA"){
        self.addblock(String::from(c))?;
    } else{
        println!("No data provided");
    }
   }
   if let Some(_)= matches.subcommand_matches("printchain"){
    self.printchain();
   }
    Ok(())
 }

    fn addblock(&mut self, data: String) -> Result<()>{
        self.bc.add_block(vec![])?;
        Ok(())
    }

    fn printchain(&mut self){
        for block in &mut self.bc.iter(){
            println!("{:?}", block);
        }
    }
}