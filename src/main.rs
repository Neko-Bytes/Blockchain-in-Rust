mod block;
mod blockchain;
mod transaction;

use transaction::Transaction;
use block::Block;
use blockchain::Blockchain;

fn main() {
    let first_tx = Transaction::new(
        String::from("Alice"),
        String::from("Bob"),
        100.50,
    );

    let tx_list = vec![first_tx.clone()];
    let blockchain = Blockchain::new();
    let first_block = Block::new(
        1,
        tx_list,
        String::from("0000000000000000"),
    );



    println!("Successfully created {:?}", &first_tx);
}
