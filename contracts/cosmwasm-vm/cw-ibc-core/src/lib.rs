pub mod constants;
pub mod context;
pub mod contract;
mod error;
pub mod gas_estimates;
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
pub use crate::error::ContractError;
use gas_estimates::*;

use crate::state::CwIbcStore;
use crate::{ics26_routing::router::CwIbcRouter, storage_keys::StorageKey};
pub use constants::*;
use context::CwIbcCoreContext;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use cosmwasm_std::{
    entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, Storage,
};
#[allow(unused_imports)]
use cw2::set_contract_version;
use cw_common::client_msg::LightClientPacketMessage;
use cw_common::types::{ChannelId, ClientId, ClientType, ConnectionId, PortId};
use cw_common::{
    IbcChannelId, IbcClientId, IbcConnectionId, IbcPortId, MsgCreateClient, MsgUpdateClient,
};
use cw_storage_plus::{Item, Map};
use ibc::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck;
use ibc::core::ics03_connection::msgs::conn_open_confirm::MsgConnectionOpenConfirm;
use ibc::core::ics03_connection::msgs::conn_open_init::MsgConnectionOpenInit;
use ibc::core::ics04_channel::msgs::acknowledgement::MsgAcknowledgement;
use ibc::core::ics04_channel::msgs::recv_packet::MsgRecvPacket;
use ibc::core::ics04_channel::msgs::timeout::MsgTimeout;
use ibc::core::ics04_channel::msgs::timeout_on_close::MsgTimeoutOnClose;
pub use ibc::core::ics04_channel::msgs::{
    chan_close_confirm::MsgChannelCloseConfirm, chan_close_init::MsgChannelCloseInit,
    chan_open_ack::MsgChannelOpenAck, chan_open_confirm::MsgChannelOpenConfirm,
    chan_open_init::MsgChannelOpenInit, chan_open_try::MsgChannelOpenTry,
};
use ibc::core::ics05_port::error::PortError;
use ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc::core::ics24_host::error::ValidationError;
pub use ibc::{
    core::{
        ics02_client::{
            client_type::ClientType as IbcClientType, error::ClientError,
            msgs::upgrade_client::MsgUpgradeClient,
        },
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
pub use ics24_host::commitment::*;
use std::str::FromStr;
use thiserror::Error;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::traits::{IbcClient, ValidateChannel};
use crate::{
    ics02_client::types::{ClientState, ConsensusState, SignedHeader},
    traits::ExecuteChannel,
};

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
