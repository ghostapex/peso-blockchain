use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub network_name: String,
    pub rpc_host: String,
    pub rpc_port: u16,
    pub data_dir: String,
    pub validators: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network_name: "PESO testnet".to_string(),
            rpc_host: "127.0.0.1".to_string(),
            rpc_port: 3000,
            data_dir: "./data".to_string(),
            validators: vec![
                "validator-1".to_string(),
                "validator-2".to_string(),
                "validator-3".to_string(),
            ],
        }
    }
}
