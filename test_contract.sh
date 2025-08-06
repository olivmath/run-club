#!/bin/bash

# Test script for the Run Club contract
# Contract ID: CDIJN5LCNOVQZOBIRLAD32PAR2EHYMS7T7YSE6TWSK7RE4CMGT67CC3M

CONTRACT_ID="CDIJN5LCNOVQZOBIRLAD32PAR2EHYMS7T7YSE6TWSK7RE4CMGT67CC3M"
SOURCE="alice"
NETWORK="testnet"

echo "🏃 Testing Run Club Contract"
echo "Contract ID: $CONTRACT_ID"
echo ""

# Initialize the contract
echo "1. Initializing contract..."
stellar contract invoke \
  --id $CONTRACT_ID \
  --source $SOURCE \
  --network $NETWORK \
  -- \
  initialize

echo ""
echo "2. Creating a test club..."
# Create a club (you'll need to replace ORGANIZER_ADDRESS with actual address)
stellar contract invoke \
  --id $CONTRACT_ID \
  --source $SOURCE \
  --network $NETWORK \
  -- \
  create_club \
  --organizer GDAT5HWTGIU4TSSZ4752OUC4SABDLTLZFRPZUJ3D6LKBNEPA7V2CIG54 \
  --name "Test Running Club" \
  --usdc_per_km 100 \
  --withdrawal_rule Equal \
  --duration_days 30

echo ""
echo "3. Getting club info..."
stellar contract invoke \
  --id $CONTRACT_ID \
  --source $SOURCE \
  --network $NETWORK \
  -- \
  get_club \
  --club_id 1

echo ""
echo "✅ Contract deployment and basic functionality test completed!"
echo "🔗 View on Stellar Expert: https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"