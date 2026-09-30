use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoubleVoteSlash {
    pub validator: String,
    pub block_hash_1: String,
    pub block_hash_2: String,
    pub epoch: u64,
    pub slash_amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InactivitySlash {
    pub validator: String,
    pub epoch: u64,
    pub missed_blocks: u64,
    pub slash_amount: u64,
}

pub struct SlashingEnforcer {
    pub double_vote_slash_rate: f64,
    pub inactivity_slash_rate: f64,
}

impl SlashingEnforcer {
    pub fn new() -> Self {
        Self {
            double_vote_slash_rate: 0.33,
            inactivity_slash_rate: 0.01,
        }
    }

    pub fn slash_for_double_vote(&self, stake: u64) -> u64 {
        (stake as f64 * self.double_vote_slash_rate) as u64
    }

    pub fn slash_for_inactivity(&self, stake: u64) -> u64 {
        (stake as f64 * self.inactivity_slash_rate) as u64
    }
}
