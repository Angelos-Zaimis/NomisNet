use std::str;
use rocksdb::{DB, Options};
use crate::blockchain::block::Block;
use serde_json;

pub struct BlockChainDB {
    db: DB,
}

impl BlockChainDB {
    pub fn new(path: &str) -> Self {
        let db = DB::open_default(path).expect("Failed to open default DB");
        BlockChainDB { db }
    }

    pub fn save_new_block(&self, new_block: &Block) {
        let key = format!("block:{}", new_block.index);
        let serialized_block = serde_json::to_vec(new_block).expect("Failed to serialize block");

        self.db.put(&key, serialized_block).expect("Failed to save block");

        // Store latest block as a string converted to bytes
        let latest_block_str = new_block.index.to_string();
        self.db.put("latest_block", latest_block_str.as_bytes()).expect("Failed to save latest block index");
    }

    pub fn get_all_blocks(&self) -> Vec<Block> {
        let mut blocks = Vec::new();

        if let Some(latest_block_bytes) = self.db.get("latest_block").ok().flatten() {
            if let Ok(latest_block_str) = String::from_utf8(latest_block_bytes) {
                if let Ok(latest_block_number) = latest_block_str.parse::<u64>() {
                    for block_number in 0..=latest_block_number {
                        if let Some(block) = self.get_block(block_number) {
                            blocks.push(block);
                        }
                    }
                }
            }
        }

        blocks
    }

    pub fn get_block(&self, block_number: u64) -> Option<Block> {
        let key = format!("block:{}", block_number);
        match self.db.get(&key) {
            Ok(Some(data)) => serde_json::from_slice(&data).ok(),
            Ok(None) => None,
            Err(_) => None,
        }
    }

    pub fn get_last_block(&self) -> Option<Block> {
        let latest_block_number = self.db.get("latest_block").ok()?
            .and_then(|b| String::from_utf8(b).ok())
            .and_then(|s| s.parse::<u64>().ok());

        latest_block_number.and_then(|block_number| self.get_block(block_number))
    }
}
