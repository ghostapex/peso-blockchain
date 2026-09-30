use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::ledger::Ledger;
use crate::rpc_models::{RpcAccount, RpcBlock, RpcResponse, RpcStatus};
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct RpcState {
    pub ledger: Arc<Ledger>,
}

#[derive(Deserialize)]
pub struct TransferBody {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
    pub signature: String,
}

pub async fn start_rpc(state: RpcState) {
    let app = Router::new()
        .route("/status", get(status_handler))
        .route("/accounts/:id", get(account_handler))
        .route("/blocks/latest", get(latest_block_handler))
        .route("/tx", post(send_tx_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("PESO RPC running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn status_handler(State(state): State<RpcState>) -> Json<RpcResponse<RpcStatus>> {
    Json(RpcResponse {
        ok: true,
        data: RpcStatus {
            name: "PESO Chain".to_string(),
            status: "online".to_string(),
            height: state.ledger.height,
            epoch: 1,
        },
    })
}

async fn account_handler(State(state): State<RpcState>, id: axum::extract::Path<String>) -> Json<RpcResponse<RpcAccount>> {
    let balance = state.ledger.get_balance(&id);
    let account = state.ledger.get_account(&id).unwrap_or_default();

    Json(RpcResponse {
        ok: true,
        data: RpcAccount {
            id: id.clone(),
            balance,
            nonce: account.nonce,
        },
    })
}

async fn latest_block_handler() -> Json<RpcResponse<RpcBlock>> {
    Json(RpcResponse {
        ok: true,
        data: RpcBlock {
            index: 0,
            hash: "genesis".to_string(),
            proposer: "genesis".to_string(),
            tx_count: 0,
        },
    })
}

async fn send_tx_handler(State(_state): State<RpcState>, Json(_payload): Json<TransferBody>) -> Json<RpcResponse<String>> {
    Json(RpcResponse {
        ok: true,
        data: "transaction accepted".to_string(),
    })
}
