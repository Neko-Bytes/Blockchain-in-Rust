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
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev: String) -> Self{
        Block{
            index, 
            transactions,
            prev,
            hash: String::new(),
            timestamp: 0,
        }
    }

    pub fn calc_hash(&self) -> String{
        let mut block_data = String::new();

        write!(&mut block_data, "{}|", self.index).expect("Failed to write index");
        write!(&mut block_data, "{:?}|", self.transactions).expect("Failed to write transactions");
        write!(&mut block_data, "{}|", self.prev).expect("Failed to write prev hash");
        write!(&mut block_data, "{}|", self.hash).expect("Failed to write hash");
        write!(&mut block_data, "{}", self.timestamp).expect("Failed to write timestamp");

        let mut hasher = Sha256::new();
        hasher.update(block_data.as_bytes());

        let result = hasher.finalize();
        return hex_to_string(result).expect("Failed to encode");
    }
}
