use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountLock {
    pub account: String,
    pub locked_until: u64,
    pub amount: u64,
}

pub struct ConflictDetector {
    pub locked_accounts: HashMap<String, AccountLock>,
}

impl ConflictDetector {
    pub fn new() -> Self {
        Self {
            locked_accounts: HashMap::new(),
        }
    }

    pub fn check_conflict(&self, account: &str, block_height: u64) -> bool {
        if let Some(lock) = self.locked_accounts.get(account) {
            lock.locked_until > block_height
        } else {
            false
        }
    }

    pub fn lock_account(&mut self, account: String, until: u64, amount: u64) {
        self.locked_accounts.insert(
            account,
            AccountLock {
                account: account.clone(),
                locked_until: until,
                amount,
            },
        );
    }
}
