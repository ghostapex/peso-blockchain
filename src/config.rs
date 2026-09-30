use serde::{Deserialize, Serialize};

use crate::types::{Amount, AccountId};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenesisAccount {
    pub id: AccountId,
    pub balance: Amount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenesisConfig {
    pub accounts: Vec<GenesisAccount>,
    pub chain_id: String,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            chain_id: "peso-chain-devnet".to_string(),
            accounts: Vec::new(),
        }
    }
}
