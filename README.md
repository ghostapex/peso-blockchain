# PESO Blockchain Prototype

PESO is a serious blockchain prototype inspired by Solana-like architecture concepts:

## Features

- **Proof-of-History-inspired** hash chain for ordering
- **Validator staking** with stake-weighted leader election
- **Epoch rotation** and leader selection by stake
- **Multi-node local network** with block propagation and sync
- **Transaction scheduler** with account-based parallelization
- **Persistent state** with state root hashing
- **RPC-ready** architecture
- **Explorer scaffolding** for chain indexing
- **Private testnet bootstrap** configuration

## Architecture

### Phase 1: Core Ledger
- Account model
- Transactions with signatures
- Block creation and validation
- Ledger state management

### Phase 2: Validator Network
- Validator registry
- Staking pool with stake-weighted selection
- Leader election by stake
- Epoch rotation
- Consensus model with quorum

### Phase 3: Multi-node Network
- Transaction mempool
- Network peer connections
- Block sync between nodes
- Ledger replication

### Phase 4: Infrastructure
- Storage layer
- State root calculation
- RPC endpoints
- Explorer data model

### Phase 5: Persistence
- File-based storage
- State snapshots
- Bootstrap configuration

### Phase 6: Bootstrap
- Private testnet configuration
- Multi-validator launch
- Network initialization

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

## Run testnet bootstrap script

```bash
bash scripts/start_testnet.sh
```

## Configuration

- `config/genesis.json` - network genesis configuration
- `config/validators.json` - validator node configuration

## Project Structure

```
src/
├── main.rs           # bootstrap entry point
├── lib.rs            # module exports
├── account.rs        # account state model
├── block.rs          # block structure
├── transaction.rs    # transaction model
├── ledger.rs         # ledger state machine
├── wallet.rs         # signing wallets
├── validator.rs      # validator registry
├── consensus.rs      # consensus logic
├── staking.rs        # stake pool
├── slashing.rs       # slashing events
├── epoch.rs          # epoch tracking
├── mempool.rs        # transaction pool
├── network.rs        # network peers
├── node.rs           # validator node
├── storage.rs        # persistent storage
├── state.rs          # state root
├── scheduler.rs      # transaction scheduling
├── tx_pool.rs        # transaction pooling
├── rpc.rs            # RPC API
├── explorer.rs       # block explorer
├── bootstrap.rs      # bootstrap configuration
├── testnet.rs        # testnet config
├── config.rs         # network config
├── types.rs          # shared types
config/
├── genesis.json      # genesis config
├── validators.json   # validator list
scripts/
├── start_testnet.sh  # testnet launcher
```

## Network Model

1. **Genesis**: Bootstrap with 3 validators with different stakes (100, 200, 300)
2. **Leader Election**: Selected by stake-weighted random from validator set
3. **Block Production**: Leader proposes blocks from mempool
4. **Propagation**: Blocks broadcast to peers and synced
5. **Finality**: Quorum agreement (2/3 + 1) on canonical chain
6. **Epoch Rotation**: Leader rotates through validator set each epoch

## Security Model

- Ed25519 transaction signatures
- SHA256 block hashing
- Stake-weighted consensus
- Slashing model for invalid blocks
- Nonce-based replay protection
- Account-based conflict detection

## Limitations

This is a prototype and **NOT** production-ready:
- No network encryption
- No persistent validator identity
- No validator rotation mechanics
- No slashing enforcement
- No smart contract runtime
- Limited to local testnet
- No finality guarantees

## Next Steps

To extend PESO toward a real network:

1. **Network Layer**: Add proper P2P gossip protocol
2. **Consensus**: Implement full BFT with votes and finality
3. **Storage**: Integrate RocksDB for persistence
4. **Runtime**: Add smart contract VM or deterministic execution
5. **Security**: Audits and formal verification
6. **Tokenomics**: Define token economics and governance
7. **Public Testnet**: Bootstrap public validator network

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy

# Run demo
cargo run
```

## References

- Solana: https://solana.com
- Proof of History: https://medium.com/solana-labs
- Byzantine Fault Tolerance: https://en.wikipedia.org/wiki/Byzantine_fault_tolerance

## License

MIT
