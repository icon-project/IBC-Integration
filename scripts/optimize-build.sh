#!/bin/bash
set -e
echo "Generating optimized cosmwasm for Archway contracts..."
sh ./scripts/optimize-cosmwasm.sh
echo "Generating optimized cosmwasm (xCall) for icon contracts..."
sh ./scripts/build-xcall.sh
echo "Generating optimized jar for icon contracts..."
sh ./scripts/optimize-jar.sh
