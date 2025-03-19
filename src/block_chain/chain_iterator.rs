use std::cell::RefCell;
use std::rc::Rc;
use crate::block_chain::block::Block;
use crate::block_chain::BLOCKS_BUCKET;

pub struct BlockChainIterator {
    pub current_hash: String,
    pub db: sled::Db,
}

impl BlockChainIterator {
    pub fn next(&mut self) -> Box<Block> {
        let block_tree = self.db.open_tree(BLOCKS_BUCKET).unwrap();
        let serialized_block_vec = block_tree.get(self.current_hash.as_bytes()).unwrap().unwrap().to_vec();
        let serialized_block_string = String::from_utf8(serialized_block_vec).unwrap();
        let block = Block::from(serialized_block_string);
        self.current_hash = block.clone().previous_hash;

        Box::new(block)
    }
}