use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::{AccountId, Amount, Hash, hash_bytes};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountState {
    pub id: AccountId,
    pub balance: Amount,
    pub nonce: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateRoot {
    pub root_hash: Hash,
    pub accounts: HashMap<AccountId, AccountState>,
}

impl StateRoot {
    pub fn from_accounts(accounts: &HashMap<AccountId, AccountState>) -> Self {
        let mut items = accounts.iter().collect::<Vec<_>>();
        items.sort_by(|a, b| a.0.cmp(b.0));

        let payload = items
            .iter()
            .map(|(id, acc)| format!("{}:{}:{}", id, acc.balance, acc.nonce))
            .collect::<Vec<_>>()
            .join("|");

        let root_hash = hash_bytes(payload.as_bytes());

        Self {
            root_hash,
            accounts: accounts.clone(),
        }
    }
}
