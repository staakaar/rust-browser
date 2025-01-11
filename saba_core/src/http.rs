use crate::error::Error;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String
}

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value
        }
    }
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Result<Self, Error> {
        let preprocessed_response = raw_response.trim_start().replace("\n\r", "\n");

        let (status_line, remaining) = match preprocessed_response.split_once("\n") {
            Some((status_line, remaining)) => (status_line, remaining),
            None => {
                return Err(Error::NetWork(format!(
                        "Invalid HTTP response {}", 
                        preprocessed_response
                )))
            },
        };

        let (headers, body) = match remaining.split_once("\n\n") {
            Some((headers, body)) => {
                let mut headers = Vec::new();
                for header in headers.split('\n') {
                    let splitted_header: Vec<&str> = header.splitn(2, ":").collect();
                    headers.push(Header::new(
                        String::from(splitted_header[0].trim()),
                        String::from(splitted_header[1].trim()),
                    ));
                }
                (headers, body)
            }
            None => (Vec::new(), remaining),
        };

        let statuses: Vec<&str> = status_line.split(' ').collect();

        Ok(Self {
            version: statuses[0].to_string(),
            status_code: statuses[1].parse().unwrap_or(404),
            reason: statuses[2].to_string(),
            headers,
            body: body.to_string(),
        })
    }
}