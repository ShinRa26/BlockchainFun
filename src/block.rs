use super::{serde_json, Json};

use transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub struct ResponseBlock<'a> {
    pub message: &'static str,
    pub index: usize,
    pub transactions: &'a Vec<Transaction>,
    pub proof: i32,
    pub previous_hash: String
}

impl<'a> ResponseBlock<'a> {
    pub fn new(message: &'static str, index: usize, transactions: &'a Vec<Transaction>, proof: i32, previous_hash: String) -> ResponseBlock<'a>{
        ResponseBlock {
            message,
            index,
            transactions,
            proof,
            previous_hash,
        }
    }
}

impl<'a> Json for ResponseBlock<'a> {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: i32,
    pub previous_hash: String,
}

impl Block {
    pub fn new(index: usize, txns: &[Transaction], proof: i32, previous_hash: String) -> Block {
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