cargo fmt --all
cargo clean
RUSTFLAGS='-C link-arg=-s' cargo build  --workspace --exclude test-utils --release --lib --target wasm32-unknown-unknown
for WASM in ./target/wasm32-unknown-unknown/release/*.wasm; do
    NAME=$(basename "$WASM" .wasm)${SUFFIX}.wasm
    echo "Creating intermediate hash for $NAME ..."
    sha256sum -- "$WASM" | tee -a artifacts/checksums_intermediate.txt
    echo "Optimizing $NAME ..."
    wasm-opt -Oz "$WASM"   -o "artifacts/$NAME"
  done
cosmwasm-check artifacts/cw_ibc_core.wasm
cosmwasm-check artifacts/cw_icon_light_client.wasm
cosmwasm-check artifacts/cw_mock_dapp.wasm
cosmwasm-check artifacts/cw_xcall.wasm
cosmwasm-check artifacts/cw_xcall_ibc_connection.wasm
cosmwasm-check artifacts/cw_xcall_app.wasm
cosmwasm-check artifacts/cw_ibc_client.wasm