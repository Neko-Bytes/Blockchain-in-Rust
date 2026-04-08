mod block;
mod blockchain;
mod transaction;

use transaction::Transaction;
use block::Block;
use blockchain::Blockchain;

fn main() {

    let mut blockchain = Blockchain::new();

    println!("Mining Block 1");
    let tx1 = Transaction::new(String::from("Alice"), String::from("Bob"), 100.50);
    let tx2 = Transaction::new(String::from("Alice"), String::from("Charlie"), 300.20);

    let transactions = vec![tx1, tx2];
    blockchain.add_block(transactions);

    println!("Mining Block 2");
    let tx3 = Transaction::new(String::from("Dominic"), String::from("Denis"), 1350.2);
    blockchain.add_block(vec![tx3]);

    println!("{:#?}", blockchain);
}
