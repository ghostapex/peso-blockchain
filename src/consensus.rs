use crate::block::Block;
use crate::types::AccountId;

#[derive(Clone, Debug)]
pub struct Validator {
    pub id: AccountId,
    pub stake: u64,
}

#[derive(Clone, Debug)]
pub struct Consensus {
    pub validators: Vec<Validator>,
    pub leader_index: usize,
}

impl Consensus {
    pub fn new(validators: Vec<Validator>) -> Self {
        Self {
            validators,
            leader_index: 0,
        }
    }

    pub fn next_leader(&mut self) -> AccountId {
        let leader = self.validators[self.leader_index % self.validators.len()].id.clone();
        self.leader_index += 1;
        leader
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        if block.transactions.is_empty() {
            return false;
        }

        block.transactions.iter().all(|tx| tx.verify())
    }
}
