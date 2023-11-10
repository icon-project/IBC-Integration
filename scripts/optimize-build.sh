#!/bin/bash
set -e
build_ibc_contracts() {
  echo "Generating optimized cosmwasm for Archway contracts..."
  bash ./scripts/optimize-cosmwasm.sh
  echo "Generating optimized jar for icon contracts..."
  bash ./scripts/optimize-jar.sh
}

if [ "$1" = "build" ]; then
  build_ibc_contracts
fi
