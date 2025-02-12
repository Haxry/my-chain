use std::process::exit;

use crate::blockchain::Blockchain;
use crate::error::Result;
use crate::transaction::Transaction;
use crate::wallet::Wallets;
use clap::Command;
use clap::arg;

pub struct Cli {}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1.0")
            .author("haxry")
            .about("A simple blockchain implementation in Rust")
            .subcommand(Command::new("printchain").about("Print all the blocks in the blockchain"))
            .subcommand(Command::new("createwallet").about("Create a new wallet"))
            .subcommand(Command::new("listaddresses").about("List all addresses in the wallet"))
            .subcommand(
                Command::new("getbalance")
                    .about("get balance in the blockchain")
                    .arg(arg!(<ADDRESS>"'The Address it get balance for'")),
            )
            .subcommand(
                Command::new("create")
                    .about("create a new blockchain")
                    .arg(arg!(<ADDRESS>"'The Address to send genesis block reward to'")),
            )
            .subcommand(
                Command::new("send")
                    .about("send amount to address")
                    .arg(arg!(<FROM> "'The address to send from'"))
                    .arg(arg!(<TO> "'The address to send to'"))
                    .arg(
                        arg!(<AMOUNT> "'The amount to send'")
                            .value_parser(clap::value_parser!(i32)),
                    ),
            )
            .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::create_blockchain(address.to_string())?;
                println!("Success! Created a new blockchain");
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::new()?;
                let utxos = bc.find_UTXO(&address);
                //println!("length of utxos: {}", utxos.len());
                let mut balance = 0;
                for out in utxos {
                    println!("{:?}", out);
                    balance += out.value;
                }
                println!("Balance of {}: {}", address, balance);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(from) = matches.get_one::<String>("FROM") {
                String::from(from)
            } else {
                println!("Missing 'FROM' address");
                exit(1)
            };
            let to = if let Some(to) = matches.get_one::<String>("TO") {
                String::from(to)
            } else {
                println!("Missing 'TO' address");
                exit(1)
            };
            let amount = if let Some(amount) = matches.get_one::<i32>("AMOUNT") {
                amount
            } else {
                println!("Missing 'AMOUNT'");
                exit(1)
            };
            let mut bc = Blockchain::new()?;
            let tx = Transaction::new_UTXO(from.clone(), to.clone(), *amount, &bc)?;
            bc.add_block(vec![tx])?;
            println!("Success! Sent {} from {} to {}", amount, from, to);
        }
        if let Some(_) = matches.subcommand_matches("printchain") {
            self.printchain();
        }
        if let Some(_) = matches.subcommand_matches("createwallet") {
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet()?;
            ws.save_all()?;
            println!("Success! Created wallet with address: {}", address);
        }
        if let Some(_) = matches.subcommand_matches("listaddresses") {
            let ws = Wallets::new()?;
            let addresses = ws.get_all_address();
            for address in addresses {
                println!("{}", address);
            }
        }
        Ok(())
    }
    fn printchain(&self) {
        let bc = Blockchain::new().unwrap();
        for block in bc.iter() {
            println!("{:?}", block);
        }
    }
}
