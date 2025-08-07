# Check if CONTRACT variable is set
if [ -z "$CONTRACT" ]; then
    echo "Error: CONTRACT variable is not set"
    exit 1
fi

echo "Using contract ID: $CONTRACT"

stellar contract bindings typescript \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  --contract-id "$CONTRACT" \
  --output-dir lib

cd lib

npm install && npm run build

cd ../frontend

# Add soroban-run-club-lib to frontend package.json
jq '.dependencies += {"soroban-run-club-lib": "file:../lib"}' package.json > temp.json && mv temp.json package.json
npm install