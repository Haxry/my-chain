use crate::blockchain::Blockchain;
use crate::error::Result;
use crate::transaction::Transaction;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

pub const TARGET_HEXT: usize = 4;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

pub struct Block {
    timestamp: u128,
    transaction: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

impl Block {
    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transaction
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn new_genesis_block(coinbase: Transaction) -> Block {
        Block::new_block(vec![coinbase], String::new(), 0).unwrap()
    }

    pub fn new_block(
        data: Vec<Transaction>,
        prev_block_hash: String,
        height: usize,
    ) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block = Block {
            timestamp,
            transaction: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };

        block.run_proof_if_work()?;
        Ok(block)
    }

    fn run_proof_if_work(&mut self) -> Result<()> {
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transaction.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce,
        );
        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1 = vec![];
        vec1.resize(TARGET_HEXT, '0' as u8);
        println!("{:?}", vec1);
        Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_blockchain(){

    //     let mut b = Blockchain::new().unwrap();
    //     b.add_block("data".to_string()).unwrap();
    //     b.add_block("data2".to_string()).unwrap();
    //     b.add_block("data3".to_string()).unwrap();
    //     dbg!(b);

    // }
}
