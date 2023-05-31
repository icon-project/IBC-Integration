#!/bin/bash
set -e

BINARYEN_VERS=110
BINARYEN_DWN="https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERS}/binaryen-version_${BINARYEN_VERS}-x86_64-linux.tar.gz"

mkdir -p artifacts
cargo fmt --all
cargo clippy --fix
cargo clean

# Install toolchains
# cargo add target wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
cargo install cosmwasm-check
if ! which wasm-opt; then
  curl -OL $BINARYEN_DWN
  tar xf binaryen-version_${BINARYEN_VERS}-x86_64-linux.tar.gz
  export PATH=$PATH:$PWD/binaryen-version_${BINARYEN_VERS}/bin
fi

wasm-opt --version

RUSTFLAGS='-C link-arg=-s' cargo build --workspace --exclude test-utils --release --lib --target wasm32-unknown-unknown
for WASM in ./target/wasm32-unknown-unknown/release/*.wasm; do
  NAME=$(basename "$WASM" .wasm)${SUFFIX}.wasm
  echo "Creating intermediate hash for $NAME ..."
  sha256sum -- "$WASM" | tee -a artifacts/checksums_intermediate.txt
  echo "Optimizing $NAME ..."
  wasm-opt -Oz "$WASM" -o "artifacts/$NAME"
done

# check all generated wasm files
cosmwasm-check artifacts/cw_ibc_core.wasm
cosmwasm-check artifacts/cw_icon_light_client.wasm
cosmwasm-check artifacts/cw_mock_dapp.wasm
cosmwasm-check artifacts/cw_xcall.wasm
cosmwasm-check artifacts/cw_xcall_ibc_connection.wasm
cosmwasm-check artifacts/cw_xcall_app.wasm
