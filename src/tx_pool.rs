use std::collections::VecDeque;

use crate::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct TxPool {
    pub queue: VecDeque<Transaction>,
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, tx: Transaction) {
        self.queue.push_back(tx);
    }

    pub fn take_batch(&mut self, max: usize) -> Vec<Transaction> {
        let mut batch = Vec::new();
        while batch.len() < max && !self.queue.is_empty() {
            if let Some(tx) = self.queue.pop_front() {
                batch.push(tx);
            }
        }
        batch
    }
}
