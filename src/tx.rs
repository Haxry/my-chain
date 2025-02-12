#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TxInput {
    pub txid: String,
    pub vout: i32,
    pub script_sig: String, //unlocking script
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TxOutput {
    pub value: i32,
    pub script_pub_key: String, //locking script
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
