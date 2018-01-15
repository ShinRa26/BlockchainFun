use super::{serde_json, Json};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: i32,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: i32) -> Transaction {
        Transaction {
            sender,
            recipient,
            amount,
        }
    }

    pub fn blank() -> Self {
        Transaction {
            sender: String::from(""),
            recipient: String::from(""),
            amount: 0,
        }
    }
}

impl Json for Transaction {
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

impl Clone for Transaction {
    fn clone(&self) -> Transaction {*self}
}