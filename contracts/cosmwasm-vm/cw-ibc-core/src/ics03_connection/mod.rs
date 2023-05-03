//! ICS 03: Connection implementation for connecting a client
pub mod connection;
pub mod delay;
pub mod event;
pub mod handler;
use crate::context::CwIbcCoreContext;
use crate::ics03_connection::event::create_open_ack_event;
use crate::ics03_connection::event::create_open_confirm_event;
use crate::ics03_connection::event::create_open_init_event;
use crate::ics03_connection::event::create_open_try_event;
use crate::ContractError;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Event;
use cosmwasm_std::Response;
use cosmwasm_std::Storage;
use cosmwasm_std::{from_binary, to_binary, to_vec, CosmosMsg, MessageInfo, Reply, SubMsg};
use cw_common::client_msg::{
    VerifyClientConsensusState, VerifyClientFullState, VerifyConnectionState,
};
use cw_common::client_response::{OpenAckResponse, OpenTryResponse};
use cw_common::types::{ClientId, ConnectionId};

pub use super::*;
use cw_common::client_response::OpenConfirmResponse;
use cw_common::commitment;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics03_connection::error::ConnectionError;
use ibc::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck;
use ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry;
pub use ibc::core::ics03_connection::{
    connection::{Counterparty, State},
    msgs::{conn_open_confirm::MsgConnectionOpenConfirm, conn_open_init::MsgConnectionOpenInit},
    version::Version,
};
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
pub use ibc::core::ics24_host::identifier::ConnectionId as IbcConnectionId;
use ibc::Height;
use ibc::{
    core::ics03_connection::events::{
        CLIENT_ID_ATTRIBUTE_KEY, CONN_ID_ATTRIBUTE_KEY, COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
        COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
    },
    events::IbcEventType,
};
use ibc_proto::protobuf::Protobuf;
use std::{str::FromStr, time::Duration};
