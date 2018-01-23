// TODO: Extract Node out into separate file

extern crate crypto;
extern crate serde_json;
extern crate uuid;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

/// Module imports
pub mod node;
pub mod blockchain;
pub mod transaction;
pub mod block;

/// Trait for converting to Json
pub trait Json {
    fn to_json(&self) -> String;
}

/// Supported HTTP Statuses
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