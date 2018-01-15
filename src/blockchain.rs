use super::*;
use block::Block;
use transaction::Transaction;

use crypto::digest::Digest;
use crypto::sha2::Sha256;


#[derive(Serialize, Clone)]
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
        self.new_block(100, String::from("000000000000000000000000000000000000000000000000000000000000000"));
    }

    pub fn new_block(&mut self, proof: i32, previous_hash: String) -> Option<&Block> {
        let chain_length = self.chain.len() + 1;

        self.chain.push(Block::new(chain_length, self.current_txns.as_slice(), proof, previous_hash));
        self.current_txns.clear();

        self.last_block()
    }

    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: f32) -> usize {
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
        hasher.input_str(&block.to_json());

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
        hasher.input_str(&guess);
        let guess_hash = hasher.result_str();
        println!("{}", guess_hash);

        &guess_hash[0..4] == "0000"
    }
}

impl Json for Blockchain {
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}