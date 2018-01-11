#[derive(Debug, Copy)]
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

impl ToString for Transaction {
    fn to_string(&self) -> String {
        format!("{{sender: {}, recipient: {}, amount: {:?}}}", self.sender, self.recipient, self.amount)
    }
}

impl Clone for Transaction {
    fn clone(&self) -> Transaction {*self}
}