use crate::error::Result;
use crypto::sha2::Sha256;
use crypto::digest::Digest;




#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TxInput>,
    pub vout: Vec<TxOutput>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TxInput {
    pub txid: String,
    pub vout: i32,
    pub script_sig: String,//unlocking script
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TxOutput {
    pub value: i32,
    pub script_pub_key: String, //locking script
}

impl Transaction {
    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction>{
        if data == String::from(""){
            data += &format!("Reward to '{}'", to);
        }

        let mut tx = Transaction{
            id: String::new(),
            vin: vec![TxInput{
                txid: String::from(""),
                vout: -1,
                script_sig: data,
            }],
            vout: vec![TxOutput{
                value: 100,
                script_pub_key: to,
            }],
        };
        tx.set_id();
        Ok(tx)
    }

    fn set_id(&mut self) -> Result<()>{
        let mut hasher = Sha256::new();
        let data = bincode::serialize(self)?;
        hasher.input(&data);
        self.id = hasher.result_str();
        Ok(())
    }

    pub fn is_coinbase(&self) -> bool{
        self.vin.len() == 1 && self.vin[0].txid == String::from("") && self.vin[0].vout == -1
    }
}


impl TxInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool{
        self.script_sig == unlocking_data
    }
    
}

impl TxOutput {
    pub fn can_be_unlocked_with(&self, unlocking_data: &str) -> bool{
        self.script_pub_key == unlocking_data
    }
    
}