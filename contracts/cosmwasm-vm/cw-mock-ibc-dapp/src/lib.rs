pub mod admin;
pub mod assertion;
pub mod check;
pub mod contract;
pub mod error;
pub mod ibc;
pub mod ibc_host;
pub mod msg;
pub mod owner;
pub mod receive_packet;
pub mod send_message;
pub mod state;
pub mod types;

use crate::{
    check::check_version,
    error::ContractError,
    ibc::IBC_VERSION,
    msg::InstantiateMsg,
    state::{CwIbcConnection, IbcConfig},
    types::storage_keys::StorageKey,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Never, Reply, Response,
    StdError, StdResult, Storage,
};
#[cfg(feature = "native_ibc")]
use cw_common::cw_types::{CwTimeout, CwTimeoutBlock};

use cw2::set_contract_version;
use cw_common::cw_types::{
    Cw3ChannelOpenResponse, CwBasicResponse, CwChannelCloseMsg, CwChannelConnectMsg,
    CwChannelOpenMsg, CwChannelOpenResponse, CwEndPoint, CwPacket, CwPacketAckMsg,
    CwPacketReceiveMsg, CwPacketTimeoutMsg, CwReceiveResponse,
};

use cw_storage_plus::Item;
use msg::{ExecuteMsg, QueryMsg};
use thiserror::Error;

/// This function instantiates a contract using the CwIbcConnection.
///
/// Arguments:
///
/// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
/// contract. Dependencies include the storage, API, and other modules that the contract may need to
/// interact with. The `DepsMut` object allows the contract to modify the state of the dependencies
/// * `env`: `env` is a struct that contains information about the current blockchain environment, such
/// as the block height, time, and chain ID. It is passed as a parameter to the `instantiate` function
/// in order to provide the contract with access to this information. The `env` parameter is of type
/// * `info`: `info` is a struct that contains information about the message sender, such as their
/// address, the amount of tokens they sent with the message, and any other metadata included in the
/// message. This information can be used to determine whether the sender is authorized to perform
/// certain actions and to handle the tokens sent
/// * `msg`: `msg` is a parameter of type `InstantiateMsg` which contains the data sent by the user
/// during contract instantiation. It is used to initialize the state of the contract. The fields of
/// `InstantiateMsg` are defined by the developer and can vary depending on the requirements of the
/// contract.
///
/// Returns:
///
/// The `instantiate` function returns a `Result<Response, ContractError>` which represents either a
/// successful response or an error.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let call_service = CwIbcConnection::default();

    call_service.instantiate(deps, env, info, msg)
}

/// This is a Rust function that executes a message and returns a response, using a call service.
///
/// Arguments:
///
/// * `deps`: `deps` is a `DepsMut` object that provides access to the dependencies of the contract,
/// such as the storage, API, and querier.
/// * `env`: `env` is an object that contains information about the current blockchain environment, such
/// as the block height, time, and chain ID. It also includes information about the current transaction,
/// such as the sender and recipient addresses, the amount of tokens being transferred, and the gas
/// limit and price. This information
/// * `info`: `info` is a struct that contains information about the sender of the message, such as
/// their address, the amount of tokens they sent with the message, and any other metadata that was
/// included. This information can be used to determine whether the sender is authorized to perform
/// certain actions, and to track the
/// * `msg`: `msg` is a parameter of type `ExecuteMsg` which represents the message sent to the contract
/// for execution. It contains the necessary information and data required to execute the desired action
/// on the contract. The specific fields and data contained within `ExecuteMsg` will depend on the
/// specific implementation of the contract
///
/// Returns:
///
/// The `execute` function returns a `Result<Response, ContractError>`.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut call_service = CwIbcConnection::default();

    call_service.execute(deps, env, info, msg)
}

/// This function calls a service to query data using the given dependencies, environment, and message.
///
/// Arguments:
///
/// * `deps`: `deps` is an instance of the `Deps` struct, which provides access to the dependencies of
/// the contract, such as the storage, API, and other modules.
/// * `env`: `env` is an object that contains information about the current blockchain environment, such
/// as the block height, time, and chain ID.
/// * `msg`: The `msg` parameter in the `query` function is of type `QueryMsg`, which is an enum that
/// defines all the possible query messages that can be sent to the smart contract. The `msg` parameter
/// represents the specific query message that is being sent to the smart contract. The `query
///
/// Returns:
///
/// a `StdResult<Binary>` which is a type alias for `Result<Binary, StdError>`. The `Binary` type
/// represents a binary data and `StdError` is a standard error type used in CosmWasm.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let call_service = CwIbcConnection::default();

    call_service.query(deps, env, msg)
}

/// This function handles a reply message in a Rust smart contract.
///
/// Arguments:
///
/// * `deps`: `deps` is a mutable reference to the dependencies of the contract. It allows the contract
/// to access the necessary modules and traits to interact with the blockchain and its state.
/// * `env`: `env` is an object that contains information about the current blockchain environment, such
/// as the block height, time, and chain ID. It is provided by the Cosmos SDK and is used to interact
/// with the blockchain.
/// * `msg`: The `msg` parameter in the `reply` function is of type `Reply`. It represents the reply
/// message sent by an external contract in response to a previous message sent by the current contract.
/// The `Reply` struct contains the following fields:
///
/// Returns:
///
/// a `Result<Response, ContractError>` where `Response` and `ContractError` are types defined in the
/// contract's codebase. The `Result` type indicates that the function can either return a successful
/// `Response` or an error of type `ContractError`.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let call_service = CwIbcConnection::default();

    call_service.reply(deps, env, msg)
}

#[cw_serde]
pub struct MigrateMsg {}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let call_service = CwIbcConnection::default();
    call_service.migrate(deps, env, msg)
}
