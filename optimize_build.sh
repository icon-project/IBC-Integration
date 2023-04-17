RUSTFLAGS='-C link-arg=-s' cargo build --release --lib --target wasm32-unknown-unknown --locked
for WASM in ./target/wasm32-unknown-unknown/release/*.wasm; do
    NAME=$(basename "$WASM" .wasm)${SUFFIX}.wasm
    echo "Creating intermediate hash for $NAME ..."
    sha256sum -- "$WASM" | tee -a artifacts/checksums_intermediate.txt
    echo "Optimizing $NAME ..."
    wasm-opt -Os "$WASM" -o "artifacts/$NAME"
  done