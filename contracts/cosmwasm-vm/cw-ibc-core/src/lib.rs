pub mod constants;
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
    types::{
        ChannelId, ClientId, ClientType, ConnectionId, PortId, VerifyChannelState,
        VerifyClientConsesnusState, VerifyClientFullState, VerifyConnectionState,VerifyPacketData,
    },
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, Storage,
};

use crate::ics04_channel::LightClientPacketMessage;
pub use constants::*;
use context::CwIbcCoreContext;
use cw_storage_plus::{Item, Key, KeyDeserialize, Map, Prefixer, PrimaryKey};
pub use ibc::core::ics04_channel::msgs::{
    chan_close_confirm::MsgChannelCloseConfirm, chan_close_init::MsgChannelCloseInit,
    chan_open_ack::MsgChannelOpenAck, chan_open_confirm::MsgChannelOpenConfirm,
    chan_open_init::MsgChannelOpenInit, chan_open_try::MsgChannelOpenTry,
};
use ibc::core::{ics05_port::error::PortError, ContextError};
pub use ibc::{
    core::{
        ics02_client::{
            client_type::ClientType as IbcClientType,
            error::ClientError,
            msgs::{
                create_client::MsgCreateClient, update_client::MsgUpdateClient,
                upgrade_client::MsgUpgradeClient,
            },
        },
        ics03_connection::connection::ConnectionEnd,
        ics04_channel::{
            channel::ChannelEnd,
            error::{ChannelError, PacketError},
            packet::Sequence,
        },
        ics24_host::identifier::{
            ChannelId as IbcChannelId, ClientId as IbcClientId, ConnectionId as IbcConnectionId,
            PortId as IbcPortId,
        },
        ics26_routing::context::ModuleId as IbcModuleId,
    },
    Height,
};
pub use ics24_host::commitment::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Error as FmtError, Formatter},
    str::FromStr,
};
use thiserror::Error;

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
    msg: msg::ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut call_service = CwIbcCoreContext::default();

    call_service.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    let call_service = CwIbcCoreContext::default();

    call_service.query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let call_service = CwIbcCoreContext::default();

    call_service.reply(deps, env, msg)
}
