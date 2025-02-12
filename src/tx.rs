use crate::error::Result;
use crate::transaction::Transaction;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TxInput {
    pub txid: String,
    pub vout: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TxOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>, //locking script
}

impl TxInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig == unlocking_data
    }
}

impl TxOutput {
    pub fn can_be_unlocked_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}
