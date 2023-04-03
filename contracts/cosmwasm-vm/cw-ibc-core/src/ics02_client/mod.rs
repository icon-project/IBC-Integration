//! ICS 02: Client implementation for verifying remote IBC-enabled chains.

pub mod client;
pub mod events;
pub mod handler;
pub mod types;

use crate::traits::IbcClient;
use crate::{
    context::CwIbcCoreContext,
    types::{ClientId, ClientType},
    ContractError, IbcClientId, IbcClientType,
};
use common::icon::icon::lightclient::v1::ClientState as RawClientState;
use common::icon::icon::lightclient::v1::ConsensusState as RawConsensusState;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{from_binary, to_binary, CosmosMsg, MessageInfo, Reply, Response, SubMsg};
use cosmwasm_std::{Addr, DepsMut, Event, Storage};
use ibc::core::ics02_client::client_state::ClientState as IbcClientState;
use ibc::core::ics02_client::consensus_state::ConsensusState as IbcConsensusState;
use ibc::core::ics02_client::error::ClientError;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics04_channel::channel::ChannelEnd;
use ibc::core::ics04_channel::packet::Sequence;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::core::ContextError;
use ibc::{
    core::ics02_client::events::{
        CLIENT_ID_ATTRIBUTE_KEY, CLIENT_TYPE_ATTRIBUTE_KEY, CONSENSUS_HEIGHTS_ATTRIBUTE_KEY,
        CONSENSUS_HEIGHT_ATTRIBUTE_KEY,
    },
    events::IbcEventType,
};
use ibc::{
    core::ics02_client::msgs::{
        create_client::MsgCreateClient, update_client::MsgUpdateClient,
        upgrade_client::MsgUpgradeClient,
    },
    Height,
};
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use types::*;
