#!/bin/bash
set -e
# contracts
CONTRACTS=("CallService" "DAppProxySample" "MultiProtocolSampleDapp" "LayerZeroAdapter" "WormholeAdapter" "CentralizedConnection")

# Directory paths
build_directory="build"
artifacts_directory="artifacts/evm"

mkdir -p "$artifacts_directory"


cd contracts/evm

forge clean
forge build --out "$build_directory" --extra-output-files abi bin
cd -
for file in "${CONTRACTS[@]}"; do
  file_path="contracts/evm/$build_directory/$file.sol"
  mv "$file_path/$file.abi.json" "$artifacts_directory/$file.abi.json"
  mv "$file_path/$file.bin" "$artifacts_directory/$file.bin"
done

cd -
