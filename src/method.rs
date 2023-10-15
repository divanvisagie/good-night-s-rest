use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Method {
    pub fn from_string(method: String) -> Method {
        let  method = method.to_uppercase();
        match method.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => panic!("Invalid method {}", method),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
        };
        write!(f, "{}", method)
    }
}

