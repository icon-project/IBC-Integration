#!/bin/sh
set -e
forge remappings
forge build
mkdir artifacts
cat out/CallService.sol/CallService.json | jq '{"abi": .abi, "bytecode": .bytecode.object}'  > artifacts/xcall_abi_bytecode.json
cat out/CentralizedConnection.sol/CentralizedConnection.json | jq '{"abi": .abi, "bytecode": .bytecode.object}' > artifacts/centralized_connection_abi_byte_code.json
cat out/LayerZeroAdapter.sol/LayerZeroAdapter.json | jq '{"abi": .abi, "bytecode": .bytecode.object}' > artifacts/layer_zero_adapter_abi_bytecode.json
cat out/WormholeAdapter.sol/WormholeAdapter.json | jq '{"abi": .abi, "bytecode": .bytecode.object}' > artifacts/wormhole_adapter_abi.json
