use crate::block::Block;
use crate::consensus::{Consensus, Validator};
use crate::ledger::Ledger;
use crate::network::Network;
use crate::transaction::Transaction;
use crate::types::AccountId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: AccountId,
    pub ledger: Ledger,
    pub mempool: Vec<Transaction>,
    pub network: Network,
    pub consensus: Consensus,
    pub blocks: Vec<Block>,
}

impl Node {
    pub fn new(id: AccountId, ledger: Ledger, validators: Vec<Validator>) -> Self {
        let mut network = Network::new();
        for validator in &validators {
            network.connect(validator.id.clone());
        }

        Self {
            id,
            ledger,
            mempool: Vec::new(),
            network,
            consensus: Consensus::new(validators),
            blocks: Vec::new(),
        }
    }

    pub fn submit_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        if !tx.verify() {
            return Err("transaction signature invalid".to_string());
        }

        self.mempool.push(tx.clone());
        self.network.broadcast_tx(tx);

        Ok(())
    }

    pub fn mine_block(&mut self) -> Result<Block, String> {
        if self.mempool.is_empty() {
            return Err("mempool is empty".to_string());
        }

        let transactions = std::mem::take(&mut self.mempool);

        let block = Block::new(
            self.ledger.height + 1,
            self.ledger.last_hash.clone(),
            self.id.clone(),
            transactions.clone(),
        );

        if !self.consensus.validate_block(&block) {
            return Err("block rejected by consensus".to_string());
        }

        for tx in &transactions {
            self.ledger.apply_transaction(tx)?;
        }

        self.ledger.last_hash = block.hash.clone();
        self.ledger.height += 1;
        self.blocks.push(block.clone());
        self.network.broadcast_block(block.clone());

        Ok(block)
    }

    pub fn balance(&self, account_id: &str) -> u64 {
        self.ledger.get_balance(account_id)
    }

    pub fn sync_block(&mut self, block: &Block) -> Result<(), String> {
        if block.index != self.ledger.height + 1 {
            return Err("block index mismatch".to_string());
        }

        self.ledger.apply_block(block)?;
        self.blocks.push(block.clone());
        Ok(())
    }

    pub fn get_chain_height(&self) -> u64 {
        self.ledger.height
    }
}
