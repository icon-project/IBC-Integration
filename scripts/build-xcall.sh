#!/bin/sh

set -e
cd xCall
./scripts/optimize-cosmwasm.sh
echo "$PWD"
cp artifacts/archway/*.wasm ../artifacts/archway/