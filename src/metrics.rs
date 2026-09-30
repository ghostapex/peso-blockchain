pub struct MetricsCollector {
    pub blocks_produced: u64,
    pub transactions_processed: u64,
    pub validators_active: u64,
    pub total_stake: u64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            blocks_produced: 0,
            transactions_processed: 0,
            validators_active: 0,
            total_stake: 0,
        }
    }

    pub fn record_block(&mut self) {
        self.blocks_produced += 1;
    }

    pub fn record_transaction(&mut self) {
        self.transactions_processed += 1;
    }

    pub fn set_validators(&mut self, count: u64, stake: u64) {
        self.validators_active = count;
        self.total_stake = stake;
    }

    pub fn get_tps(&self) -> f64 {
        if self.blocks_produced == 0 {
            0.0
        } else {
            self.transactions_processed as f64 / self.blocks_produced as f64
        }
    }
}
