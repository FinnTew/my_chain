use crate::block_chain::pow::ProofOfWork;

mod block_chain;

fn main() {
    let mut block_chain = block_chain::chain::BlockChain::new().unwrap();

    // block_chain.push_new_block("Send 1 BTC to Ivan".to_string()).unwrap();
    // block_chain.push_new_block("Send 2 more BTC to Ivan".to_string()).unwrap();

    let mut iter = block_chain.iter();
    loop {
        let block = iter.next();

        println!("Previous Hash: {:?}", block.previous_hash);
        println!("Data: {:?}", block.data);
        println!("Hash: {:?}", block.hash);
        println!("Pow: {:?}", ProofOfWork::new(*block.clone()).validate());

        if block.previous_hash.len() == 0 {
            break;
        }
    }
}
