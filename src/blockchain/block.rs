use sha2::{Sha256, Digest};
use std::fmt::Write;
use chrono::Utc;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u64,
    pub difficulty: u32,
}


impl Block {

    pub fn new(index: u64, previous_hash: String, data: String, difficulty: u32) -> Self {
        let timestamp: String = Self::current_timestamp();
        let nonce: u64 = 0;
        let hash = Self::calculate_hash(index, &timestamp, &previous_hash, &data, nonce, &difficulty);

        Self {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
            nonce,
            difficulty
        }
    }

    pub fn calculate_hash(index: u64, timestamp: &String, previous_hash: &String, data: &String, nonce: u64, difficulty: &u32) -> String {
        let mut hash = Sha256::new();
        hash.update(index.to_le_bytes());
        hash.update(timestamp.to_string().as_bytes());
        hash.update(previous_hash.as_bytes());
        hash.update(data.as_bytes());
        hash.update(nonce.to_le_bytes());
        hash.update(difficulty.to_le_bytes());

        let final_hash = hash.finalize();

        let mut hash_str = String::new();
        for byte in final_hash {
            write!(&mut hash_str, "{:02x}", byte).expect("Unable to write");
        }

        hash_str
    }

    pub fn recalculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.data.as_bytes());
        hasher.update(self.nonce.to_le_bytes());
        hasher.update(self.difficulty.to_le_bytes());

        let final_hash = hasher.finalize();
        format!("{:x}", final_hash)
    }

    pub fn is_valid(&self, prev_block: &Block) -> bool {
        if self.hash != self.recalculate_hash() {
            println!("Invalid hast: does not match calculated hash");
            return false;
        }

        if self.previous_hash != prev_block.hash {
            println!("Invalid link: previous hash does not match");
            return false;
        }

        if !self.is_valid_difficulty() {
            println!("Invalid difficulty: hash does not meet difficulty requirements");
            return false;
        }

        true
    }

    pub fn is_valid_difficulty(&self) -> bool {
        let hash_binary = Self::hex_to_binary(&self.hash);
        hash_binary.starts_with(&"0".repeat(self.difficulty as usize))
    }

    fn hex_to_binary(hex: &str) -> String {
        hex.chars()
            .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
            .collect::<String>()
    }

    fn current_timestamp() -> String {
        Utc::now().timestamp().to_string()
    }
}