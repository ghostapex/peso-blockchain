use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatorVote {
    pub validator: String,
    pub block_hash: String,
    pub epoch: u64,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finality {
    pub block_hash: String,
    pub votes: Vec<ValidatorVote>,
    pub epoch: u64,
}

impl Finality {
    pub fn is_final(&self, total_validators: u64) -> bool {
        let votes_needed = (total_validators * 2 / 3) + 1;
        self.votes.len() as u64 >= votes_needed
    }
}
