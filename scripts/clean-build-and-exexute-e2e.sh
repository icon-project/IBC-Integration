#!/bin/sh
set -e
echo "cleaning contract directories..."
find artifacts/icon -type f -exec rm {} \;
find artifacts/archway -type f -exec rm {} \;
echo "building contracts..."
./scripts/build-xcall.sh
./scripts/optimize-cosmwasm.sh
./scripts/optimize-jar.sh
echo "executing e2e test..."
go test -v ./test/e2e -timeout 0 -count=1


