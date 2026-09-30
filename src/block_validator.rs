use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockProposal {
    pub proposer: String,
    pub epoch: u64,
    pub block_hash: String,
}

pub struct BlockValidator {
    pub max_block_size: usize,
}

impl BlockValidator {
    pub fn new() -> Self {
        Self {
            max_block_size: 1_000_000,
        }
    }

    pub fn validate_block_size(&self, size: usize) -> bool {
        size <= self.max_block_size
    }

    pub fn validate_proposer(&self, proposer: &str) -> bool {
        !proposer.is_empty()
    }
}
