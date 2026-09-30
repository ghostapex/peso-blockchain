use serde::{Deserialize, Serialize};

use crate::types::{Amount, AccountId};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub balance: Amount,
    pub nonce: u64,
}

impl Account {
    pub fn new(id: AccountId, balance: Amount) -> Self {
        Self {
            id,
            balance,
            nonce: 0,
        }
    }
}
