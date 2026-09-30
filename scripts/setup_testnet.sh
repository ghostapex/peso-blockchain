#!/usr/bin/env bash
set -e

echo "🔧 PESO Testnet Setup"
echo "===================="

echo "Creating directories..."
mkdir -p peso_data
mkdir -p logs
mkdir -p config

echo "Generating validator configs..."
cat > config/validator-1.json << 'EOF'
{
  "id": "validator-1",
  "stake": 1000000,
  "peer": "127.0.0.1:9001",
  "rpc_port": 8545,
  "commission": 5
}
EOF

cat > config/validator-2.json << 'EOF'
{
  "id": "validator-2",
  "stake": 2000000,
  "peer": "127.0.0.1:9002",
  "rpc_port": 8546,
  "commission": 5
}
EOF

cat > config/validator-3.json << 'EOF'
{
  "id": "validator-3",
  "stake": 3000000,
  "peer": "127.0.0.1:9003",
  "rpc_port": 8547,
  "commission": 5
}
EOF

echo "Generating genesis..."
cat > config/genesis.json << 'EOF'
{
  "network_name": "PESO Testnet",
  "chain_id": "peso-testnet-v1",
  "genesis_time": 1720000000,
  "validators": [
    {"id": "validator-1", "stake": 1000000, "commission": 5},
    {"id": "validator-2", "stake": 2000000, "commission": 5},
    {"id": "validator-3", "stake": 3000000, "commission": 5}
  ],
  "faucet_total": 1000000000
}
EOF

echo "✅ Testnet setup complete!"
echo "Run: bash scripts/start_validators.sh"
