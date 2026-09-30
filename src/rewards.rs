use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidatorReward {
    pub validator: String,
    pub epoch: u64,
    pub base_reward: u64,
    pub commission: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakingReward {
    pub staker: String,
    pub validator: String,
    pub epoch: u64,
    pub amount: u64,
}

pub struct RewardCalculator {
    pub base_rate: f64,
    pub inflation_schedule: Vec<(u64, f64)>,
}

impl RewardCalculator {
    pub fn new(base_rate: f64) -> Self {
        Self {
            base_rate,
            inflation_schedule: vec![
                (0, 0.08),
                (100, 0.06),
                (200, 0.04),
                (300, 0.02),
            ],
        }
    }

    pub fn calculate_validator_reward(&self, stake: u64, epoch: u64) -> u64 {
        let rate = self
            .inflation_schedule
            .iter()
            .find(|(e, _)| *e <= epoch)
            .map(|(_, r)| r)
            .copied()
            .unwrap_or(self.base_rate);

        (stake as f64 * rate) as u64
    }

    pub fn calculate_staking_reward(&self, delegated: u64, epoch: u64) -> u64 {
        self.calculate_validator_reward(delegated, epoch)
    }
}
