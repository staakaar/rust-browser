#![no_std]
#![no_main]
#![cfg_attr(not(target_os = "linux"), no_main)]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    Api::write_string("Hello World\n");
    println!("Hello, from println!");
    Api::exit(42);

    let client = HttpClient::new();
    match client.get("example.com".to_string(), 80, "/".to_string()) {
        Ok(res) => {
            print!("response:\n{:#?}", res);
        }
        Err(e) => {
            print!("error:\n{:#?}", e);
        }
    }
    ()
}

entry_point!(main);