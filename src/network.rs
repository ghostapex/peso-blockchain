use crate::transaction::Transaction;
use crate::block::Block;
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub from: String,
    pub msg_type: MessageType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageType {
    Transaction(Transaction),
    Block(Block),
    Vote(String), // block hash
}

#[derive(Clone, Debug)]
pub struct Network {
    pub peers: Vec<String>,
    pub tx_queue: VecDeque<Transaction>,
    pub block_queue: VecDeque<Block>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            peers: Vec::new(),
            tx_queue: VecDeque::new(),
            block_queue: VecDeque::new(),
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

    pub fn broadcast_block(&mut self, block: Block) {
        self.block_queue.push_back(block);
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }
}
