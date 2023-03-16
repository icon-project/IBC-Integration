#!/usr/bin/env bash

set -eo pipefail

echo "Generating gogo proto code"
cd proto

wget -q https://sh.rustup.rs -O rustup_init.sh && chmod +x rustup_init.sh
sh rustup_init.sh -y
source "$HOME/.cargo/env"

apk --no-cache --update add build-base

cargo install protoc-gen-prost protoc-gen-prost-crate protoc-gen-prost-serde 

buf generate --template buf.gen.gogo.yaml $file

rm -rf rustup_init.sh

cargo fmt --all