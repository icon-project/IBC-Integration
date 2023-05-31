#!/bin/sh

set -ex

WALLET=fd # This account is default while running the chain on docker.
### LOCALNET ###
ENDPOINT=http://localhost:26657
CHAIN_ID=localnet
TOKEN=stake

deploy_wasm() {
  RES=$(archwayd tx wasm store $CONTRACT_WASM \
    --keyring-backend test \
    --from $WALLET \
    --node $ENDPOINT \
    --chain-id $CHAIN_ID \
    --gas-prices 0.02$TOKEN \
    --gas auto \
    --gas-adjustment 1.3 -y \
    --output json -b block)

  echo "Result: "
  echo "$RES"
}

# This wasm directory is inside the docker container
for CONTRACT_WASM in /contracts/artifacts/*.wasm; do
  echo "=> Deploying $CONTRACT_WASM"
  deploy_wasm "$CONTRACT_WASM"
done
