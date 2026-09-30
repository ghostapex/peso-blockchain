use crate::transaction::Transaction;

pub fn schedule_by_account(txs: Vec<Transaction>) -> Vec<Vec<Transaction>> {
    let mut groups: Vec<Vec<Transaction>> = Vec::new();
    let mut current: Vec<Transaction> = Vec::new();

    for tx in txs {
        let conflict = current.iter().any(|c| {
            c.sender == tx.sender
                || c.receiver == tx.sender
                || c.sender == tx.receiver
                || c.receiver == tx.receiver
        });

        if conflict {
            if !current.is_empty() {
                groups.push(current);
                current = Vec::new();
            }
        }

        current.push(tx);
    }

    if !current.is_empty() {
        groups.push(current);
    }

    groups
}
