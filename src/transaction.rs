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
}

impl ToString for Transaction {
    fn to_string(&self) -> String {
        format!("{{sender: {}, recipient: {}, amount: {:?}}}", self.sender, self.recipient, self.amount)
    }
}