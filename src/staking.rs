use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{AccountId, Amount};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StakeAccount {
    pub validator: AccountId,
    pub amount: Amount,
    pub epoch_locked: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StakePool {
    pub validators: HashMap<AccountId, Amount>,
    pub delegations: HashMap<AccountId, HashMap<AccountId, Amount>>,
    pub total_staked: Amount,
}

impl StakePool {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            delegations: HashMap::new(),
            total_staked: 0,
        }
    }

    pub fn stake_validator(&mut self, validator: AccountId, amount: Amount) {
        *self.validators.entry(validator.clone()).or_insert(0) += amount;
        self.total_staked += amount;
    }

    pub fn delegate(&mut self, delegator: AccountId, validator: AccountId, amount: Amount) {
        self.delegations
            .entry(delegator)
            .or_default()
            .entry(validator)
            .and_modify(|v| *v += amount)
            .or_insert(amount);

        *self.validators.entry(validator.clone()).or_insert(0) += amount;
        self.total_staked += amount;
    }

    pub fn validator_power(&self, validator: &str) -> Amount {
        self.validators.get(validator).copied().unwrap_or(0)
    }
}
