use std::time::{SystemTime, UNIX_EPOCH};
use super::Transaction;

#[derive(Debug)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: i32,
    pub previous_hash: &'static str,
}

impl Block {
    pub fn new(index: usize, txns: &[Transaction], proof: i32, previous_hash: &'static str) -> Block {
        let mut transactions = Vec::new();
        transactions.resize(txns.len(), Transaction::blank());
        transactions.clone_from_slice(txns);

        Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions,
            proof,
            previous_hash,
        }
    }
}

impl ToString for Block {
    fn to_string(&self) -> String {
        let mut txns = String::from("transactions: [");
        for txn in &self.transactions {
            txns.push_str(&format!("{}, ", txn.to_string()));
        }
        txns.push_str("]");

        format!("{{index: {:?}, timestamp: {:?}, {}, proof: {:?}, previous_hash: {}}}", 
            self.index, self.timestamp, txns, self.proof, self.previous_hash)
    }
}