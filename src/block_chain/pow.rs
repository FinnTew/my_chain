use crate::block_chain::block::Block;
use crate::block_chain::{MAX_NONCE, TARGET_BIT};
use num_bigint::BigUint;
use sha2::{Sha256, Digest};
use std::ops::Shl;

pub struct ProofOfWork {
    block: Block,
    target: BigUint,
}

impl ProofOfWork {
    pub fn new(block: Block) -> Self {
        let mut target = BigUint::from(1u32);
        target = target.shl(256 - TARGET_BIT);
        Self { block, target }
    }

    pub fn validate(&self) -> bool {
        let mut hash_int = BigUint::from(0u32);
        let data = self.prepare_data(self.block.nonce);
        let hash: [u8; 32] = Sha256::digest(&data).into();
        hash_int = BigUint::from_bytes_be(&hash);
        hash_int < self.target.clone()
    }

    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = [0u8; 32];

        println!("Mining the block containing \"{}\"", self.block.data);

        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = Sha256::digest(&data).into();
            print!("\r{}", hex::encode(&hash));

            let hash_int = BigUint::from_bytes_be(&hash);

            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        println!("\n");

        (nonce, hex::encode(&hash))
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(self.block.previous_hash.as_bytes());  // 修改这里
        data.extend(self.block.data.as_bytes());
        data.extend(&self.block.timestamp.to_be_bytes());
        data.extend(&TARGET_BIT.to_be_bytes());
        data.extend(&nonce.to_be_bytes());
        data
    }
}