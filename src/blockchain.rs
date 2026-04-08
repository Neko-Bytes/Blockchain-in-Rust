use crate::block::Block;
use crate::transaction::Transaction;

#[allow(dead_code)]
#[derive(Debug, Clone)]

pub struct Blockchain{
    pub chain: Vec<Block>,
}

impl Blockchain{
    pub fn new() -> Self{
        let mut blockchain = Blockchain{
            chain: Vec::new(),
        };

        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self){
        let genesis_block = Block::new(
            0,
            vec![],
            String::from("0000000000000000"),
        );
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>){
        let prev_block = self.chain.last().unwrap();

        let new_index = prev_block.index + 1;
        let prev_hash = prev_block.hash.clone();

        let new_block = Block::new(new_index, transactions, prev_hash);

        self.chain.push(new_block);
    }
}
