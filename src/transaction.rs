use std::vec;

use crate::blockchain::Blockchain;
use crate::error::Result;
use crate::tx::{TxInput, TxOutput};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use failure::format_err;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TxInput>,
    pub vout: Vec<TxOutput>,
}

impl Transaction {
    pub fn new_UTXO(from: String, to: String, amount: i32, bc: &Blockchain) -> Result<Transaction> {
        let mut vin = Vec::new();
        let acc_v = bc.find_spendable_outputs(&from, amount);
        if acc_v.0 < amount {
            return Err(format_err!(
                "Not enough balance: current balance {}",
                acc_v.0
            ));
        }
        for tx in acc_v.1 {
            for out in tx.1 {
                let input = TxInput {
                    txid: tx.0.clone(),
                    vout: out,
                    script_sig: from.clone(),
                };
                vin.push(input);
            }
        }
        let mut vout = vec![TxOutput {
            value: amount,
            script_pub_key: to.clone(),
        }];
        if acc_v.0 > amount {
            vout.push(TxOutput {
                value: acc_v.0 - amount,
                script_pub_key: from.clone(),
            });
        }
        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout,
        };
        tx.set_id()?;
        Ok(tx)
    }

    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
        if data == String::from("") {
            data += &format!("Reward to '{}'", to);
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![TxInput {
                txid: String::from(""),
                vout: -1,
                script_sig: data,
            }],
            vout: vec![TxOutput {
                value: 100,
                script_pub_key: to,
            }],
        };
        tx.set_id();
        Ok(tx)
    }

    fn set_id(&mut self) -> Result<()> {
        let mut hasher = Sha256::new();
        let data = bincode::serialize(self)?;
        hasher.input(&data);
        self.id = hasher.result_str();
        Ok(())
    }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid == String::from("") && self.vin[0].vout == -1
    }
}
