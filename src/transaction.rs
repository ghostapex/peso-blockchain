use serde::{Deserialize, Serialize};

use crate::types::{AccountId, Amount, Hash, Signature, hash_bytes};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub sender: AccountId,
    pub receiver: AccountId,
    pub amount: Amount,
    pub nonce: u64,
    pub signature: Signature,
    pub hash: Hash,
}

impl Transaction {
    pub fn new(sender: AccountId, receiver: AccountId, amount: Amount, nonce: u64) -> Self {
        let mut tx = Self {
            sender,
            receiver,
            amount,
            nonce,
            signature: String::new(),
            hash: String::new(),
        };

        tx.hash = tx.compute_hash();
        tx
    }

    pub fn payload(&self) -> Vec<u8> {
        let raw = format!("{}:{}:{}:{}", self.sender, self.receiver, self.amount, self.nonce);
        raw.into_bytes()
    }

    pub fn compute_hash(&self) -> Hash {
        hash_bytes(&self.payload())
    }

    pub fn sign(&mut self, wallet: &crate::wallet::Wallet) {
        self.signature = wallet.sign(&self.payload());
        self.hash = self.compute_hash();
    }

    pub fn verify(&self) -> bool {
        if self.signature.is_empty() {
            return false;
        }

        crate::wallet::Wallet::verify(&self.sender, &self.payload(), &self.signature)
    }
}
