use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RPCEndpoint {
    pub name: String,
    pub version: String,
}

pub struct RPCServer {
    pub endpoints: HashMap<String, String>,
}

impl RPCServer {
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
        }
    }

    pub fn register_endpoint(&mut self, path: String, handler: String) {
        self.endpoints.insert(path, handler);
    }

    pub fn get_endpoint(&self, path: &str) -> Option<&String> {
        self.endpoints.get(path)
    }
}
