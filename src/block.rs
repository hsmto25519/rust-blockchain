use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u128,
    pub data: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The time is something wrong")
            .as_millis();

        let hash = Self::calculate_hash(index, &previous_hash, &data, timestamp);

        Block {
            index,
            hash,
            previous_hash,
            timestamp,
            data,
        }
    }

    fn calculate_hash(index: u64, previous_hash: &str, data: &str, timestamp: u128) -> String {
        let input = format!("{index}{previous_hash}{data}{timestamp}");
        let hash = format!("{:x}", Sha256::digest(input.as_bytes()));
        hash
    }

    pub fn is_valid(&self, previous_block: &Block) -> bool {
        self.previous_hash == previous_block.hash
            && self.hash
                == Self::calculate_hash(self.index, &self.previous_hash, &self.data, self.timestamp)
    }
}
