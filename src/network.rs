use std::collections::VecDeque;

use crate::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct Network {
    pub peers: Vec<String>,
    pub tx_queue: VecDeque<Transaction>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            peers: Vec::new(),
            tx_queue: VecDeque::new(),
        }
    }

    pub fn connect(&mut self, peer: String) {
        if !self.peers.contains(&peer) {
            self.peers.push(peer);
        }
    }

    pub fn broadcast_tx(&mut self, tx: Transaction) {
        self.tx_queue.push_back(tx);
    }
}
