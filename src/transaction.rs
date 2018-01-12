use super::{serde_json, Json};

#[derive(Debug, Copy, Serialize)]
pub struct Transaction {
    sender: &'static str,
    recipient: &'static str,
    amount: i32,
}

impl Transaction {
    pub fn new(sender: &'static str, recipient: &'static str, amount: i32) -> Transaction {
        Transaction {
            sender,
            recipient,
            amount,
        }
    }

    pub fn blank() -> Self {
        Transaction {
            sender: "",
            recipient: "",
            amount: 0,
        }
    }
}

impl Json for Transaction {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Clone for Transaction {
    fn clone(&self) -> Transaction {*self}
}