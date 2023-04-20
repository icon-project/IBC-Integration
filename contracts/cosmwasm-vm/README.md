## Rust

This Repo contains the smart contracts for ICON-IBC in rust.

### Standard Libraries

| crate          | Usage              | Download                |
|----------------|--------------------|-------------------------|
|cosmwasm-schema |Contract Development| [![cosmwasm-schema on crates.io](https://img.shields.io/crates/v/cosmwasm-schema.svg)](https://crates.io/crates/cosmwasm-schema) |
|cosmwasm-std    |Contract Development| [![cosmwasm-std on crates.io](https://img.shields.io/crates/v/cosmwasm-std.svg)](https://crates.io/crates/cosmwasm-std)       |
|cosmwasm-storage|Contract Development| [![cosmwasm-storage on crates.io](https://img.shields.io/crates/v/cosmwasm-storage.svg)](https://crates.io/crates/cosmwasm-storage)
  
### Build Contracts

- [cosmwasm-template](https://github.com/CosmWasm/cosmwasm-template) :

  A starter-pack to get you quickly building your custom contract compatible with the cosmwasm system.

- [rust-optimizer](https://github.com/cosmwasm/rust-optimizer) :

  A Docker image and scripts take your rust code to give the smallest possible Wasm output. This is designed both for preparing contracts for deployment as well as validating the given deployed contract is based on some given source code. 

- [deployment and interaction](https://docs.cosmwasm.com/docs/getting-started/interact-with-contract) :
  
  If we have the wasm binary ready. Now its time to deploy it to the testnet and start interacting.

#### Checking contract validity

When the contract is built, the last step is to ensure it is valid CosmWasm contract is to call `check_contract` on it.

```rust
$ cargo wasm
...
$ check_contract ./target/wasm32-unknown-unknown/release/contract.wasm
Supported features: {"iterator", "staking", "stargate"}
contract checks passed.
```

#### Prerequiste

- To install Rust in Linux/Mac,

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

### Unit Testing

```
cd <contract>
cargo test
```