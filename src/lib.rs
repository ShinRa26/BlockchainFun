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

/// Standard Imports
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::exit;
use std::fs::File;


pub trait Json {
    fn to_json(&self) -> String;
}

pub enum HttpStatus {
    OK,
    NotFound,
}

impl HttpStatus {
    pub fn as_str(&self) -> &str {
        match *self {
            HttpStatus::OK => "HTTP/1.1 200 OK\r\n\r\n",
            HttpStatus::NotFound => "HTTP/1.1 404 NOT FOUND\r\n\r\n",
        }
    }
}


pub struct Node {
    blockchain: Blockchain,
    uuid: &'static str,
}

// TODO: Implement asynchronous multithreading
impl Node {
    pub fn run(address: &str, uuid: &'static str) {
        let mut server = Node::create(uuid);
        let listener = server.connect(address);
        server.listen(listener);
    }

    fn create(uuid: &'static str) -> Self {
        let mut blockchain = Blockchain::new();
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
        let request: Vec<&str> = buf_to_str.split(" ").collect();

        match request[0] {
            "GET" => self.process_get_request(stream, &request[1..]),
            "POST" => self.process_post_request(stream, &request[1..]),
            _ => {
                eprintln!("Unsupported HTTP request!")
            }
        }
    }

    fn process_get_request(&mut self, mut stream: TcpStream, request: &[&str]) { 
        match request[0] {
            "/" => {
                let contents = "Blockchain interface.\nNavigate to /chain to see the full chain.";
                write_OK_response(stream, contents);
            },
            "/mine" => { 
                let contents = self.mine_block();
                write_OK_response(stream, &contents);
            },
            "/chain" => {
                let contents = self.get_chain_contents();
                write_OK_response(stream, &contents);
            },
            _ => {
                write_404_response(stream, "404 Not Found");
            }
        }
    }

    fn process_post_request(&mut self, mut stream: TcpStream, request: &[&str]) {
        // TODO: Handle post requests
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
        self.forge_new_block(last_proof, proof, &last_block)
    }

    fn forge_new_block(&mut self, last_proof: i32, proof: i32, last_block: &Block) -> String {
        // Receive reward for finding proof
        self.blockchain.new_transaction("0", self.uuid, 1);

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

fn write_OK_response(mut stream: TcpStream, contents: &str) {
    let resp = format!("{}{}", HttpStatus::OK.as_str(), contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn write_404_response(mut stream: TcpStream, contents: &str) {
    let resp = format!("{}{}", HttpStatus::NotFound.as_str(), contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_connection() {
        let uuid = Uuid::new_v4().to_simple_string();
        // TODO: Fix UUID reference issues
        Node::run("127.0.0.1:8080", "f4w5s5a5zc5c55s4ds8875s3");
    }
}