use peso_chain::consensus::Validator;
use peso_chain::ledger::Ledger;
use peso_chain::node::Node;
use peso_chain::transaction::Transaction;
use peso_chain::wallet::Wallet;
use std::sync::{Arc, Mutex};

fn main() {
    println!("\n=== PESO Chain Multi-Node Prototype ===");
    println!("Building 3-node local blockchain network...\n");

    // Create ledgers for each node
    let mut ledger_a = Ledger::new();
    let mut ledger_b = Ledger::new();
    let mut ledger_c = Ledger::new();

    // Create user wallets
    let alice = Wallet::new();
    let bob = Wallet::new();
    let charlie = Wallet::new();

    // Create validator wallets
    let validator_1 = Wallet::new();
    let validator_2 = Wallet::new();
    let validator_3 = Wallet::new();

    // Initialize ledgers with accounts
    ledger_a.add_account(alice.account_id.clone(), 1_000_000);
    ledger_a.add_account(bob.account_id.clone(), 500_000);
    ledger_a.add_account(charlie.account_id.clone(), 750_000);

    ledger_b.add_account(alice.account_id.clone(), 1_000_000);
    ledger_b.add_account(bob.account_id.clone(), 500_000);
    ledger_b.add_account(charlie.account_id.clone(), 750_000);

    ledger_c.add_account(alice.account_id.clone(), 1_000_000);
    ledger_c.add_account(bob.account_id.clone(), 500_000);
    ledger_c.add_account(charlie.account_id.clone(), 750_000);

    // Create validator set
    let validators = vec![
        Validator {
            id: validator_1.account_id.clone(),
            stake: 100,
        },
        Validator {
            id: validator_2.account_id.clone(),
            stake: 100,
        },
        Validator {
            id: validator_3.account_id.clone(),
            stake: 100,
        },
    ];

    // Create nodes
    let mut node1 = Node::new("node-1".to_string(), ledger_a, validators.clone());
    let mut node2 = Node::new("node-2".to_string(), ledger_b, validators.clone());
    let mut node3 = Node::new("node-3".to_string(), ledger_c, validators.clone());

    // Connect nodes
    println!("Connecting nodes to network...");
    node1.network.connect("node-2".to_string());
    node1.network.connect("node-3".to_string());
    node2.network.connect("node-1".to_string());
    node2.network.connect("node-3".to_string());
    node3.network.connect("node-1".to_string());
    node3.network.connect("node-2".to_string());
    println!("✓ Network established\n");

    // Create transactions
    println!("=== Transaction Block 1 ===");
    let mut tx1 = Transaction::new(
        alice.account_id.clone(),
        bob.account_id.clone(),
        250_000,
        0,
    );
    tx1.sign(&alice);
    println!("TX1: Alice -> Bob [250,000 PESO]");

    let mut tx2 = Transaction::new(
        bob.account_id.clone(),
        charlie.account_id.clone(),
        100_000,
        0,
    );
    tx2.sign(&bob);
    println!("TX2: Bob -> Charlie [100,000 PESO]\n");

    // Submit transactions to all nodes
    println!("Propagating transactions across network...");
    node1.submit_transaction(tx1.clone()).unwrap();
    node2.submit_transaction(tx1.clone()).unwrap();
    node3.submit_transaction(tx1.clone()).unwrap();

    node1.submit_transaction(tx2.clone()).unwrap();
    node2.submit_transaction(tx2.clone()).unwrap();
    node3.submit_transaction(tx2.clone()).unwrap();
    println!("✓ Transactions broadcast to all nodes\n");

    // Node 1 (leader) mines block
    println!("=== Block Production ===");
    println!("Node-1 selected as leader");
    let block1 = node1.mine_block().unwrap();
    println!("✓ Block #{} produced", block1.index);
    println!("  Hash: {}", &block1.hash[..16]);
    println!("  Transactions: {}\n", block1.transactions.len());

    // Sync block to other nodes
    println!("=== State Synchronization ===");
    node2.ledger.apply_block(&block1).unwrap();
    node3.ledger.apply_block(&block1).unwrap();
    println!("✓ Block synchronized to node-2 and node-3\n");

    // Create second block
    println!("=== Transaction Block 2 ===");
    let mut tx3 = Transaction::new(
        charlie.account_id.clone(),
        alice.account_id.clone(),
        50_000,
        0,
    );
    tx3.sign(&charlie);
    println!("TX3: Charlie -> Alice [50,000 PESO]\n");

    node1.submit_transaction(tx3.clone()).unwrap();
    node2.submit_transaction(tx3.clone()).unwrap();
    node3.submit_transaction(tx3.clone()).unwrap();

    // Node 2 mines next block (round-robin leader)
    println!("=== Block Production ===");
    println!("Node-2 selected as leader (round-robin)");
    let block2 = node2.mine_block().unwrap();
    println!("✓ Block #{} produced", block2.index);
    println!("  Hash: {}", &block2.hash[..16]);
    println!("  Transactions: {}\n", block2.transactions.len());

    // Sync to other nodes
    node1.ledger.apply_block(&block2).unwrap();
    node3.ledger.apply_block(&block2).unwrap();
    println!("✓ Block synchronized to node-1 and node-3\n");

    // Display final state
    println!("=== Final Ledger State ===");
    println!("Node-1 Ledger (Height: {}):", node1.ledger.height);
    println!("  Alice:   {:>10} PESO", node1.balance(&alice.account_id));
    println!("  Bob:     {:>10} PESO", node1.balance(&bob.account_id));
    println!("  Charlie: {:>10} PESO", node1.balance(&charlie.account_id));
    println!();

    println!("Node-2 Ledger (Height: {}):", node2.ledger.height);
    println!("  Alice:   {:>10} PESO", node2.balance(&alice.account_id));
    println!("  Bob:     {:>10} PESO", node2.balance(&bob.account_id));
    println!("  Charlie: {:>10} PESO", node2.balance(&charlie.account_id));
    println!();

    println!("Node-3 Ledger (Height: {}):", node3.ledger.height);
    println!("  Alice:   {:>10} PESO", node3.balance(&alice.account_id));
    println!("  Bob:     {:>10} PESO", node3.balance(&bob.account_id));
    println!("  Charlie: {:>10} PESO", node3.balance(&charlie.account_id));
    println!();

    // Verify consensus
    let node1_alice = node1.balance(&alice.account_id);
    let node2_alice = node2.balance(&alice.account_id);
    let node3_alice = node3.balance(&alice.account_id);

    if node1_alice == node2_alice && node2_alice == node3_alice {
        println!("✓ CONSENSUS ACHIEVED - All nodes agree on state");
    } else {
        println!("✗ CONSENSUS FAILED - Nodes have different state");
    }

    println!("\n=== PESO Network Test Complete ===");
}
