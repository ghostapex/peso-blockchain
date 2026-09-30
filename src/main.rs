use peso_chain::bootstrap::BootstrapConfig;
use peso_chain::consensus::Consensus;
use peso_chain::ledger::Ledger;
use peso_chain::node::Node;
use peso_chain::staking::StakePool;
use peso_chain::transaction::Transaction;
use peso_chain::validator::Validator;
use peso_chain::wallet::Wallet;

fn main() {
    let bootstrap = BootstrapConfig::default();

    println!("=== PESO Bootstrap Network ===");
    println!("Bootstrap port: {}", bootstrap.bootstrap_port);

    for v in &bootstrap.validators {
        println!("Validator {} stake={} peer={} rpc={}", v.id, v.stake, v.peer, v.rpc_port);
    }

    let validators = vec![
        Validator::new("validator-1".to_string(), 100),
        Validator::new("validator-2".to_string(), 200),
        Validator::new("validator-3".to_string(), 300),
    ];

    let mut pool = StakePool::new();
    for validator in &validators {
        pool.add_stake(validator.id.clone(), validator.stake);
    }

    let leader = pool.choose_leader(123, &validators).unwrap();
    println!("\nSelected leader by stake: {}", leader.id);

    let mut consensus = Consensus::new(validators.clone());
    println!("Current leader: {}", consensus.current_leader().unwrap().id);

    let alice = Wallet::new();
    let bob = Wallet::new();
    let charlie = Wallet::new();

    let mut ledger1 = Ledger::new();
    let mut ledger2 = Ledger::new();
    let mut ledger3 = Ledger::new();

    for account in [&alice, &bob, &charlie] {
        ledger1.add_account(account.account_id.clone(), 1_000_000);
        ledger2.add_account(account.account_id.clone(), 1_000_000);
        ledger3.add_account(account.account_id.clone(), 1_000_000);
    }

    let mut node1 = Node::new("validator-1".to_string(), ledger1, validators.clone());
    let mut node2 = Node::new("validator-2".to_string(), ledger2, validators.clone());
    let mut node3 = Node::new("validator-3".to_string(), ledger3, validators.clone());

    node1.connect("validator-2".to_string());
    node1.connect("validator-3".to_string());
    node2.connect("validator-1".to_string());
    node2.connect("validator-3".to_string());
    node3.connect("validator-1".to_string());
    node3.connect("validator-2".to_string());

    let mut tx1 = Transaction::new(alice.account_id.clone(), bob.account_id.clone(), 250_000, 0);
    tx1.sign(&alice);

    let mut tx2 = Transaction::new(bob.account_id.clone(), charlie.account_id.clone(), 100_000, 0);
    tx2.sign(&bob);

    node1.submit_transaction(tx1.clone());
    node2.submit_transaction(tx1.clone());
    node3.submit_transaction(tx1.clone());

    node1.submit_transaction(tx2.clone());
    node2.submit_transaction(tx2.clone());
    node3.submit_transaction(tx2.clone());

    let block = node1.mine_block().unwrap();
    node2.sync_block(&block).unwrap();
    node3.sync_block(&block).unwrap();

    println!("\nBlock mined: {}", block.hash);
    println!("\nFinal state:");
    println!("Alice: node1={} node2={} node3={}", 
        node1.balance(&alice.account_id),
        node2.balance(&alice.account_id),
        node3.balance(&alice.account_id)
    );
    println!("Bob: node1={} node2={} node3={}", 
        node1.balance(&bob.account_id),
        node2.balance(&bob.account_id),
        node3.balance(&bob.account_id)
    );
    println!("\nPESO private testnet bootstrap complete.");
}
