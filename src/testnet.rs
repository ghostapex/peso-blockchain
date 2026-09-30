use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatorConfig {
    pub id: String,
    pub stake: u64,
    pub peer: String,
    pub rpc_port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestnetConfig {
    pub validators: Vec<ValidatorConfig>,
    pub bootstrap_port: u16,
}

impl Default for TestnetConfig {
    fn default() -> Self {
        Self {
            validators: vec![
                ValidatorConfig {
                    id: "validator-1".to_string(),
                    stake: 100,
                    peer: "127.0.0.1:9001".to_string(),
                    rpc_port: 3001,
                },
                ValidatorConfig {
                    id: "validator-2".to_string(),
                    stake: 200,
                    peer: "127.0.0.1:9002".to_string(),
                    rpc_port: 3002,
                },
                ValidatorConfig {
                    id: "validator-3".to_string(),
                    stake: 300,
                    peer: "127.0.0.1:9003".to_string(),
                    rpc_port: 3003,
                },
            ],
            bootstrap_port: 9000,
        }
    }
}
