extern crate crypto;

pub mod transaction;
pub mod block;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use block::Block;
use transaction::Transaction;

// #[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_txns: Vec<Transaction>,
}


impl Blockchain {
    pub fn new() -> Self {
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

        self.chain.push(Block::new(chain_length, self.current_txns.as_slice(), proof, previous_hash));
        self.current_txns.clear();
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
            None => None,
        }
    }

    pub fn hash_block(&self, block: &Block) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(&block.to_string());

        hasher.result_str()
    }

    pub fn proof_of_work(&self, last_proof: i32) -> i32 {
        let mut proof = 0;
        let mut hasher = Sha256::new();

        while !self.valid_proof(last_proof, proof, &mut hasher) {
            proof += 1;
            hasher.reset(); // Stops the Sha256 from crashing
        }

        proof
    }

    fn valid_proof(&self, last_proof: i32, proof: i32, hasher: &mut Sha256) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        println!("{}", guess);
        hasher.input_str(&guess);
        let guess_hash = hasher.result_str();
        println!("{}", guess_hash);
        &guess_hash[0..4] == "0000"
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
    fn hash_blocks() {
        let mut bc = Blockchain::new();
        bc.generate_genesis_block();
        bc.new_transaction("Alice", "Bob", 100);
        bc.new_transaction("Bob", "Alice", 20);
        bc.new_block(1200, ""); // <-- FAILED Fix slice length
        
        let last_block = bc.last_block().unwrap();
        let digest1 = bc.hash_block(last_block);
        let digest2 = bc.hash_block(last_block);
        assert_eq!(digest1, digest2);
    }

    #[test]
    fn proofs() {
        let mut bc = Blockchain::new();
        let mut proof = 0;
        let last_proof = 100;
        proof = bc.proof_of_work(last_proof);
        println!("Proof: {}", proof);

        // assert!(proof != 10000);
    }
}