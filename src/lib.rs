extern crate crypto;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod blockchain;
pub mod transaction;
pub mod block;

pub trait Json {
    fn to_json(&self) -> String;
}

pub struct Node {
    // Listens for clients on address
    // Parses their HTTP request (Get/Post)
    // If Get, find the data and return
    // If Post, receive data and process
    // DC client
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockchain::{Blockchain};

}