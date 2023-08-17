# Getting Started with IBC Dapp for Contracts based IBC Host.

This guide will walk you through the process of getting started with a CosmWasm contract compatible with IBC Core Smart Contracts. We'll cover the basic structure of the contract code, the different entrypoints, and how to handle different types of messages.

## Prerequisites

Before you begin, make sure you have the following:

- A working development environment with Rust and the necessary tools installed.
- Familiarity with Rust programming language.
- Knowledge of smart contracts and blockchain concepts.

## Setting Up Your Project

1. Create a new Rust project using `cargo`:

   ```bash
   cargo new my_cosmwasm_contract
   cd my_cosmwasm_contract
   ```

2. Edit the `Cargo.toml` file to add the required dependencies:

   ```toml
   [dependencies]
   cosmwasm-std = "0.16"
   cosmwasm-derive = "0.16"
   ```

## Writing the Contract

Create a new Rust file in the `src` directory of your project, e.g., `contract.rs`. This is where you will define your contract logic.On contracts file add the entrypints to handle channel handshake from IBC Core Contract.
```rust
// src/contract.rs

use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, ContractError,
    log, to_binary, StdResult,
};

// Import message types
use crate::msg::{ExecuteMsg, CwIbcConnection};

// Import storage functions
use crate::state::{add_admin, send_message, ensure_ibc_handler, on_channel_open, /* ... */};

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
         ExecuteMsg::SetAdmin { address } => {
            let validated_address = CwIbcConnection::validate_address(deps.api, address.as_str())?;
            add_admin(deps.storage, info, validated_address.to_string())
        },
        ExecuteMsg::SendMessage { msg } => {
            log!("Received Payload From  App");
            send_message(deps, info, env, msg)
        }
        // Handle IBC-related messages (if not using native IBC)
        #[cfg(not(feature = "native_ibc"))]
        ExecuteMsg::IbcChannelOpen { msg } => {
            ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
            Ok(on_channel_open(deps.storage, msg)?)
        }
        // Add other IBC-related cases...
        #[cfg(feature = "native_ibc")]
        _ => Err(ContractError::DecodeFailed {
            error: "InvalidMessage Variant".to_string(),
        }),
    }
}
```

## Message Types and Storage

Create two new files in the `src` directory: `msg.rs` and `state.rs`. In `msg.rs`, define the message types that your contract will handle:

```rust
// src/msg.rs

use cosmwasm_std::{Addr, Binary, CanonicalAddr, Env, MessageInfo, StdError, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub enum ExecuteMsg {

    SendMessage {
        msg: Vec<u8>,
    },

    IbcChannelOpen {
        msg: CwChannelOpenMsg,
    },

   
    IbcChannelConnect {
        msg: CwChannelConnectMsg,
    },
   
    IbcChannelClose {
        msg: CwChannelCloseMsg,
    },
   
    IbcPacketReceive {
        msg: CwPacketReceiveMsg,
    },
   
    IbcPacketAck {
        msg: CwPacketAckMsg,
    },
   
    IbcPacketTimeout {
        msg: CwPacketTimeoutMsg,
    },
}

// Define your message structs here...

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Admin {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Message {
    pub msg: String,
}

// Define IBC-related message structs (if not using native IBC)...

```

In `state.rs`, define the storage functions and related logic:

```rust
// src/state.rs

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, ContractError, Addr, CanonicalAddr};

// Define your storage-related functions here...

pub fn add_admin(
    storage: &mut dyn Storage,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    // Add admin logic here...
}

pub fn send_message(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    msg: String,
    timeout_height: u64,
) -> Result<Response, ContractError> {
    // Send message logic here...
}

// Define IBC-related storage functions (if not using native IBC)...

```

## Conclusion

Congratulations! You've created a basic CosmWasm contract with the provided `execute` entrypoint and different message types. You can now build upon this foundation to add more functionality, handle additional message types, and integrate with other contracts and systems. Make sure to test your contract thoroughly before deploying it on a blockchain network.You can look at this contract for code details and how to proceed with channel handshake and other methods.

Remember to consult the official CosmWasm documentation and resources for more advanced topics and best practices. Happy coding!
```

Please note that this guide assumes you have some knowledge of Rust and blockchain concepts. It's a starting point, and you should refer to the official CosmWasm documentation for more in-depth information and advanced features. Additionally, ensure that your environment is set up properly with the necessary tools and dependencies.