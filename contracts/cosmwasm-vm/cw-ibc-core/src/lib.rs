#![recursion_limit = "256"]
pub mod constants;
pub mod context;
pub mod contract;
pub mod conversions;
mod error;
pub mod gas_estimates;
pub mod ics02_client;
pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics05_port;
pub mod ics24_host;
pub mod ics26_routing;
pub mod light_client;
pub mod msg;
pub mod state;
pub mod storage_keys;
pub mod traits;
pub mod validations;
pub use crate::error::ContractError;
use gas_estimates::*;
use ics04_channel::ics03_connection::msg::MigrateMsg;

use crate::state::CwIbcStore;
use crate::{ics26_routing::router::CwIbcRouter, storage_keys::StorageKey};

use common::ibc::core::ics05_port::error::PortError;
use common::ibc::core::ics24_host::error::ValidationError;
use common::ibc::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use common::ibc::signer::Signer;
pub use common::ibc::{
    core::{
        ics02_client::{client_type::ClientType as IbcClientType, error::ClientError},
        ics03_connection::connection::ConnectionEnd,
        ics04_channel::{
            channel::ChannelEnd,
            error::{ChannelError, PacketError},
            packet::Sequence,
        },
        ics26_routing::context::ModuleId as IbcModuleId,
    },
    Height,
};
pub use constants::*;
use context::CwIbcCoreContext;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use cosmwasm_std::{
    entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, Storage,
};
#[allow(unused_imports)]
use cw2::set_contract_version;
use cw_common::client_msg::LightClientPacketMessage;
use cw_common::ibc_types::{IbcChannelId, IbcClientId, IbcConnectionId, IbcPortId};
use cw_storage_plus::{Item, Map};

pub use cw_common::commitment::*;
use std::str::FromStr;
use thiserror::Error;

use crate::msg::InstantiateMsg;
use crate::traits::ExecuteChannel;
use crate::traits::{IbcClient, ValidateChannel};

use cw_common::core_msg::ExecuteMsg as CoreExecuteMsg;
use cw_common::core_msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::InstantiateMsg,
) -> Result<Response, ContractError> {
    let call_service = CwIbcCoreContext::default();

    call_service.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: CoreExecuteMsg,
) -> Result<Response, ContractError> {
    let mut call_service = CwIbcCoreContext::default();

    call_service.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let call_service = CwIbcCoreContext::default();

    call_service.query(deps, env, msg)
}

/// This function handles a reply message in a contract this is the entry point for reply.
///
/// Arguments:
///
/// * `deps`: `deps` is a mutable reference to the dependencies of the contract. It allows the contract
/// to access the necessary modules and traits to interact with the blockchain and other contracts.
/// * `env`: `env` is an object that contains information about the current execution environment of the
/// contract, such as the block height, time, and chain ID. It is provided by the Cosmos SDK and is
/// passed as an argument to most contract functions.
/// * `msg`: The `msg` parameter in the `reply` function is of type `Reply`. It represents the message
/// that is being replied to by the contract. The `Reply` struct contains information about the original
/// message, such as the sender, recipient, and the actual message content.
///
/// Returns:
///
/// The function `reply` returns a `Result<Response, ContractError>`.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let call_service = CwIbcCoreContext::default();

    call_service.reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    if msg.clear_store {
        let store = CwIbcStore::default();
        store.clear_storage(deps.storage);
    }
    Ok(Response::default().add_attribute("migrate", "successful"))
}
