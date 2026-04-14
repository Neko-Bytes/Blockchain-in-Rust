use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};

use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::block::Block;

type SharedState = Arc<Mutex<Blockchain>>;

pub async fn start_node(blockchain: Blockchain){
    let shared_state = Arc::new(Mutex::new(blockchain));

    let app = Router::new()
        .route("/chain", get(get_chain))
        .route("/transactions/new", post(add_tx))
        .route("/mine", get(mine_block))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Blockchain Node running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// Helper functions
async fn get_chain(State(state): State<SharedState>) -> Json<Blockchain>{
    let blockchain = state.lock().unwrap();
    Json(blockchain.clone())
}

async fn add_tx(State(state): State<SharedState>, Json(new_tx): Json<Transaction>) -> String {
    let mut blockchain = state.lock().unwrap();
    blockchain.mempool.push(new_tx);
    String::from("New transaction added\n")
}

async fn mine_block(State(state): State<SharedState>) -> String {
    let (new_index, prev_hash, diff, txs) = {
        let blockchain = state.lock().unwrap();
        if blockchain.mempool.is_empty() {
            return String::from("Mempool is empty, nothing to mine\n");
        }
        let prev_block = blockchain.chain.last().unwrap();
        (prev_block.index + 1, prev_block.hash.clone(), blockchain.diff, blockchain.mempool.clone())
    };

    let new_block = tokio::task::spawn_blocking(move || {
        let mut block = Block::new(new_index, txs, prev_hash);
        block.mine_block(diff);
        block
    }).await.unwrap();

    let mut blockchain = state.lock().unwrap();
    blockchain.chain.push(new_block);
    blockchain.mempool.clear();

    String::from("Block has been mined\n")
}
