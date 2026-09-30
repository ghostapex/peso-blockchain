use serde::{Deserialize, Serialize};

use crate::types::{AccountId, Amount, Hash};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcStatus {
    pub name: String,
    pub status: String,
    pub height: u64,
    pub epoch: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcAccount {
    pub id: AccountId,
    pub balance: Amount,
    pub nonce: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcBlock {
    pub index: u64,
    pub hash: Hash,
    pub proposer: AccountId,
    pub tx_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcResponse<T> {
    pub ok: bool,
    pub data: T,
}
