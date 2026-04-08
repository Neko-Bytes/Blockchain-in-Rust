use crate::transaction::Transaction;

#[allow(dead_code)];
#[derive(Debug, Clone)]

pub struct Block{
    pub index: u64,
    pub transactions: Vec<Transaction>,
    pub prev: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev: String) -> Self{
        Block{
            index, 
            transactions,
            prev,
        }
    }
}
