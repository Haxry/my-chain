use std::collections::HashMap;

//use bincode::Result;
use log::info;

use crate::block::Block;
use crate::block::TARGET_HEXT;
use crate::error::Result;
use crate::transaction::{Transaction, TxInput, TxOutput};

#[derive(Debug, Clone)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}

pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        info!("open the blockchain");
        let db = sled::open("data/blocks")?;
        let hash = db
            .get("LAST")?
            .expect("Must create a new block database first");
        info!("found block database");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain {
            current_hash: lasthash,
            db,
        })
    }

    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating a new blockchain");
        let db = sled::open("data/blocks")?;
        info!("Creating new block database");
        let cbtx = Transaction::new_coinbase(address, String::from("")).unwrap();
        let genesis = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            current_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
    }
    pub fn add_block(&mut self, data: Vec<Transaction>) -> Result<()> {
        let lasthash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(data, String::from_utf8(lasthash.to_vec())?, TARGET_HEXT)?;
        // println!("{:?}", new_block);
        self.db
            .insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();

        Ok(())
    }

    // fn find_unspent_transactions(&self, address: &str) -> Vec<Transaction> {
    //     let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
    //     let mut unspend_TXs: Vec<Transaction> = Vec::new();
    //     for block in self.iter() {
    //         for tx in block.get_transaction() {
    //             for index in 0..tx.vout.len() {
    //                 if let Some(ids) = spent_TXOs.get(&tx.id) {
    //                     if ids.contains(&(index as i32)) {
    //                         continue;
    //                     }
    //                     if tx.vout[index as usize].can_be_unlocked_with(address) {
    //                         unspend_TXs.push(tx.clone());
    //                     }
    //                 }
    //             }
    //             if !tx.is_coinbase() {
    //                 for i in &tx.vin {
    //                     if i.can_unlock_output_with(address) {
    //                         match spent_TXOs.get_mut(&i.txid) {
    //                             Some(v) => v.push(i.vout),
    //                             None => {
    //                                 spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     unspend_TXs
    // }
    fn find_unspent_transactions(&self, address: &str) -> Vec<Transaction> {
        let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspend_TXs: Vec<Transaction> = Vec::new();
        for block in self.iter() {
            for tx in block.get_transaction() {
                'outputs: for (index, output) in tx.vout.iter().enumerate() {
                    if let Some(ids) = spent_TXOs.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue 'outputs;
                        }
                    }
                    if output.can_be_unlocked_with(address) {
                        unspend_TXs.push(tx.clone());
                    }
                }
    
                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        if i.can_unlock_output_with(address) {
                            spent_TXOs.entry(i.txid.clone())
                                .or_insert_with(Vec::new)
                                .push(i.vout);
                        }
                    }
                }
            }
        }
        unspend_TXs
    }
    

    pub fn find_UTXO(&self, address: &str) -> Vec<TxOutput> {
        let mut UTXOs: Vec<TxOutput> = Vec::new();
        let unspent_txs = self.find_unspent_transactions(address);
        //println!("no of unspent txs are {:?}", unspent_txs.len());
        for tx in unspent_txs {
            //println!("unspent txs are {:?}", tx);
            for out in tx.vout {
                if out.can_be_unlocked_with(address) {
                    UTXOs.push(out.clone());
                }
            }
        }
        UTXOs
    }

    pub fn find_spendable_outputs(
        &self,
        address: &str,
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let unspent_txs = self.find_unspent_transactions(address);
        let mut accumulated = 0;
        for tx in unspent_txs {
            let txid = tx.id.clone();
            for (index, out) in tx.vout.iter().enumerate() {
                if out.can_be_unlocked_with(address) && accumulated < amount {
                    accumulated += out.value;
                    match unspent_outputs.get_mut(&txid) {
                        Some(v) => v.push(index as i32),
                        None => {
                            unspent_outputs.insert(txid.clone(), vec![index as i32]);
                        }
                    }
                }
            }
        }
        (accumulated, unspent_outputs)
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.current_hash.clone(),
            bc: self,
        }
    }
}

impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
            return match encode_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}

// #[cfg(test)]
// mod tests{
// use super::*;

// #[test]
// fn test_add_block(){
//     let mut b = Blockchain::new().unwrap();
//     b.add_block("data".to_string()).unwrap();
//     b.add_block("data2".to_string()).unwrap();
//     b.add_block("data3".to_string()).unwrap();

//     for block in b.iter(){
//         println!("block {:?}", block);
//     }
// }
// }
