use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Faucet {
    pub total_supply: u64,
    pub distributed: u64,
    pub drip_amount: u64,
}

impl Faucet {
    pub fn new(total: u64) -> Self {
        Self {
            total_supply: total,
            distributed: 0,
            drip_amount: 1_000_000,
        }
    }

    pub fn request(&mut self, account: &str) -> Result<u64, &str> {
        if self.distributed + self.drip_amount > self.total_supply {
            return Err("Faucet empty");
        }

        self.distributed += self.drip_amount;
        Ok(self.drip_amount)
    }
}
