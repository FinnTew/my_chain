use crate::block_chain::block::Block;
use crate::block_chain::chain_iterator::BlockChainIterator;
use crate::block_chain::{BLOCKS_BUCKET, DB_PATH};
use sled;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct BlockChain {
    tip: String,
    db: sled::Db,
}

impl BlockChain {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let db = sled::open(DB_PATH)
            .expect(format!("Failed to open sled {:?}", DB_PATH).as_str());
        let blocks_tree = db.open_tree(BLOCKS_BUCKET)
            .expect(format!("Failed to open tree: {:?}", BLOCKS_BUCKET).as_str());
        let tip = match blocks_tree.get(b"l") {
            Ok(Some(last_hash)) => {
                last_hash.to_vec()
            }
            Ok(None) => {
                let genesis_block = Block::new_genesis();
                let mut batch = sled::Batch::default();
                batch.insert(&genesis_block.hash, genesis_block.serialize());
                batch.insert(b"l", &genesis_block.hash);
                blocks_tree.apply_batch(batch)
                    .expect(format!("Failed to apply batch: {:?}", &blocks_tree).as_str());
                blocks_tree.flush()
                    .expect("Failed to flush blocks tree");
                genesis_block.hash.as_bytes().to_vec()
            }
            _ => {
                vec![]
            }
        };

        if tip.len() == 0 {
            return Err("Failed to get block from db".into());
        }

        let tip_string = String::from_utf8(tip).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(BlockChain { tip: tip_string, db })
    }

    pub fn push_new_block(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        let mut blocks_tree = self.db.open_tree(BLOCKS_BUCKET).unwrap();

        let last_hash = match blocks_tree.get(b"l") {
            Ok(Some(last_hash)) => {
                last_hash.to_vec()
            }
            _ => {
                vec![]
            }
        };

        let last_hash = String::from_utf8(last_hash).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let new_block = Block::new(data, last_hash);

        let mut batch = sled::Batch::default();
        batch.insert(&new_block.hash, new_block.serialize());
        batch.insert(b"l", &new_block.hash);
        if let Ok(_res) = blocks_tree.apply_batch(batch) {
            self.tip = new_block.hash;
            Ok(())
        } else {
            Err("Failed to apply batch".to_string())?
        }
    }

    pub fn iter(&self) -> BlockChainIterator {
        BlockChainIterator {
            current_hash: self.tip.clone(),
            db: self.db.clone(),
        }
    }

}