use std::vec;

use crate::blockchain::Blockchain;
use crate::error::Result;
use crate::tx::{TxInput, TxOutput};
use crate::wallet::{hash_pub_key, Wallets};
use crypto::digest::Digest;
use crypto::ed25519;
use crypto::sha2::Sha256;
use failure::format_err;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TxInput>,
    pub vout: Vec<TxOutput>,
}

impl Transaction {
    pub fn new_UTXO(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Result<Transaction> {
        let mut vin = Vec::new();
        let wallets= Wallets::new()?;
        let wallet = match wallets.get_wallet(from){
            Some(w) => w,
            None => return Err(format_err!("Wallet not found")),
        };
        if let None = wallets.get_wallet(&to){
            return Err(format_err!("to Wallet not found"));
        }

        let mut pub_key_hash = wallet.public_key.clone();
        hash_pub_key(&mut pub_key_hash);
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
                    signature: Vec::new(),
                    pub_key: wallet.public_key.clone(),
                };
                vin.push(input);
            }
        }
        let mut vout = vec![
            TxOutput::new(amount, to.to_string())?,
        ];
        if acc_v.0 > amount {
            vout.push(TxOutput::new(acc_v.0 - amount, from.to_string())?);
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

    pub fn verify(&mut self, prev_txs: HashMap<String, Transaction>) -> Result<bool> {
        if self.is_coinbase() {
            return Ok(true);
        }

        for vin in &self.vin {
            if prev_txs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("Previous transaction is not correct"));
            }
        }

        let mut tx_copy = self.trim_copy();

        for in_id in 0..tx_copy.vin.len() {
            let prev_tx = prev_txs.get(&tx_copy.vin[in_id].txid).unwrap();
            tx_copy.vin[in_id].signature.clear();
            tx_copy.vin[in_id].pub_key = prev_tx.vout[tx_copy.vin[in_id].vout as usize]
                .pub_key_hash
                .clone();
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[in_id].pub_key = Vec::new();
            let signature = self.vin[in_id].signature.clone();
            if !ed25519::verify(
                signature.as_slice(),
                tx_copy.as_bytes(),
                &tx_copy.vin[in_id].pub_key,
            ) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn sign(
        &mut self,
        private_key: &[u8],
        prev_txs: HashMap<String, Transaction>,
    ) -> Result<()> {
        if self.is_coinbase() {
            return Ok(());
        }

        for vin in &self.vin {
            if prev_txs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("Previous transaction is not correct"));
            }
        }

        let mut tx_copy = self.trim_copy();

        for in_id in 0..tx_copy.vin.len() {
            let prev_tx = prev_txs.get(&tx_copy.vin[in_id].txid).unwrap();
            tx_copy.vin[in_id].signature.clear();
            tx_copy.vin[in_id].pub_key = prev_tx.vout[tx_copy.vin[in_id].vout as usize]
                .pub_key_hash
                .clone();
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[in_id].pub_key = Vec::new();
            let signature = ed25519::signature(tx_copy.as_bytes(), private_key);
            self.vin[in_id].signature = signature.to_vec();
        }

        Ok(())
    }

    fn hash(&mut self) -> Result<String> {
        self.id = String::new();
        let data = bincode::serialize(self)?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        Ok(hasher.result_str())
    }

    fn trim_copy(&self) -> Transaction {
        let mut vin = Vec::new();
        let mut vout = Vec::new();
        for input in &self.vin {
            vin.push(TxInput {
                txid: input.txid.clone(),
                vout: input.vout,
                signature: Vec::new(),
                pub_key: Vec::new(),
            });
        }
        for output in &self.vout {
            vout.push(TxOutput {
                value: output.value,
                pub_key_hash: output.pub_key_hash.clone(),
            });
        }

        Transaction {
            id: self.id.clone(),
            vin,
            vout,
        }
    }
}
