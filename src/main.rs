extern crate chrono;
extern crate sha2;

use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, timestamp: i64, data: String, previous_hash: String) -> Block {
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u32, timestamp: i64, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

fn main() {
    let mut blockchain: Vec<Block> = Vec::new();

    // Genesis block
    let genesis_block = Block::new(0, Utc::now().timestamp(), "Genesis Block".to_string(), "0".to_string());
    blockchain.push(genesis_block);

    let block1 = Block::new(1, Utc::now().timestamp(), "Data for Block 1".to_string(), blockchain[0].hash.clone());
    blockchain.push(block1);

    let block2 = Block::new(2, Utc::now().timestamp(), "Data for Block 2".to_string(), blockchain[1].hash.clone());
    blockchain.push(block2);

    println!("{:#?}", blockchain);
}
