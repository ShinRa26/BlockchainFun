extern crate crypto;

pub mod transaction;
pub mod block;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use block::Block;
use transaction::Transaction;


use std::process;

pub struct Blockchain<'a> {
    chain: Vec<Block<'a>>,
    current_txns: Vec<Transaction>,
}


impl<'a> Blockchain<'a> {
    pub fn new() -> Blockchain<'a> {
        Blockchain {
            chain: Vec::new(),
            current_txns: Vec::new(),
        }
    }

    pub fn new_block(&'a mut self, proof: i32, previous_hash: String) -> &'a Block {
        let chain_length = self.chain.len() + 1;
        self.chain.push(Block::new(chain_length, &self.current_txns, proof, previous_hash));

        match self.chain.last() {
            Some(block) => block,
            None => {
                eprintln!("Cannot obtain last known block for some reason! Exiting in shame!");
                process::exit(-1);
            }
        }
    }

    pub fn new_transaction(&'a mut self, sender: String, recipient: String, amount: i32) -> usize {
        self.current_txns.push(Transaction::new(sender, recipient, amount));

        match self.last_block() {
            Some(block) => block.index,
            None => 1,
        }
    }

    pub fn last_block(&self) -> Option<&'a Block>{
        match self.chain.last() {
            Some(block) => Some(block),
            None => None
        }
    }

    pub fn hash_block(&self, block: &'a Block) -> String {
        String::from("")
    }
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_block_test() {
        let f = vec![Transaction::new(String::from("send1"), String::from("recv1"), 12), Transaction::new(String::from("send2"), String::from("recv2"), 50)];
        let b = Block::new(1, &f, 100, String::from(""));
        
        let mut sha = Sha256::new();
        sha.input_str(&b.to_string());
        println!("{}", sha.result_str());
    }
}