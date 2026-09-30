use peso_chain::bootstrap::BootstrapConfig;
use peso_chain::consensus::Consensus;
use peso_chain::execution_engine::ConflictDetector;
use peso_chain::faucet::Faucet;
use peso_chain::finality::Finality;
use peso_chain::ledger::Ledger;
use peso_chain::metrics::MetricsCollector;
use peso_chain::node::Node;
use peso_chain::p2p_network::P2PNetwork;
use peso_chain::persistent_store::PersistentStore;
use peso_chain::rewards::RewardCalculator;
use peso_chain::slashing_enforcer::SlashingEnforcer;
use peso_chain::staking::StakePool;
use peso_chain::testnet_genesis::TestnetGenesis;
use peso_chain::transaction::Transaction;
use peso_chain::validator::Validator;
use peso_chain::validator_identity::ValidatorRegistry;
use peso_chain::wallet::Wallet;

fn main() {
    println!("\n🚀 ===== PESO BLOCKCHAIN - COMPLETE LAUNCH ===== 🚀");
    println!("Version: 0.2.0 - Production Ready Architecture\n");

    let genesis = TestnetGenesis::default();

    println!("📋 Network Configuration:");
    println!("   Name: {}", genesis.network_name);
    println!("   Chain ID: {}", genesis.chain_id);
    println!("   Validators: {}", genesis.validators.len());
    println!("   Faucet Total: {} PESO\n", genesis.faucet_total);

    let mut validator_registry = ValidatorRegistry::new();
    let mut validator_set = Vec::new();
    let mut stake_pool = StakePool::new();

    println!("👤 Registering Validators:");
    for v in &genesis.validators {
        println!("   - {} (stake: {}, commission: {}%)", v.id, v.stake, v.commission);
        validator_registry.register(v.id.clone(), format!("pub-key-{}", v.id), v.commission);
        let validator = Validator::new(v.id.clone(), v.stake);
        stake_pool.add_stake(v.id.clone(), v.stake);
        validator_set.push(validator);
    }
    println!();

    let mut consensus = Consensus::new(validator_set.clone());
    println!("🔗 Consensus Initialized:");
    println!("   Current Leader: {}", consensus.current_leader().unwrap().id);
    println!("   Current Epoch: {}\n", consensus.epoch);

    let reward_calc = RewardCalculator::new(0.08);
    println!("💰 Reward Calculator:");
    println!("   Base Rate: 8%");
    println!("   Validator-1 Reward: {} PESO", reward_calc.calculate_validator_reward(1_000_000, 0));
    println!("   Validator-2 Reward: {} PESO", reward_calc.calculate_validator_reward(2_000_000, 0));
    println!("   Validator-3 Reward: {} PESO\n", reward_calc.calculate_validator_reward(3_000_000, 0));

    let slashing_enforcer = SlashingEnforcer::new();
    println!("⚠️ Slashing Configuration:");
    println!("   Double Vote Slash: 33%");
    println!("   Inactivity Slash: 1%\n");

    let mut ledger = Ledger::new();
    let alice = Wallet::new();
    let bob = Wallet::new();

    ledger.add_account(alice.account_id.clone(), 10_000_000);
    ledger.add_account(bob.account_id.clone(), 5_000_000);

    println!("👛 Wallets Created:");
    println!("   Alice: {} (10M PESO)", alice.account_id);
    println!("   Bob: {} (5M PESO)\n", bob.account_id);

    let mut tx = Transaction::new(alice.account_id.clone(), bob.account_id.clone(), 1_000_000, 0);
    tx.sign(&alice);

    ledger.apply_transaction(&tx).unwrap();

    println!("💸 Transaction Executed:");
    println!("   From: {} -> To: {}", alice.account_id, bob.account_id);
    println!("   Amount: 1,000,000 PESO");
    println!("   Alice Balance: {}", ledger.get_balance(&alice.account_id));
    println!("   Bob Balance: {}\n", ledger.get_balance(&bob.account_id));

    let mut p2p = P2PNetwork::new("peso-node-main".to_string());
    p2p.add_peer("validator-1:9001".to_string());
    p2p.add_peer("validator-2:9002".to_string());
    p2p.add_peer("validator-3:9003".to_string());

    println!("🌐 P2P Network Initialized:");
    println!("   Local Peer: {}", p2p.local_peer_id);
    println!("   Connected Peers: {}\n", p2p.peers.len());

    let persistent_store = PersistentStore::new("./peso_data");
    println!("💾 Persistent Storage:");
    println!("   Database: RocksDB");
    println!("   Location: ./peso_data\n");

    let mut faucet = Faucet::new(genesis.faucet_total);
    let test_account = Wallet::new();
    if let Ok(amount) = faucet.request(&test_account.account_id) {
        println!("🚰 Faucet:");
        println!("   Drip Amount: {} PESO", amount);
        println!("   Requested Account: {}", test_account.account_id);
        println!("   Total Distributed: {}", faucet.distributed);
        println!("   Remaining: {}\n", faucet.total_supply - faucet.distributed);
    }

    let mut metrics = MetricsCollector::new();
    metrics.record_block();
    metrics.record_transaction();
    metrics.set_validators(3, 6_000_000);

    println!("📊 Metrics:");
    println!("   Blocks Produced: {}", metrics.blocks_produced);
    println!("   Transactions Processed: {}", metrics.transactions_processed);
    println!("   Active Validators: {}", metrics.validators_active);
    println!("   Total Stake: {}", metrics.total_stake);
    println!("   TPS: {}\n", metrics.get_tps());

    let conflict_detector = ConflictDetector::new();
    println!("⚙️  Execution Engine:");
    println!("   Conflict Detection: Enabled");
    println!("   Account Locking: Enabled");
    println!("   Parallel Execution: Ready\n");

    println!("✅ ===== PESO BLOCKCHAIN READY =====");
    println!("\n📚 Next Steps:");
    println!("   1. Start RPC Server: cargo run --features rpc");
    println!("   2. Launch Validators: bash scripts/start_validators.sh");
    println!("   3. Run Block Explorer: cargo run --features explorer");
    println!("   4. Monitor Network: cargo run --features monitor");
    println!("\n🌍 Public Testnet Configuration:");
    println!("   - Validator Registration: Open");
    println!("   - Block Explorer: http://localhost:8080");
    println!("   - RPC Endpoint: http://localhost:8545");
    println!("   - Faucet: http://localhost:3000/faucet\n");
}
