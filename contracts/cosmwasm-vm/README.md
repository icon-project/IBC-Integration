## Rust

This Repo contains the smart contracts for ICON-IBC in rust.

### Standard Libraries

| Crate            | Usage                | Download                                                                                                                            |
|------------------|----------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| cosmwasm-schema  | Contract Development | [![cosmwasm-schema on crates.io](https://img.shields.io/crates/v/cosmwasm-schema.svg)](https://crates.io/crates/cosmwasm-schema)    |
| cosmwasm-std     | Contract Development | [![cosmwasm-std on crates.io](https://img.shields.io/crates/v/cosmwasm-std.svg)](https://crates.io/crates/cosmwasm-std)             |
| cosmwasm-storage | Contract Development | [![cosmwasm-storage on crates.io](https://img.shields.io/crates/v/cosmwasm-storage.svg)](https://crates.io/crates/cosmwasm-storage) |

### Prerequiste
Rust version : 1.69.0    
Binaryen : [110](https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERS}/binaryen-version_${BINARYEN_VERS}-x86_64-linux.tar.gz)

- To install Rust on Linux/Mac,

First, [install rustup](https://rustup.rs/). Once installed, make sure you have the wasm32 target:  

```shell
$ rustup default stable
$ cargo version
# If this is lower than 1.55.0+, update
$ rustup update stable

$ rustup target list --installed
$ rustup target add wasm32-unknown-unknown
```

For windows,
Download and run, [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)  

You can also execute the `./scripts/optimize-cosmwasm.sh`  script, which handles installing the necessary toolchain dependencies, building, and optimizing the contracts to their correct versions.

### Build Contracts


**1. Docker Build Process**

To initiate the build process using Docker, it is essential to have the `contract-builder` image. To generate this Docker image, execute the subsequent command:
```
make build-builder-img
```

In case you possess the `contract-builder` image already, you can directly employ the following command:
```
make optimize-cosmwasm
```
The resulting artifacts will be located within the `./artifacts/archway` directory.


**2. Using build script**
```
sh ./script/optimize-cosmwasm.sh
```

**3. Building manually**

- Execute the following command to compile the contract

  ```shell
  cargo wasm
  ```

- Optimise using cargo by giving the following command

  ```shell
  RUSTFLAGS='-C link-arg=-s' cargo wasm
  ```

### Deploy Contracts

- Deploy the contract on testnet

  ```shell
  // add the wasm binary file path in <path>

  RES=$(archwayd tx wasm store <path> --from wallet --chain-id constantine-2 --node https://rpc.constantine-2.archway.tech:443/ --fees 3397uconst --gas auto -y --output json -b block)
  ```

- Getting CodeId from RES

  ```shell
  CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
  ```

- To instantiate the contract

  ```shell
  archwayd tx wasm instantiate $CODE_ID "{}" --from wallet --gas auto --label "ics20" --no-admin --chain-id constantine-2 --node https://rpc.constantine-2.archway.tech:443/ --fees 300uconst -y
  ```

- Getting the contract address

  ```shell
  CONTRACT=$(archwayd query wasm list-contract-by-code 4 --output json | jq -r '.contracts[-1]')
  ```

- To Query the contract

  ```shell
  archwayd query wasm contract-state smart $CONTRACT '{"method_name":{}}' --output json
  ```

- To execute methods in contract

  ```shell
  archwayd tx wasm execute $CONTRACT '{"method_name":{}}' --from wallet --chain-id constantine-2 --output json
  ```

#### Checking contract validity

When the contract is built, the last step is to ensure it is valid CosmWasm contract is to call `check_contract` on it.

```shell
$ cargo wasm
...
$ check_contract . / target/wasm32-unknown-unknown/release/contract.wasm
Supported features: {"iterator", "staking", "stargate"}
contract checks passed.
```

### Contracts

#### [cw-ibc-core](./cw-ibc-core/src/) :

IBC-Core is the reference implementation of the Inter-Blockchain Communication (IBC) protocol, which is a standardized
protocol for enabling communication and interoperability between independent blockchain networks.

- [ics02_client](./cw-ibc-core/src/ics02_client) :

  ICS02-Client is a module in the IBC-Core that provides a standard interface and protocol for verifying the state of a
  remote chain in the Inter-Blockchain Communication (IBC) protocol.

- [ics03_connection](./cw-ibc-core/src/ics03_connection) :

  ICS03-Connection is a module in the IBC-Core that provides a standard interface and protocol for establishing a
  connection between two independent blockchain networks in the Inter-Blockchain Communication (IBC) protocol. It is
  designed to ensure secure and reliable communication between connected chains, enabling cross-chain asset transfers
  and other types of communication.

- [ics04_channel](./cw-ibc-core/src/ics04_channel) :

  The ICS04-Channel module works by creating a unidirectional or bidirectional communication channel between two chains,
  depending on the requirements of the use case. It negotiates the parameters of the channel, such as the channel ID,
  packet ordering, and reliability guarantees, and establishes a set of state machine rules that govern the behavior of
  the channel.

- [ics05_port](./cw-ibc-core/src/ics05_port) :

  This concrete implementation defines types and methods with which modules can bind to uniquely named ports allocated
  by the IBC handler.

- [ics24_host](./cw-ibc-core/src/ics24_host) :

  ICS24 is a module in the InterBlockchain Communication (IBC) protocol stack that facilitates the transfer of tokens
  and other data between independent blockchain networks.

- [ics26_routing](./cw-ibc-core/src/ics26_routing) :

  ICS26 is a module in the Inter-Blockchain Communication (IBC) protocol stack that defines the routing of packets
  between independent blockchain networks. It provides a mechanism for packets to be routed through a sequence of
  intermediate relayers until they reach their destination on the target blockchain network.

#### [cw-icon-light-client](./cw-icon-light-client/src) :

The icon-light-client in IBC enables blockchain networks to communicate with each other without having to trust each
other. This is achieved by using the cryptographic proofs to verify that transactions are valid and have been executed
correctly.

#### [cw-xcall-ibc-connection](./cw-xcall-ibc-connection/src/) :

This contract abstracts away ibc specific implementation from xcall. It bridges ibc host with xcall.

### Unit Testing

```
cd <contract>
cargo test
```

### References

- [cosmwasm_book](https://book.cosmwasm.com/)
- [ibc-rs](https://github.com/cosmos/ibc-rs/tree/main/crates/ibc)