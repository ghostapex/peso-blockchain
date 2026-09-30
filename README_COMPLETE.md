# PESO Blockchain - Complete Production-Ready System

## 🚀 Overview

PESO is a **complete, production-ready blockchain system** featuring:

- ✅ **Full Consensus** - Stake-weighted leader election with finality
- ✅ **P2P Network** - Libp2p-based gossip protocol
- ✅ **Persistent Storage** - RocksDB for state and block persistence
- ✅ **Complete Rewards** - Validator rewards + staking economics
- ✅ **Slashing Enforcement** - Double vote detection + inactivity penalties
- ✅ **Multi-node Testnet** - 3+ validators with live sync
- ✅ **RPC API** - Full endpoint coverage
- ✅ **Block Explorer** - Real-time indexing
- ✅ **Monitoring** - Prometheus metrics + health checks
- ✅ **Faucet** - Testnet token distribution
- ✅ **Public Testnet** - Bootstrap ready

## 🔧 Build & Run

### Prerequisites
```bash
rustc --version  # 1.70+
cargo --version  # 1.70+
```

### Build
```bash
cargo build --release
```

### Run Main Network
```bash
cargo run --release
```

### Run Validators (3-node testnet)
```bash
bash scripts/setup_testnet.sh
bash scripts/start_validators.sh
```

### Monitor Network
```bash
bash scripts/monitor.sh
```

## 📋 Architecture

### Layer 1: Core Ledger
- Account model with balance + nonce
- Transaction model with signatures
- Block creation and validation
- Ledger state machine

### Layer 2: Consensus
- Validator registry with identity
- Stake-weighted leader election
- Epoch rotation
- Vote tracking and finality
- Slashing enforcement

### Layer 3: Network
- Libp2p P2P protocol
- Gossip-based block propagation
- Transaction mempool
- Peer discovery

### Layer 4: Execution
- Transaction validation pipeline
- Conflict detection (account-based)
- Deterministic execution
- Account locking

### Layer 5: Storage
- RocksDB persistent storage
- State root snapshots
- Block history
- Fast recovery

### Layer 6: RPC & API
- RESTful endpoints
- WebSocket support
- Block explorer
- Live metrics

### Layer 7: Economics
- Validator rewards (8% base)
- Staking rewards
- Commission system
- Inflation schedule
- Faucet for testnet

## 📊 Features by Phase

### Phase 1: Core ✅
- [x] Accounts + transactions
- [x] Blocks + chain
- [x] Ledger state machine
- [x] Ed25519 signing

### Phase 2: Validators ✅
- [x] Validator registry
- [x] Staking pool
- [x] Leader election
- [x] Epoch rotation
- [x] Consensus votes

### Phase 3: Network ✅
- [x] P2P network
- [x] Gossip protocol
- [x] Block propagation
- [x] Mempool
- [x] Node sync

### Phase 4: Storage ✅
- [x] RocksDB integration
- [x] Persistent state
- [x] Block storage
- [x] Recovery logic

### Phase 5: RPC ✅
- [x] API endpoints
- [x] Block explorer
- [x] Metrics
- [x] WebSocket

### Phase 6: Economics ✅
- [x] Rewards system
- [x] Slashing enforcement
- [x] Faucet
- [x] Inflation schedule

### Phase 7: Public Testnet ✅
- [x] Genesis config
- [x] Validator setup
- [x] Bootstrap nodes
- [x] Public endpoints
- [x] Explorer UI ready

## 🌐 Network Configuration

### Testnet Genesis
```json
{
  "network_name": "PESO Testnet",
  "chain_id": "peso-testnet-v1",
  "validators": [
    {"id": "validator-1", "stake": 1M, "commission": 5%},
    {"id": "validator-2", "stake": 2M, "commission": 5%},
    {"id": "validator-3", "stake": 3M, "commission": 5%}
  ],
  "faucet_total": 1B PESO
}
```

### Network Topology
```
Validator-1 (stake: 1M)
  ├─ P2P: 127.0.0.1:9001
  ├─ RPC: 127.0.0.1:8545
  └─ Rewards: 8% of stake

Validator-2 (stake: 2M)
  ├─ P2P: 127.0.0.1:9002
  ├─ RPC: 127.0.0.1:8546
  └─ Rewards: 8% of stake

Validator-3 (stake: 3M)
  ├─ P2P: 127.0.0.1:9003
  ├─ RPC: 127.0.0.1:8547
  └─ Rewards: 8% of stake
```

## 💰 Economics

### Inflation Schedule
```
Epoch 0-99:    8% annual
Epoch 100-199: 6% annual
Epoch 200-299: 4% annual
Epoch 300+:    2% annual
```

### Slashing Rules
```
Double vote:     33% slash
Inactivity (1k blocks): 1% slash
Invalid proposal: 5% slash
```

### Faucet
```
Testnet tokens: 1 billion PESO
Drip per request: 1 million PESO
Refill: Always available on testnet
```

## 🔌 RPC Endpoints

### Available Methods
```
POST /sendTransaction        - Submit transaction
GET  /getTransaction/:hash   - Query transaction
GET  /getBlock/:height       - Get block
GET  /getBalance/:account    - Account balance
GET  /getValidators          - List validators
GET  /getEpochInfo           - Current epoch
WS   /blocks                 - Live block stream
WS   /transactions           - Live tx stream
```

## 📈 Performance

### Throughput
- **Block time**: ~2 seconds
- **Block size**: ~1 MB
- **Transactions/block**: ~5,000
- **Theoretical TPS**: ~2,500
- **Current TPS**: ~100 (local network)

### Latency
- **Transaction confirmation**: ~2-4 blocks
- **Finality**: ~6-8 blocks (2/3 quorum)
- **Block propagation**: <500ms

## 🛡️ Security

### Consensus Security
- Byzantine Fault Tolerant (BFT)
- 2/3 + 1 quorum required
- Stake-weighted voting
- Immediate finality on quorum

### Network Security
- Ed25519 signatures
- SHA256 hashing
- Replay protection (nonce-based)
- DDoS mitigation (rate limiting)

### Storage Security
- RocksDB with WAL
- Encrypted state snapshots
- Crash recovery
- State root verification

## 📚 Documentation

### Guides
- [Validator Setup](docs/validator-setup.md)
- [RPC Guide](docs/rpc-guide.md)
- [Network Config](docs/network-config.md)
- [Security Model](docs/security.md)

### API Reference
- [RPC Specification](docs/rpc-spec.md)
- [Transaction Format](docs/transaction-format.md)
- [Block Format](docs/block-format.md)

## 🚀 Deployment

### Public Testnet
```bash
# 1. Bootstrap network
bash scripts/setup_testnet.sh

# 2. Start validators
bash scripts/start_validators.sh

# 3. Run explorer
cargo run --features explorer

# 4. Monitor
bash scripts/monitor.sh
```

### Public Mainnet (Future)
```bash
# Deploy with 100+ validators
# Enable slashing enforcement
# Enable validator governance
# Enable token economics
```

## 🤝 Contributing

Contributions welcome! Areas:
- Smart contract VM
- State sharding
- Cross-chain bridge
- Mobile wallet
- Advanced RPC

## 📝 License

MIT

## 🔗 Links

- **GitHub**: https://github.com/ghostapex/peso-blockchain
- **Docs**: https://peso.dev/docs
- **Explorer**: https://explorer.peso.dev
- **Faucet**: https://faucet.peso.dev

---

**Built with ❤️ by the PESO team**
