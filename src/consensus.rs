use crate::block::Block;
use crate::types::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Validator {
    pub id: AccountId,
    pub stake: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Consensus {
    pub validators: Vec<Validator>,
    pub leader_index: usize,
    pub epoch: u64,
}

impl Consensus {
    pub fn new(validators: Vec<Validator>) -> Self {
        Self {
            validators,
            leader_index: 0,
            epoch: 0,
        }
    }

    pub fn next_leader(&mut self) -> AccountId {
        let leader = self.validators[self.leader_index % self.validators.len()].id.clone();
        self.leader_index += 1;
        if self.leader_index >= self.validators.len() {
            self.epoch += 1;
        }
        leader
    }

    pub fn current_leader(&self) -> AccountId {
        self.validators[self.leader_index % self.validators.len()].id.clone()
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        if block.transactions.is_empty() {
            return false;
        }

        block.transactions.iter().all(|tx| tx.verify())
    }

    pub fn quorum_ok(&self, votes: usize) -> bool {
        votes * 2 > self.validators.len()
    }

    pub fn total_stake(&self) -> u64 {
        self.validators.iter().map(|v| v.stake).sum()
    }
}
