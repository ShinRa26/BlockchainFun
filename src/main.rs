extern crate blockchain;
use blockchain::node::{Node};

fn main() {
    Node::run("127.0.0.1:5000");
}