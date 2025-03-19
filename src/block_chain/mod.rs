use std::string::ToString;

pub mod block;
pub mod chain;
pub mod pow;
mod chain_iterator;

const TARGET_BIT: i32 = 24;
const MAX_NONCE: i64 = i64::MAX;
const DB_PATH: &str = "my_db";
const BLOCKS_BUCKET: &str = "blocks";