use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::account::Account;
use crate::block::Block;
use crate::transaction::Transaction;
use crate::types::{AccountId, Amount, Hash};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Ledger {
    pub accounts: HashMap<AccountId, Account>,
    pub last_hash: Hash,
    pub height: u64,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            last_hash: String::from("0"),
            height: 0,
        }
    }

    pub fn add_account(&mut self, account_id: AccountId, balance: Amount) {
        self.accounts
            .insert(account_id.clone(), Account::new(account_id, balance));
    }

    pub fn get_balance(&self, account_id: &str) -> Amount {
        self.accounts
            .get(account_id)
            .map(|account| account.balance)
            .unwrap_or(0)
    }

    pub fn get_account(&self, account_id: &str) -> Option<Account> {
        self.accounts.get(account_id).cloned()
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), String> {
        if !tx.verify() {
            return Err("invalid signature".to_string());
        }

        let sender = self
            .accounts
            .get_mut(&tx.sender)
            .ok_or_else(|| format!("sender {} not found", tx.sender))?;

        if sender.balance < tx.amount {
            return Err(format!("insufficient funds: {} < {}", sender.balance, tx.amount));
        }

        if sender.nonce != tx.nonce {
            return Err(format!("nonce mismatch: {} != {}", sender.nonce, tx.nonce));
        }

        sender.balance -= tx.amount;
        sender.nonce += 1;

        let receiver = self
            .accounts
            .entry(tx.receiver.clone())
            .or_insert(Account::new(tx.receiver.clone(), 0));

        receiver.balance += tx.amount;

        Ok(())
    }

    pub fn apply_block(&mut self, block: &Block) -> Result<(), String> {
        if block.index != self.height + 1 {
            return Err(format!("block index mismatch: expected {}, got {}", self.height + 1, block.index));
        }

        if !block.prev_hash.is_empty() && block.prev_hash != self.last_hash {
            return Err(format!("prev hash mismatch"));
        }

        for tx in &block.transactions {
            self.apply_transaction(tx)?;
        }

        self.last_hash = block.hash.clone();
        self.height = block.index;

        Ok(())
    }
}
