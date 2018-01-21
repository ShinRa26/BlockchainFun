// TODO: Extract Node out into separate file

extern crate crypto;
extern crate serde_json;
extern crate uuid;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

/// Crate Imports
use uuid::Uuid;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde_json::{Value};

/// Module imports
pub mod blockchain;
pub mod transaction;
pub mod block;

use block::{Block, ResponseBlock};
use blockchain::Blockchain;
use transaction::Transaction;

/// Standard Imports
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::exit;
use std::collections::HashSet;


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
    nodes: HashSet<String>,
}

// TODO: Implement asynchronous multithreading
impl Node {
    pub fn run(address: &str) {
        let mut server = Node::create(address);
        let listener = server.connect(address);
        server.listen(listener);
    }

    fn create(address: &str) -> Self {
        let mut blockchain = Blockchain::new();
        let uuid = Uuid::new_v4().to_simple_string();
        blockchain.generate_genesis_block();

        let mut nodes = HashSet::new();
        nodes.insert(address.to_owned());

        Node {
            blockchain,
            uuid,
            nodes,
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
            "/nodes" => {
                let contents = self.get_node_list();
                write_response(stream, HttpStatus::OK, &contents);
            },
            "/nodes/resolve" => {
                let contents = self.consensus();
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
            // Maybe have to move nodes over to the chain itself
            "/nodes/register" => {
                if let Some(raw_data) = full_request.last() {
                    let data: Vec<&str> = raw_data.split("\u{0}").collect();
                    let json_data: Value = serde_json::from_str(data[0]).unwrap();
                    let nodes_list = json_data["nodes"].as_array().unwrap();
                    let content = self.register_all_nodes(nodes_list);
                    write_response(stream, HttpStatus::Created, &content);
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

    fn get_node_list(&mut self) -> String {
        let mut node_list = String::from("Nodes: [\n");
        for node in &self.nodes {
            node_list.push_str(&node);
            node_list.push_str(",\n");
        }
        node_list.push_str("]");
        node_list
    }

    fn register_all_nodes(&mut self, nodes: &Vec<Value>) -> String {
        for node in nodes {
            let addr = node.as_str().unwrap();
            self.register_node(addr.to_owned());
        }

        String::from("New nodes have been registered.")
    }

    fn register_node(&mut self, address: String) {
        let addr_split: Vec<&str> = address.split("//").collect();
        self.nodes.insert(addr_split[1].to_owned());
    }

    fn consensus(&mut self) -> String {
        let replaced = self.resolve_conflicts();

        if replaced {
            let resp = String::from("Our chain has been replaced and updated");
            return resp
        }

        let resp = String::from("Our chain is authoritative");
        resp
    }

    fn valid_chain(&self, chain: &Blockchain) -> bool {
        let mut current_index = 1;
        let mut hasher = Sha256::new();
        let mut last_block = &chain.chain[0];
        let mut valid = true;

        while current_index < chain.chain.len() {
            let block = &chain.chain[current_index];
            println!("{}", last_block.to_json());
            println!("{}", block.to_json());
            println!("\n------------\n");

            // Check that the previous hash of the block is correct
            if block.previous_hash != chain.hash_block(last_block) {
                valid = false;
                break;
            }

            // Check that the proof of work is correct
            if !chain.valid_proof(last_block.proof, block.proof, &mut hasher) {
                valid = false;
                break;
            }

            last_block = block;
            current_index += 1;
            hasher.reset();
        }

        valid
    }

    fn resolve_conflicts(&mut self) -> bool {
        let neighbours = &self.nodes;
        let mut new_chain: Vec<Block>= Vec::new();

        // Only looking for chains longer than ours
        let mut max_length = self.blockchain.chain.len();

        // Grab and verify all chains from the nodes on the network
        for node in neighbours {
            let address = format!("http://{}/chain", node);
            let mut response = reqwest::get(&address).unwrap(); //Fix

            if response.status() == reqwest::StatusCode::Ok {
                let chain_text = response.text().unwrap();
                let other_blockchain: Blockchain = serde_json::from_str(&chain_text).unwrap();
                let length = &other_blockchain.length;

                // Check if the length is longer and the chain is valid
                if (length > &max_length) && (self.valid_chain(&other_blockchain)) {
                    max_length = *length;
                    new_chain = other_blockchain.chain;
                }
            }
        }

        // Replace the chain if a new, valid, and longer chain is discovered
        if new_chain.len() > 0 {
            self.blockchain.chain = new_chain;
            return true
        }
        return false
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