use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExplorerEntry {
    pub hash: String,
    pub proposer: String,
    pub tx_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Explorer {
    pub blocks: Vec<ExplorerEntry>,
}

impl Explorer {
    pub fn push(&mut self, hash: String, proposer: String, tx_count: usize) {
        self.blocks.push(ExplorerEntry {
            hash,
            proposer,
            tx_count,
        });
    }
}
