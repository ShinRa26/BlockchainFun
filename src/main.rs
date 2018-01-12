extern crate reqwest;

use std::io::Read;

fn main() {
    let mut response = reqwest::get("https://httpbin.org/status/418")
        .expect("Failed to send request");
        println!("{}", response.status());
}