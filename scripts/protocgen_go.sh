#!/usr/bin/env bash

set -eo pipefail

echo "Generating go proto code"
cd proto

buf generate --template buf.gen.go.yaml $file

cd ..

rm -rf github.com

go mod tidy
