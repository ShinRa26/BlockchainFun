use std::time::{SystemTime, UNIX_EPOCH};
use super::Transaction;
use super::serde_json;
use super::Json;

#[derive(Debug, Serialize)]
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

impl Json for Block {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}