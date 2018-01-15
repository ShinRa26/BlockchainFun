use super::{serde_json, Json};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f32,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: f32) -> Self {
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
            amount: 0.0,
        }
    }
}

impl Json for Transaction {
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}