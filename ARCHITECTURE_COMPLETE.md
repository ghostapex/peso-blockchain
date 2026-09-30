# PESO Complete Architecture Document

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    PESO Blockchain Network                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│  │  Validator Node  │  │  Validator Node  │  │  Validator Node  │
│  │   (Leader)       │  │                  │  │                  │
│  │   Stake: 1M      │  │   Stake: 2M      │  │   Stake: 3M      │
│  ├──────────────────┤  ├──────────────────┤  ├──────────────────┤
│  │ RPC: 8545        │  │ RPC: 8546        │  │ RPC: 8547        │
│  │ P2P: 9001        │  │ P2P: 9002        │  │ P2P: 9003        │
│  └────────┬─────────┘  └────────┬─────────┘  └────────┬─────────┘
│           │                     │                     │
│           └─────────────────────┼─────────────────────┘
│                                 │
│                        ┌────────▼────────┐
│                        │   P2P Gossip    │
│                        │   Network       │
│                        └────────┬────────┘
│                                 │
│        ┌────────────────────────┼────────────────────────┐
│        │                        │                        │
│  ┌─────▼────────┐        ┌──────▼──────┐        ┌──────▼──────┐
│  │   Mempool    │        │   Ledger    │        │  RocksDB    │
│  │              │        │   State     │        │  Storage    │
│  └──────────────┘        └─────────────┘        └─────────────┘
│        │
│        ├─ Transactions
│        └─ Blocks
│
└─────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. Validator Nodes

Each validator is a full node running:
- Consensus engine
- Transaction validator
- Block producer (when leader)
- RPC server
- P2P listener

#### Validator Lifecycle
```
Start → Genesis → Join Network → Wait for Slot → Produce Block → Rotate → Repeat
```

### 2. P2P Network

Uses Libp2p for:
- Peer discovery
- Block gossip
- Transaction propagation
- Validator health checks

#### Message Types
```
Peer Discovery → Connected Peer → Block Announcement
                                 → Transaction Announcement
                                 → Validator Status
                                 → Vote Message
```

### 3. Consensus Engine

#### Leader Selection
```rust
stake_weighted_random(epoch_seed, validators)
  → Select validator with probability = stake / total_stake
```

#### Block Validation
```
Leader → Create Block → Broadcast → Validators Receive
                                  → Validate Transactions
                                  → Validate Block Structure
                                  → Vote on Block
                                  → Accumulate Votes
                                  → 2/3 Quorum Reached
                                  → Block Finalized
```

### 4. Transaction Pipeline

```
User Creates TX
  ↓
Client Signs (Ed25519)
  ↓
Send to Mempool
  ↓
P2P Gossip to Validators
  ↓
Each Validator:
  - Validate Signature
  - Check Account Exists
  - Check Balance >= Amount
  - Check Nonce Sequential
  - Add to Mempool
  ↓
Leader:
  - Select Transactions from Mempool
  - Detect Conflicts (account-based)
  - Order by Account (parallelizable)
  - Execute Transactions
  - Update State Root
  - Create Block
  ↓
Broadcast Block
  ↓
Validators:
  - Validate Block
  - Validate Transactions
  - Vote on Block
  ↓
Finality (2/3 votes)
  ↓
Update Canonical Chain
```

### 5. Reward Distribution

#### Per Epoch
```
Total Inflation = Total Stake × Inflation Rate

For Each Validator:
  Base Reward = Validator Stake × Inflation Rate
  Commission Reward = (Delegated Stake × Inflation Rate) × Commission %
  Validator Total = Base Reward + Commission Reward

For Each Delegator:
  Share = (Delegated Amount / Total Delegated) × Available Rewards
```

#### Inflation Schedule
```
Epoch 0-99:    8% → Total: 480M PESO (per epoch: 6M)
Epoch 100-199: 6% → Total: 360M PESO (per epoch: 4.5M)
Epoch 200-299: 4% → Total: 240M PESO (per epoch: 3M)
Epoch 300+:    2% → Total: 120M PESO (per epoch: 1.5M)
```

### 6. Slashing Mechanism

#### Double Vote Slash
```
If Validator Votes on 2 Different Blocks in Same Slot:
  Slash Amount = Stake × 33%
  Status = Jailed for Recovery Period
```

#### Inactivity Slash
```
If Validator Misses 1000 Consecutive Slots:
  Slash Amount = Stake × 1%
  Status = Inactive (no rewards)
```

#### Invalid Block Proposal
```
If Leader Proposes Invalid Block:
  Slash Amount = Stake × 5%
  Status = Jailed
```

### 7. Storage Architecture

#### RocksDB Schema
```
Key Prefix         | Value Type              | Example
─────────────────────────────────────────────────────
block:{height}     | Serialized Block        | block:1000
account:{id}       | Account State           | account:alice
tx:{hash}          | Transaction             | tx:0x123abc
state:{root}       | State Root              | state:0xfff
epoch:{num}        | Epoch Info              | epoch:10
validator:{id}     | Validator Info          | validator:val-1
```

#### State Snapshots
```
Every 100 Blocks:
  - Save current state root
  - Export all accounts
  - Store in RocksDB with epoch marker
  - Enable fast recovery
```

### 8. RPC API

#### Transaction Methods
```
POST /rpc
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "sendTransaction",
  "params": [
    {
      "sender": "alice",
      "receiver": "bob",
      "amount": 1000000,
      "nonce": 0,
      "signature": "0x..."
    }
  ]
}
```

#### Query Methods
```
GET /balance/:account
GET /block/:height
GET /validators
GET /epoch/current
GET /tx/:hash
```

#### WebSocket Streams
```
WS /stream/blocks      → Live block events
WS /stream/transactions → Live tx events
WS /stream/slots       → Slot progression
WS /stream/validators  → Validator updates
```

### 9. Security Model

#### Assumptions
```
1. Honest Majority: >2/3 of stake is honest
2. Synchrony: Network delays < 1 epoch
3. Cryptography: SHA256 + Ed25519 are secure
4. Time: Clocks synchronized within tolerance
5. Persistence: Storage is crash-safe
```

#### Attack Resistance
```
Double Spend:           Prevented by nonce + ledger ordering
Replay:                 Prevented by nonce tracking
Fork:                   Resolved by stake + finality
Equivocation:           Slashed by double vote detection
DDoS:                   Rate limiting + peer reputation
State Manipulation:     Prevented by state root verification
```

## Deployment Scenarios

### Local Development
```bash
cargo run                    # Start single node
```

### Private Testnet
```bash
bash scripts/setup_testnet.sh
bash scripts/start_validators.sh  # 3 validators
bash scripts/monitor.sh
```

### Public Testnet
```
- Deploy 10+ validators
- Enable faucet
- Public RPC endpoints
- Explorer UI
- Community validator recruitment
```

### Mainnet
```
- Deploy 100+ validators
- Mainnet tokens
- Governance
- Staking economics live
- Full security audits
```

## Performance Characteristics

### Throughput
```
Tx/Block:        ~5,000 (assuming ~400 bytes/tx)
Block Time:      ~2 seconds
Finality:        ~8 blocks (16 seconds)
TPS:             ~2,500 theoretical, ~100 local
```

### Latency
```
Tx Submit → Mempool:        <100ms
Mempool → Block:            1-2 seconds
Block → Finality:           16 seconds
Total E2E:                  ~18 seconds
```

### Storage
```
Blocks/Day:     43,200
Block Size:     ~1 MB avg
Storage/Day:    ~43 GB
Storage/Year:   ~15 TB (single validator)
```

## Future Roadmap

### Q1: Foundation ✅
- [x] Core consensus
- [x] Validator system
- [x] P2P network
- [x] RPC API

### Q2: Economics
- [ ] Staking rewards live
- [ ] Slashing enforcement
- [ ] Faucet integration
- [ ] Public testnet

### Q3: Advanced Features
- [ ] Smart contract VM
- [ ] State sharding
- [ ] Light clients
- [ ] Governance

### Q4: Mainnet
- [ ] Security audits
- [ ] Mainnet launch
- [ ] Exchange listings
- [ ] DeFi ecosystem

---

**PESO: The Production-Ready Blockchain**
