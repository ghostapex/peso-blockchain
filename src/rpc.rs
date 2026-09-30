use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct RpcStatus {
    pub name: &'static str,
    pub status: &'static str,
}

pub async fn start_rpc() {
    let app = Router::new().route("/status", get(status));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("PESO RPC running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn status() -> Json<RpcStatus> {
    Json(RpcStatus {
        name: "PESO",
        status: "online",
    })
}
