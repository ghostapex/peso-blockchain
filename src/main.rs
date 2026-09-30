use solana_like_chain::consensus::Validator;
use solana_like_chain::ledger::Ledger;
use solana_like_chain::node::Node;
use solana_like_chain::transaction::Transaction;
use solana_like_chain::wallet::Wallet;

fn main() {
    let mut ledger = Ledger::new();

    let alice = Wallet::new();
    let bob = Wallet::new();
    let validator_1 = Wallet::new();
    let validator_2 = Wallet::new();
    let validator_3 = Wallet::new();

    ledger.add_account(alice.account_id.clone(), 1000);
    ledger.add_account(bob.account_id.clone(), 500);

    let validators = vec![
        Validator { id: validator_1.account_id.clone(), stake: 100 },
        Validator { id: validator_2.account_id.clone(), stake: 100 },
        Validator { id: validator_3.account_id.clone(), stake: 100 },
    ];

    let mut node = Node::new("peso-node-1".to_string(), ledger, validators);

    let mut transfer = Transaction::new(alice.account_id.clone(), bob.account_id.clone(), 250, 0);
    transfer.sign(&alice);

    node.submit_transaction(transfer).unwrap();
    let block = node.mine_block().unwrap();

    println!("PESO block produced: {}", block.hash);
    println!("Alice balance: {}", node.balance(&alice.account_id));
    println!("Bob balance: {}", node.balance(&bob.account_id));
}
