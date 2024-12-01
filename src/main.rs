mod block;
mod blockchain;
mod constants;

use blockchain::Blockchain;

use log::{debug, error, info};

fn main() {
    pretty_env_logger::init();

    // create a new blockchain
    let mut blockchain = Blockchain::new();
    info!("{:?}", blockchain.chain[0]);

    // Add blocks
    blockchain.add_block("First block".to_string());
    blockchain.add_block("Second block".to_string());
    blockchain.add_block("Third block".to_string());

    // print the blockchain
    for block in &blockchain.chain {
        println!("{:?}", block);
    }

    // Validate the blockchain
    println!(
        "Check if the blockchain is valid: {}",
        blockchain.is_chain_valid()
    );
}
