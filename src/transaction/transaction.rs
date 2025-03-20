use std::io::empty;
use std::ptr::null;
use serde::{Deserialize, Serialize};
use sha2::digest::Update;
use sha2::{Digest, Sha256};
use crate::transaction::SUBSIDY;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub v_in: Vec<TxInput>,
    pub v_out: Vec<TxOutput>
}

impl Transaction {
    pub fn new_coinbase(to: String, data: String) -> Self {
        let mut data_copy = data.clone();
        if data.is_empty() {
            data_copy = format!("Reward to '{}'", to);
        }

        let tx_in = TxInput {
            tx_id: String::default(),
            v_out: -1,
            script_sig: data_copy,
        };
        let tx_out = TxOutput {
            value: SUBSIDY,
            script_pub_key: to,
        };

        let tx = Self {
            id: String::default(),
            v_in: vec![tx_in],
            v_out: vec![tx_out],
        };
    }

    fn set_id(&mut self) {
        let encoded = serde_json::to_string(self).expect("Failed to serialize transaction");

        let mut hasher = Sha256::new();
        hasher.update(&encoded.as_bytes());

        let hash_result = hasher.finalize();
        self.id = String::from_utf8(hash_result.to_vec()).expect("Failed to hash transaction");
    }
}

struct TxInput {
    pub tx_id: String,
    pub v_out: i64,
    pub script_sig: String,
}

struct TxOutput {
    pub value: i64,
    pub script_pub_key: String,
}