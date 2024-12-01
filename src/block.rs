use log::{debug, error, info};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::DIFFICULTY_PREFIX;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u128,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The time is something wrong")
            .as_millis();

        let (nonce, hash) = Self::mine_block(index, &previous_hash, timestamp, &data);

        Block {
            index,
            hash,
            previous_hash,
            timestamp,
            data,
            nonce,
        }
    }

    fn mine_block(index: u64, previous_hash: &str, timestamp: u128, data: &str) -> (u64, String) {
        info!("mining block...");

        // find a nonce that makes the hash start with the DIFFICULTY_PREFIX
        let mut nonce = 0;

        loop {
            if nonce % 100000 == 0 {
                info!("nonce: {}", nonce);
            }
            let hash = Self::calculate_hash(index, previous_hash, timestamp, data, nonce);
            if hash.starts_with(DIFFICULTY_PREFIX) {
                info!("mined! nonce: {}, hash: {}", nonce, &hash,);
                return (nonce, hash);
            }
            nonce += 1;
        }
    }

    fn calculate_hash(
        index: u64,
        previous_hash: &str,
        timestamp: u128,
        data: &str,
        nonce: u64,
    ) -> String {
        let input = format!("{index}{previous_hash}{timestamp}{data}{nonce}");
        let hash = format!("{:x}", Sha256::digest(input.as_bytes()));
        hash
    }

    pub fn is_block_valid(&self, previous_block: &Block) -> bool {
        // check if the previous hash is correct
        if !(self.previous_hash == previous_block.hash) {
            return false;
        }

        if !self.is_valid_hash() {
            return false;
        }

        // check if the hash meets the difficulty requirement
        if !self.hash.starts_with(DIFFICULTY_PREFIX) {
            return false;
        }

        // check if the block is sequential
        if !self.index == previous_block.index + 1 {
            return false;
        }

        true
    }

    fn is_valid_hash(&self) -> bool {
        self.hash
            == Self::calculate_hash(
                self.index,
                &self.previous_hash,
                self.timestamp,
                &self.data,
                self.nonce,
            )
    }
}
