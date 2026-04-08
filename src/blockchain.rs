use crate::block::Block;
// use crate::transaction::Transaction;

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
        return blockchain;
    }

    fn create_genesis_block(&mut self){
        let genesis_block = Block::new(
            0,
            vec![],
            String::from("0000000000000000"),
        );
        self.chain.push(genesis_block);
    }
}
