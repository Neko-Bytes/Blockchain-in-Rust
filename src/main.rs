mod block;
mod blockchain;
mod transaction;
mod api;

use blockchain::Blockchain;

#[tokio::main]
async fn main() {
    let blockchain = Blockchain::new();

    api::start_node(blockchain).await;
}
