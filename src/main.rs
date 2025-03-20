mod block_chain;
mod transaction;

fn main() {
    let block_chain = block_chain::chain::BlockChain::new().unwrap();
    block_chain::cli::Cli::new(block_chain).run()
}
