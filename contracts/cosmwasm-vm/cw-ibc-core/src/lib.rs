pub mod context;
pub mod contract;
mod error;
pub mod helpers;
pub mod ics02_client;
pub mod ics03_connection;
pub mod ics04_channel;
pub mod ics05_port;
pub mod ics24_host;
pub mod ics26_routing;
pub mod msg;
pub mod state;
pub mod storage_keys;
pub mod traits;
pub mod types;

pub use crate::error::ContractError;
use crate::state::CwIbcStore;
use crate::{
    ics26_routing::router::CwIbcRouter,
    storage_keys::StorageKey,
    types::{ChannelId, ClientId, ClientType, ConnectionId, PortId},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult,
};
use cw_storage_plus::{Item, Key, KeyDeserialize, Map, Prefixer, PrimaryKey};
use ibc::core::{
    ics02_client::error::ClientError,
    ics04_channel::error::{ChannelError, PacketError},
    ics05_port::error::PortError,
    ContextError,
};
pub use ibc::core::{
    ics02_client::{
        client_type::ClientType as IbcClientType,
        msgs::{
            create_client::MsgCreateClient, update_client::MsgUpdateClient,
            upgrade_client::MsgUpgradeClient,
        },
    },
    ics03_connection::connection::ConnectionEnd,
    ics04_channel::channel::ChannelEnd,
    ics04_channel::packet::Sequence,
    ics24_host::identifier::{
        ChannelId as IbcChannelId, ClientId as IbcClientId, ConnectionId as IbcConnectionId,
        PortId as IbcPortId,
    },
    ics26_routing::context::ModuleId as IbcModuleId,
    ics26_routing::context::{Module, ModuleId},
};
use ics04_channel::context::CwIbcCoreContext;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Error as FmtError, Formatter},
    str::FromStr,
};
use thiserror::Error;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::InstantiateMsg,
) -> Result<Response, ContractError> {
    let call_service = CwIbcCoreContext::default();

    call_service.instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut call_service = CwIbcCoreContext::default();

    call_service.execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    let call_service = CwIbcCoreContext::default();

    call_service.query(deps, env, msg)
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let call_service = CwIbcCoreContext::default();

    call_service.reply(deps, env, msg)
}
