#!/bin/sh

set -e
mkdir -p artifacts/icon
mkdir -p artifacts/archway

echo "building xcall contracts..."
cd xcall-multi
./scripts/optimize-cosmwasm.sh
./scripts/optimize-jar.sh
echo "$PWD"
cp artifacts/archway/*.wasm ../artifacts/archway/
cp artifacts/icon/*.jar ../artifacts/icon/
