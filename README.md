# PESO Chain

PESO is a Solana-inspired blockchain prototype built in Rust. This project demonstrates a realistic foundation for a local blockchain network with:

- **Wallet & Signatures**: Ed25519 cryptographic signing
- **Transaction Validation**: Proper signature, nonce, and balance verification
- **Account Ledger**: Account-based state model with balances and nonces
- **Block Creation**: Merkle-hashed blocks with transaction sets
- **Multi-Node Consensus**: Simple round-robin leader selection and validator quorum
- **Network Propagation**: Transaction and block gossip across nodes
- **State Synchronization**: Ledger state consensus across validators

## Goal

PESO is the foundation for a future blockchain network: local prototype → multi-node testnet → public network.

## Running the Prototype

```bash
cargo build --release
cargo run --release
```

### Expected Output

The demo runs 3 local nodes and simulates:

1. Network setup: Nodes connect to each other
2. Transaction propagation: Transactions broadcast across all nodes
3. Block production: Leader node mines blocks
4. State synchronization: Blocks sync to other nodes
5. Final consensus: All nodes agree on account balances

## Project Structure

- `src/account.rs` — Account model (balance, nonce)
- `src/transaction.rs` — Signed transactions (Ed25519)
- `src/block.rs` — Block structure with SHA256 hashing
- `src/ledger.rs` — State updates and transaction application
- `src/node.rs` — Node implementation with block mining
- `src/consensus.rs` — Validator set and quorum logic
- `src/network.rs` — Network communication primitives
- `src/wallet.rs` — Cryptographic wallet (keypair generation)
- `src/types.rs` — Common types and hash utilities
- `src/main.rs` — 3-node local network demo

## How It Works

### Consensus Model

- **Leader Selection**: Round-robin validator rotation
- **Block Production**: Leader collects transactions from mempool and creates a block
- **Validation**: All transactions must have valid signatures, sufficient balance, and correct nonce
- **State Sync**: Other nodes receive and apply blocks to their local ledger
- **Quorum**: Block is final once majority of validators agree

### Transaction Flow

1. User creates a transaction (sender, receiver, amount)
2. User signs the transaction with their private key
3. Transaction is submitted to the leader node
4. Node adds transaction to mempool
5. Leader includes transaction in the next block
6. Block is validated and applied to all node ledgers
7. Balances update across the network

## Notes

This is a prototype, not production-grade. It is designed for learning and as a foundation for extension.

## Next Steps

- Multi-epoch leader rotation
- Stake-weighted voting
- Slashing for validator misbehavior
- RPC API for external clients
- Web-based explorer
- Persistent storage (RocksDB)
- Public testnet bootstrap
