#!/bin/sh

set -e
cd xcall-multi
./scripts/optimize-cosmwasm.sh
echo "$PWD"
cp artifacts/archway/*.wasm ../artifacts/archway/