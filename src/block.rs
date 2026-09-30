use serde::{Deserialize, Serialize};

use crate::transaction::Transaction;
use crate::types::{AccountId, Hash, hash_bytes};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Block {
    pub index: u64,
    pub prev_hash: Hash,
    pub proposer: AccountId,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub hash: Hash,
}

impl Block {
    pub fn new(index: u64, prev_hash: Hash, proposer: AccountId, transactions: Vec<Transaction>) -> Self {
        let mut block = Self {
            index,
            prev_hash,
            proposer,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            transactions,
            hash: String::new(),
        };

        block.hash = block.compute_hash();
        block
    }

    pub fn compute_hash(&self) -> Hash {
        let payload = serde_json::to_vec(self).unwrap();
        hash_bytes(&payload)
    }
}
