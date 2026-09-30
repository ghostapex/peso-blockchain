# PESO Blockchain Architecture

## Overview

PESO is a prototype blockchain system with the following design:

```
┌─────────────────────────────────────────────┐
│         Network Bootstrap Layer              │
│  (validators.json, genesis.json, bootstrap) │
└─────────┬───────────────────────────────────┘
          │
┌─────────▼───────────────────────────────────┐
│      Consensus & Validator Layer             │
│  (validator, staking, epochs, slashing)      │
└─────────┬───────────────────────────────────┘
          │
┌─────────▼───────────────────────────────────┐
│        Node & Network Layer                  │
│  (nodes, peers, mempool, block propagation)  │
└─────────┬───────────────────────────────────┘
          │
┌─────────▼───────────────────────────────────┐
│      Ledger & Transaction Layer              │
│  (accounts, transactions, blocks, execution) │
└─────────┬───────────────────────────────────┘
          │
┌─────────▼───────────────────────────────────┐
│        Storage & State Layer                 │
│  (state root, snapshots, persistence)       │
└─────────┬───────────────────────────────────┘
          │
┌─────────▼───────────────────────────────────┐
│     RPC & Explorer Layer                     │
│  (API endpoints, block explorer, indexing)  │
└─────────────────────────────────────────────┘
```

## Core Components

### 1. Wallet & Signing (wallet.rs)
- Ed25519 keypair generation
- Message signing and verification
- Account ID derivation from public key

### 2. Account Model (account.rs, ledger.rs)
- Account state: balance, nonce
- Ledger tracks all accounts
- State transitions via transactions
- Account ownership via public key

### 3. Transaction Model (transaction.rs)
- Sender, receiver, amount, nonce
- Ed25519 signature
- Signature verification
- Replay protection via nonce

### 4. Block Model (block.rs)
- Index, hash, previous hash
- PoH hash (ordering proof)
- Transaction list
- Proposer (validator)

### 5. Validator & Consensus (validator.rs, consensus.rs, staking.rs)
- Validator set with stake amounts
- Leader election by stake-weighted selection
- Epoch rotation
- Vote tracking
- Quorum calculation (2/3 + 1)

### 6. Slashing (slashing.rs)
- Track slash events
- Validator penalties for invalid behavior
- Epoch-based tracking

### 7. Network (network.rs, node.rs, mempool.rs)
- Peer connections
- Transaction broadcasting
- Block propagation
- Mempool for pending transactions

### 8. Execution (scheduler.rs)
- Parallel transaction scheduling
- Account-based conflict detection
- Grouped execution

### 9. State Management (state.rs)
- State root calculation
- Account state snapshots
- Deterministic state hashing

### 10. Storage (storage.rs)
- File-based persistence
- JSON serialization
- Load/save operations

### 11. RPC (rpc.rs)
- HTTP endpoints
- `/status` - network status
- `/balance/:account` - account balance
- `/block/latest` - latest block

### 12. Explorer (explorer.rs)
- Block entry tracking
- Block history
- Transaction count per block

## Data Flow

### Transaction Flow
```
Wallet.sign(tx)
  ↓
Node.submit_transaction(tx)
  ↓
Mempool.push(tx)
  ↓
Network.broadcast_tx(tx)
  ↓
Peer nodes receive in mempool
  ↓
Leader.mine_block(mempool)
  ↓
Scheduler.schedule_by_account(txs)
  ↓
Executor.apply_transactions()
  ↓
Ledger.apply_transaction(tx)
  ↓
StateRoot.compute()
  ↓
Block.hash()
  ↓
Network.propagate_block(block)
  ↓
Peer nodes sync block
  ↓
Consensus.validate_block()
  ↓
Ledger.apply_block(block)
  ↓
Explorer.index_block()
```

### Leader Election Flow
```
Epoch starts
  ↓
StakePool.choose_leader(seed, validators)
  ↓
Stake-weighted random selection
  ↓
Validator selected as leader
  ↓
Leader mines block
  ↓
Epoch ends
  ↓
Consensus.rotate_leader()
  ↓
Next leader selected
```

## Consensus Model

### Simplified BFT
1. Leader selected by stake weight
2. Leader proposes block from mempool
3. Validators validate block
4. Validators vote on block
5. If quorum (2/3 + 1) votes, block is final
6. Block is broadcast and replicated
7. Epoch rotates to next leader

### Finality
- Canonical chain = longest valid chain with quorum
- Once quorum reached, block is final
- Forks resolved by stake weight + height

### Slashing
- Double vote in same epoch → slash
- Invalid block proposer → slash
- Inactivity during epoch → slash

## Bootstrap Process

1. Load `genesis.json` with validator set
2. Load `validators.json` with peer information
3. Initialize StakePool from validator stakes
4. Create Node for each validator
5. Connect peers in network topology
6. Start RPC server
7. Mempool ready for transactions
8. Leader election begins

## State Snapshots

```json
{
  "root_hash": "<state merkle root>",
  "accounts": {
    "<account_id>": {
      "id": "<account_id>",
      "balance": 1000000,
      "nonce": 0
    }
  }
}
```

## Persistence Strategy

1. **Blocks**: Stored as JSON files
2. **State**: State root snapshots on disk
3. **Ledger**: Account state serialized
4. **Config**: JSON configuration files

For production, upgrade to RocksDB or similar.

## Scalability Notes

### Current Limits
- Local network only (no real P2P)
- ~10 TPS (10 transactions per block, 1 block per epoch)
- In-memory ledger
- File-based storage

### Improvements Needed
- Proper P2P network with gossip
- RocksDB for persistent storage
- Account-based parallelization
- Pipelined consensus
- State sharding

## Security Assumptions

1. **Honest Majority Assumption**: >2/3 stake is honest
2. **Synchrony Assumption**: Network delays bounded
3. **Cryptographic Assumptions**: SHA256, Ed25519 secure
4. **Time Assumption**: Clocks roughly synchronized
5. **Storage Assumption**: Persistent storage is reliable

## Future Enhancements

### Near Term
- [ ] Real P2P network (libp2p)
- [ ] RocksDB persistence
- [ ] Validator identity keys
- [ ] Slashing enforcement
- [ ] Smart contract VM

### Medium Term
- [ ] Public testnet bootstrap
- [ ] Staking rewards
- [ ] Validator governance
- [ ] Cross-shard transactions
- [ ] Light client protocol

### Long Term
- [ ] Production mainnet
- [ ] Audited security
- [ ] Formal verification
- [ ] DeFi ecosystem
- [ ] Scale to 65k TPS

## References

- Solana: https://solana.com/whitepaper
- BFT Consensus: https://pmg.csail.mit.edu/papers/osdi99.pdf
- PoH: https://solana.com/solana-whitepaper.pdf
