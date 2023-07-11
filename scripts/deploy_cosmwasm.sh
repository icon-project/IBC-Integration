#!/bin/sh

set -e

print_usage() {
    echo "Usage: $0 [local|testnet]"
    echo "Description: This script requires deployment type as an argument."
}


if [ $# -eq 0 ]; then
    print_usage
    exit 1
fi

if [ "$1" == "local" ]; then
  ENDPOINT=http://localhost:26657
  CHAIN_ID=localnet
  WALLET=fd # This account is default while running the chain on docker.
  TOKEN=stake
elif [ "$1" == "testnet" ]; then
  ENDPOINT=https://rpc.constantine.archway.tech:443
  CHAIN_ID=constantine-3
  WALLET=constantine2_wallet
  TOKEN=uconst
else
  print_usage
  exit 1
fi

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

# Correct path
# echo $PWD
# sed -i "s|^CONTRACTS_DIR=.*|CONTRACTS_DIR=$PWD|" ./contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
# sed -i "s|^ARCHWAY_WALLET=.*|ARCHWAY_WALLET=constantine3Wallet|" ./contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
# sed -i "s|^ARCHWAY_NETWORK=.*|ARCHWAY_NETWORK=testnet|" ./contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
# sed -i "s|^ARCHWAY_NETWORK=.*|ARCHWAY_NETWORK=testnet|" ./contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
# sed -i "s|constantine-2|constantine-3|" ./contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
# sed -i "s|^ARCHWAY_ADDRESS=.*|ARCHWAY_ADDRESS=archway1z6r0f8r735mfrtrd4uv6x9f77tc6dsqzxtj7j4|" ./contracts/cosmwasm-vm/icon-ibc-setup/consts.sh

# sed -i '2i\sed -x' ./contracts/cosmwasm-vm/icon-ibc-setup/consts.sh

cd ./contracts/cosmwasm-vm/icon-ibc-setup
make archway
# This wasm directory is inside the docker container
sed -i "s|^CONTRACTS_DIR=.*|CONTRACTS_DIR=$PWD/IBC-Integration|" ../contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
sed -i "s|^ARCHWAY_WALLET=.*|ARCHWAY_WALLET=constantine3Wallet|" ../contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
sed -i "s|^ARCHWAY_NETWORK=.*|ARCHWAY_NETWORK=testnet|" ../contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
sed -i "s|^ARCHWAY_NETWORK=.*|ARCHWAY_NETWORK=testnet|" ../contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
sed -i "s|constantine-2|constantine-3|" ../contracts/cosmwasm-vm/icon-ibc-setup/consts.sh
sed -i '2i\sed -x' ../contracts/cosmwasm-vm/icon-ibc-setup/consts.sh

cd ../contracts/cosmwasm-vm/icon-ibc-setup
make archway
# # This wasm directory is inside the docker container
# for CONTRACT_WASM in /contracts/artifacts/*.wasm; do
#   echo "=> Deploying $CONTRACT_WASM"
#   deploy_wasm "$CONTRACT_WASM" 
# done

