## CosmWasm Contracts

### API

### Build

#### Prerequiste

- To install Rust in Linux and Mac,

First, [install rustup](https://rustup.rs/). Once installed, make sure you have the wasm32 target:
  
```shell
rustup default stable
cargo version
# If this is lower than 1.55.0+, update
rustup update stable

rustup target list --installed
rustup target add wasm32-unknown-unknown
```

For windows,
Download and run, [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)

### Testing

```
cd <contract>
cargo test
```