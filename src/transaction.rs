pub struct Transaction<'a> {
    sender: &'a str,
    recipient: &'a str,
    amount: i32,
}

impl<'a> Transaction<'a> {
    pub fn new(sender: &'a str, recipient: &'a str, amount: i32) -> Transaction<'a> {
        Transaction {
            sender,
            recipient,
            amount,
        }
    }
}

impl<'a> ToString for Transaction<'a> {
    fn to_string(&self) -> String {
        format!("{{sender: {}, recipient: {}, amount: {:?}}}", self.sender, self.recipient, self.amount)
    }
}