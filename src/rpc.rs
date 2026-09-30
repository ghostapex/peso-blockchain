use std::sync::Arc;

use axum::{extract::Path, routing::get, Json, Router};

use crate::ledger::Ledger;

#[derive(Clone)]
pub struct RpcState {
    pub ledger: Arc<Ledger>,
}

#[derive(serde::Serialize)]
pub struct StatusResponse {
    pub name: String,
    pub height: u64,
    pub online: bool,
}

#[derive(serde::Serialize)]
pub struct BalanceResponse {
    pub id: String,
    pub balance: u64,
}

pub async fn start_rpc(state: RpcState) {
    let app = Router::new()
        .route("/status", get(status))
        .route("/balance/:account", get(balance))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("PESO RPC running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        name: "PESO".to_string(),
        height: 0,
        online: true,
    })
}

async fn balance(Path(account): Path<String>, state: axum::extract::State<RpcState>) -> Json<BalanceResponse> {
    let balance = state.ledger.get_balance(&account);
    Json(BalanceResponse {
        id: account,
        balance,
    })
}
