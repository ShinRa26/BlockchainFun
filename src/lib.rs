extern crate crypto;

pub mod transaction;
pub mod block;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::process;
use block::Block;
use transaction::Transaction;


pub struct Blockchain {
    chain: Vec<Block>,
    current_txns: Vec<Transaction>,
}


impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: Vec::new(),
            current_txns: Vec::new(),
        }
    }

    pub fn generate_genesis_block(&mut self) {
        self.new_block(100, "");
    }

    pub fn new_block(&mut self, proof: i32, previous_hash: &'static str) {
        let chain_length = self.chain.len() + 1;
        self.chain.push(Block::new(chain_length, &self.current_txns, proof, previous_hash));
    }

    pub fn new_transaction(&mut self, sender: &'static str, recipient: &'static str, amount: i32) -> usize {
        self.current_txns.push(Transaction::new(sender, recipient, amount));

        match self.last_block() {
            Some(block) => block.index,
            None => 1,
        }
    }

    pub fn last_block(&self) -> Option<&Block>{
        match self.chain.last() {
            Some(block) => Some(block),
            None => None
        }
    }

    pub fn hash_block(self, block: &Block) -> &'static str {
        ""
    }
}

impl ToString for Blockchain {
    fn to_string(&self) -> String {
        let mut chain = String::from("chain: [");
        for link in &self.chain {
            chain.push_str(&format!("{}, ", link.to_string()));
        chain.push_str("], ");
        }

        let mut curr_txns = String::from("current_transactions: [");
        for txn in &self.current_txns {
            curr_txns.push_str(&format!("{}, ", txn.to_string()));
        }
        curr_txns.push_str("]");

        format!("{{{}{}}}", chain, curr_txns)
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks() {
        let mut bc = Blockchain::new();
        bc.generate_genesis_block();

        bc.new_transaction("Alice", "Bob", 12);
        bc.new_transaction("Bob", "Alice", 5);

        println!("{:?}", bc.to_string());
        drop(bc);
    }

    // #[test]
    // fn hash_block_test() {
    //     let f = vec![Transaction::new(String::from("send1"), String::from("recv1"), 12), Transaction::new(String::from("send2"), String::from("recv2"), 50)];
    //     let b = Block::new(1, &f, 100, String::from(""));
        
    //     let mut sha = Sha256::new();
    //     sha.input_str(&b.to_string());
    //     println!("{}", sha.result_str());
    // }
}