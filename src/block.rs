use std::fmt::format;
use faster_hex::hex_string;
use std::fmt::Write;

use crate::transaction::Transaction;
use sha2::{Sha256, Digest};

#[allow(dead_code)]
#[derive(Debug, Clone)]

pub struct Block{
    pub index: u64,
    pub transactions: Vec<Transaction>,
    pub prev: String,
    pub hash: String,
    pub timestamp: u64,
    pub nonce: u64, // Number used once
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev: String) -> Self{
        let mut new_block = Block{
            index, 
            transactions,
            prev,
            hash: String::new(),
            timestamp: 0,
            nonce: 0,
        };

        new_block.hash = new_block.calc_hash();

        new_block
    }

    pub fn calc_hash(&self) -> String{
        let mut block_data = String::new();

        write!(&mut block_data, "{}|", self.index).expect("Failed to write index");
        write!(&mut block_data, "{:?}|", self.transactions).expect("Failed to write transactions");
        write!(&mut block_data, "{}|", self.prev).expect("Failed to write prev hash");
        write!(&mut block_data, "{}|", self.timestamp).expect("Failed to write timestamp");
        write!(&mut block_data, "{}", self.nonce).expect("Failed to write nonce");

        let mut hasher = Sha256::new();
        hasher.update(block_data.as_bytes());

        let result = hasher.finalize();
        hex_string(&result)
    }

    pub fn mine_block(&mut self, diff: usize){
        let target = "0".repeat(diff);

        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calc_hash();
        }
    }
}
