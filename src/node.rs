use crate::block::Block;
use crate::consensus::Consensus;
use crate::ledger::Ledger;
use crate::mempool::Mempool;
use crate::transaction::Transaction;
use crate::validator::Validator;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub ledger: Ledger,
    pub mempool: Mempool,
    pub peers: Vec<String>,
    pub consensus: Consensus,
}

impl Node {
    pub fn new(id: String, ledger: Ledger, validators: Vec<Validator>) -> Self {
        Self {
            id,
            ledger,
            mempool: Mempool::new(),
            peers: Vec::new(),
            consensus: Consensus::new(validators),
        }
    }

    pub fn connect(&mut self, peer: String) {
        if !self.peers.contains(&peer) {
            self.peers.push(peer);
        }
    }

    pub fn submit_transaction(&mut self, tx: Transaction) {
        self.mempool.push(tx.clone());
    }

    pub fn mine_block(&mut self) -> Result<Block, String> {
        let txs = self.mempool.pop_batch(10);
        if txs.is_empty() {
            return Err("mempool empty".to_string());
        }

        let leader = self.consensus.current_leader().ok_or_else(|| "no leader".to_string())?;

        let block = Block::new(
            self.ledger.height + 1,
            self.ledger.last_hash.clone(),
            format!("poh-{}", self.ledger.height + 1),
            leader.id.clone(),
            txs.clone(),
        );

        for tx in &txs {
            self.ledger.apply_transaction(tx)?;
        }

        self.ledger.last_hash = block.hash.clone();
        self.ledger.height += 1;

        Ok(block)
    }

    pub fn sync_block(&mut self, block: &Block) -> Result<(), String> {
        self.ledger.apply_block(block)?;
        Ok(())
    }

    pub fn balance(&self, account_id: &str) -> u64 {
        self.ledger.get_balance(account_id)
    }
}
