use std::sync::Arc;

use peso_chain::config::Config;
use peso_chain::ledger::Ledger;
use peso_chain::rpc::{start_rpc, RpcState};
use peso_chain::transaction::Transaction;
use peso_chain::validator::Validator;
use peso_chain::wallet::Wallet;

#[tokio::main]
async fn main() {
    let config = Config::default();

    let alice = Wallet::new();
    let bob = Wallet::new();
    let eve = Wallet::new();

    let v1 = Wallet::new();
    let v2 = Wallet::new();
    let v3 = Wallet::new();

    let validators = vec![
        Validator::new(v1.account_id.clone(), 100),
        Validator::new(v2.account_id.clone(), 200),
        Validator::new(v3.account_id.clone(), 300),
    ];

    let mut ledger = Ledger::new();
    for account in [&alice, &bob, &eve] {
        ledger.add_account(account.account_id.clone(), 1_000_000);
    }

    let mut tx = Transaction::new(alice.account_id.clone(), bob.account_id.clone(), 250_000, 0);
    tx.sign(&alice);
    ledger.apply_transaction(&tx).unwrap();

    println!("PESO testnet bootstrap started.");
    println!("Network: {}", config.network_name);
    println!("Validators: {:?}", validators.iter().map(|v| v.id.clone()).collect::<Vec<_>>());
    println!("Alice balance: {}", ledger.get_balance(&alice.account_id));
    println!("Bob balance: {}", ledger.get_balance(&bob.account_id));
    println!("RPC ready on {}:{}", config.rpc_host, config.rpc_port);

    let state = RpcState {
        ledger: Arc::new(ledger),
    };

    start_rpc(state).await;
}
