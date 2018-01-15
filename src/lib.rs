extern crate crypto;
extern crate serde_json;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

/// Crate Imports
use uuid::Uuid;

/// Module imports
pub mod blockchain;
pub mod transaction;
pub mod block;

use block::{Block, ResponseBlock};
use blockchain::Blockchain;
use transaction::Transaction;

/// Standard Imports
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::exit;


pub trait Json {
    fn to_json(&self) -> String;
}

pub enum HttpStatus {
    OK,
    NotFound,
    Created,
}

impl HttpStatus {
    pub fn as_str(&self) -> &str {
        match *self {
            HttpStatus::OK => "HTTP/1.1 200 OK\r\n\r\n",
            HttpStatus::NotFound => "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            HttpStatus::Created => "HTTP/1.1 201 CREATED\r\n\r\n",
        }
    }
}


pub struct Node {
    blockchain: Blockchain,
    uuid: String,
}

// TODO: Implement asynchronous multithreading
impl Node {
    pub fn run(address: &str) {
        let mut server = Node::create();
        let listener = server.connect(address);
        server.listen(listener);
    }

    fn create() -> Self {
        let mut blockchain = Blockchain::new();
        let uuid = Uuid::new_v4().to_simple_string();
        blockchain.generate_genesis_block();

        Node {
            blockchain,
            uuid,
        }
    }

    fn connect(&self, address: &str) -> TcpListener {
        match TcpListener::bind(address) {
            Ok(server) => server,
            _ => {
                eprintln!("Error binding to address {}, exitting...", address);
                exit(-1)
            } 
        }
    }

    fn listen(&mut self, listener: TcpListener) {
        for stream in listener.incoming() {
            match stream {
                Ok(s) => self.handle_connection(s),
                _ => {
                    eprintln!("Unable to connect to client, exiting...");
                    exit(-1);
                }
            };
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        // Loop here?
        stream.read(&mut buffer).unwrap();
        let buf_to_str = String::from_utf8_lossy(&buffer[..]);
        let request: Vec<&str> = buf_to_str.split("\n").collect();
        let method: Vec<&str> = buf_to_str.split(" ").collect();

        match method[0] {
            "GET" => {
                self.process_get_request(stream, &method[1]);
            },
            "POST" => {
                self.process_post_request(stream, &request, &method[1]);
            }
            _ => {
                eprintln!("Unsupported HTTP Request!");
            }
        }
    }

    fn process_get_request(&mut self, stream: TcpStream, route: &str) { 
        match route {
            "/" => {
                let contents = "Blockchain interface.\nNavigate to /chain to see the full chain.";
                write_response(stream, HttpStatus::OK, contents);
            },
            "/mine" => { 
                let contents = self.mine_block();
                write_response(stream, HttpStatus::OK, &contents);
            },
            "/chain" => {
                let contents = self.get_chain_contents();
                write_response(stream, HttpStatus::OK, &contents);
            },
            _ => {
                write_response(stream, HttpStatus::NotFound, "404 Not Found");
            }
        }
    }

    fn process_post_request(&mut self, stream: TcpStream, full_request: &[&str], route: &str) {
        match route {
            "/transaction/new" => {
                // TODO: Prevent malformed JSON being sent -- Fix splitting issues
                if let Some(raw_data) = full_request.last() {
                    let data: Vec<&str> = raw_data.split("\u{0}").collect();
                    let txn: Transaction = serde_json::from_str(data[0]).unwrap();
                    let index = self.blockchain.new_transaction(txn.sender, txn.recipient, txn.amount);
                    let response = format!("Message: Transaction will be added to Block {}", index+1);
                    write_response(stream, HttpStatus::Created, &response);
                }
            },
            _ => {
                write_response(stream, HttpStatus::NotFound, "404 Not Found");
            }
        }
    }

    fn mine_block(&mut self) -> String {
        // Run proof of work to get next proof
        let copy_bc = self.blockchain.clone(); // Dirty clone!
        let last_block = match copy_bc.last_block() {
            Some(block) => block,
            None => {
                eprintln!("Error reciving last block from mining!");
                exit(-1)
            }
        };
        let last_proof = last_block.proof;
        let proof = self.blockchain.proof_of_work(last_proof);

        // Once solved, forge a new block
        self.forge_new_block(proof, &last_block)
    }

    fn forge_new_block(&mut self, proof: i32, last_block: &Block) -> String {
        // Receive reward for finding proof
        self.blockchain.new_transaction(String::from("0"), self.uuid.clone(), 1.0);

        // Forge the new block by adding it to the chain
        let previous_hash = self.blockchain.hash_block(last_block);
        let block = match self.blockchain.new_block(proof, previous_hash) {
            Some(block) => block,
            None => {
                eprintln!("Error receiving last block!");
                exit(-1)
            }
        };
        let block_hash = &block.previous_hash;
        let resp_block = ResponseBlock::new("New block forged", block.index, &block.transactions, block.proof, block_hash.to_string());
        
        resp_block.to_json()
    }

    fn get_chain_contents(&mut self) -> String {
        self.blockchain.to_json()
    }
}

// generalise all the above into one method
fn write_response(mut stream: TcpStream, status: HttpStatus, contents: &str) {
    let resp = format!("{}{}", status.as_str(), contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}




#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_connection() {
        Node::run("127.0.0.1:8080");
    }
}