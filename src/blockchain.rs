use crate::block::Block;
use crate::transaction::Transaction;

#[allow(dead_code)]
#[derive(Debug, Clone)]

pub struct Blockchain{
    pub chain: Vec<Block>,
    pub diff: usize,
}

impl Blockchain{
    pub fn new() -> Self{
        let mut blockchain = Blockchain{
            chain: Vec::new(),
            diff: 2,
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

        let mut new_block = Block::new(new_index, transactions, prev_hash);

        new_block.mine_block(self.diff);

        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len(){
            let current_block = &self.chain[i];
            let prev_block = &self.chain[i - 1];

            if current_block.hash != current_block.calc_hash() {
                println!("SECURITY BREACH: Block {} data was tampered", i);
                return false;
            }

            if prev_block.hash != current_block.hash {
                println!("SECURITY BREACH: Block {} is not linked to Block {}", i, i-1);
                return false;
            }

            if !current_block.hash.starts_with(&"0".repeat(self.diff)){
                println!("SECURITY BREACH: Block {} was not mined", i);
                return false;
            }
        }

        println!("The chain is valid and secure");
        true
    }
}
