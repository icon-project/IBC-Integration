//! ICS 02: Client implementation for verifying remote IBC-enabled chains.

pub mod client;
pub mod events;
pub mod handler;
pub mod types;

use crate::{
    context::CwIbcCoreContext,
    msg::*,
    traits::IbcClient,
    types::{ClientId, ClientType},
    ContractError, IbcClientId, IbcClientType,
};
use common::icon::icon::lightclient::v1::{
    ClientState as RawClientState, ConsensusState as RawConsensusState,
};
use cosmwasm_std::{
    from_binary, to_binary, to_vec, Addr, CosmosMsg, DepsMut, Event, MessageInfo, Reply, Response,
    Storage, SubMsg,
};
use events::{create_client_event, update_client_event, upgrade_client_event};
use ibc::core::{
    ics02_client::{
        client_state::ClientState as IbcClientState,
        consensus_state::ConsensusState as IbcConsensusState, error::ClientError,
    },
    ics03_connection::connection::ConnectionEnd,
    ics04_channel::channel::ChannelEnd,
    ics04_channel::packet::Sequence,
    ics23_commitment::commitment::CommitmentRoot,
    ContextError,
};
use ibc::{
    core::ics02_client::{
        events::{
            CLIENT_ID_ATTRIBUTE_KEY, CLIENT_TYPE_ATTRIBUTE_KEY, CONSENSUS_HEIGHTS_ATTRIBUTE_KEY,
            CONSENSUS_HEIGHT_ATTRIBUTE_KEY,
        },
        msgs::{
            create_client::MsgCreateClient, update_client::MsgUpdateClient,
            upgrade_client::MsgUpgradeClient,
        },
    },
    events::IbcEventType,
    timestamp::Timestamp,
    Height,
};
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use types::*;
