use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestnetGenesis {
    pub network_name: String,
    pub chain_id: String,
    pub validators: Vec<TestnetValidator>,
    pub faucet_total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestnetValidator {
    pub id: String,
    pub stake: u64,
    pub commission: u64,
}

impl Default for TestnetGenesis {
    fn default() -> Self {
        Self {
            network_name: "PESO Testnet".to_string(),
            chain_id: "peso-testnet-v1".to_string(),
            validators: vec![
                TestnetValidator {
                    id: "validator-1".to_string(),
                    stake: 1_000_000,
                    commission: 5,
                },
                TestnetValidator {
                    id: "validator-2".to_string(),
                    stake: 2_000_000,
                    commission: 5,
                },
                TestnetValidator {
                    id: "validator-3".to_string(),
                    stake: 3_000_000,
                    commission: 5,
                },
            ],
            faucet_total: 1_000_000_000,
        }
    }
}
