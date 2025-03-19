use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::digest::Update;
use crate::block_chain::pow::ProofOfWork;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Block {
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: i64,
}

impl Block {
    pub fn new(data: String, previous_hash: String) -> Self {
        let mut block = Self {
            timestamp: Utc::now().timestamp(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        let (nonce, hash) = ProofOfWork::new(block.clone()).run();
        block.nonce = nonce;
        block.hash = hash;

        block
    }

    pub fn new_genesis() -> Self {
        Self::new(String::from("Genesis Block"), String::default())
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self)
            .expect("Failed to serialize block")
    }
}

impl From<String> for Block {
    fn from(serialized_str: String) -> Self {
        serde_json::from_str(&serialized_str).expect("Failed to deserialize block")
    }
}