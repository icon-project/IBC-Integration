## Wasm LightClient
Implementation of icon lightclient compatible with ics-08 standard.
Since the centauri chain currently does not support features like "staking" in wasm build, we need to disable "ibc3" feature flag in comswasm-std dependency before building it. We cant build entire workspace because other projects depend on "ibc3" feature being enabled, so we have to do standalone build for wasm lightclient.
We can build the lightclient using following command
`
cargo build -p cw-wasm-light-client --release --target wasm32-unknown-unknown
`
