use std::{env, process};
use crate::block_chain::chain::BlockChain;
use crate::block_chain::pow::ProofOfWork;

pub struct Cli {
    block_chain: BlockChain,
}

impl Cli {
    pub fn new(block_chain: BlockChain) -> Self {
        Cli { block_chain }
    }

    pub fn run(&mut self) {
        self.validate_args();

        let args: Vec<String> = env::args().collect();
        match args.get(1).map(String::as_str) {
            Some("add-block") => {
                if let Some(data) = args.get(2) {
                    self.push_block(data.clone())
                } else {
                    self.print_usage();
                    process::exit(1);
                }
            }
            Some("print-chain") => {
                self.print_chain();
            }
            _ => {
                self.print_usage();
                process::exit(1);
            }
        }
    }

    fn validate_args(&self) {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            self.print_usage();
            process::exit(1);
        }

        match args.get(1).map(String::as_str) {
            Some("add-block") => {
                if args.len() != 3 {
                    self.print_usage();
                    process::exit(1);
                }
            }
            Some("print-chain") => {}
            _ => {
                self.print_usage();
                process::exit(1);
            }
        }
    }

    fn print_usage(&self) {
        println!("Usage: cli [command]");
        println!("Commands:");
        println!("  add-block <data>   Add a new block with the given data");
        println!("  print-chain        Print the entire blockchain");
    }

    fn push_block(&mut self, data: String) {
        self.block_chain.push_new_block(data.clone()).unwrap();
        println!("Successfully pushed {}", data)
    }

    fn print_chain(&self) {
        let mut iter = self.block_chain.iter();

        loop {
            let block = iter.next();

            println!("Previous Hash: {:?}", block.previous_hash);
            println!("Data: {:?}", block.data);
            println!("Hash: {:?}", block.hash);
            println!("PoW: {:?}", ProofOfWork::new(*block.clone()).validate());
            println!();

            if block.previous_hash.len() == 0 {
                break;
            }
        }
    }
}