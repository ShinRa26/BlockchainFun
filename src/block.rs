use std::time::{SystemTime, Duration, UNIX_EPOCH};
use super::Transaction;

pub struct Block<'a> {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: &'a Vec<Transaction>,
    pub proof: i32,
    pub previous_hash: String,
}

impl<'a> Block<'a> {
    pub fn new(index: usize, transactions: &'a Vec<Transaction>, proof: i32, previous_hash: String) -> Block {
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

impl<'a> ToString for Block<'a> {
    fn to_string(&self) -> String {
        let mut txns = String::from("transactions: [");
        for txn in self.transactions {
            txns.push_str(&format!("{}, ", txn.to_string()));
        }
        txns.push_str("]");

        format!("{{index: {:?}, timestamp: {:?}, {}, proof: {:?}, previous_hash: {}}}", self.index, self.timestamp, txns, self.proof, self.previous_hash)
    }
}