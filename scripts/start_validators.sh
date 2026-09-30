#!/usr/bin/env bash
set -e

echo "🚀 Starting PESO Validators..."

# Start validator 1
echo "Starting validator-1..."
cargo run --bin peso_chain -- --node validator-1 --peer 127.0.0.1:9001 &
VAL1_PID=$!

# Start validator 2
echo "Starting validator-2..."
cargo run --bin peso_chain -- --node validator-2 --peer 127.0.0.1:9002 &
VAL2_PID=$!

# Start validator 3
echo "Starting validator-3..."
cargo run --bin peso_chain -- --node validator-3 --peer 127.0.0.1:9003 &
VAL3_PID=$!

echo "All validators started."
echo "Validator 1 PID: $VAL1_PID"
echo "Validator 2 PID: $VAL2_PID"
echo "Validator 3 PID: $VAL3_PID"

wait
