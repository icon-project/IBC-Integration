//! ICS 03: Connection implementation for connecting a client
pub mod conn1_types;
pub mod connection;
pub mod event;
pub mod handler;
use crate::context::CwIbcCoreContext;
use crate::ics03_connection::event::create_open_init_event;
use crate::types::{ClientId, ConnectionId};
use crate::ContractError;
pub use conn1_types::*;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Event;
use cosmwasm_std::Response;
use cosmwasm_std::Storage;
use cosmwasm_std::{from_binary, to_binary, CosmosMsg, MessageInfo, SubMsg};
use cosmwasm_std::{to_vec, Reply};
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics03_connection::connection::Counterparty;
use ibc::core::ics03_connection::error::ConnectionError;
use ibc::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck;
use ibc::core::ics03_connection::{
    connection::State, msgs::conn_open_init::MsgConnectionOpenInit, version::Version,
};
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
pub use ibc::core::ics24_host::identifier::ConnectionId as IbcConnectionId;
use ibc::{
    core::ics03_connection::events::{
        CLIENT_ID_ATTRIBUTE_KEY, CONN_ID_ATTRIBUTE_KEY, COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
        COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
    },
    events::IbcEventType,
};
use ibc_proto::protobuf::Protobuf;
use std::str::FromStr;

use crate::ics03_connection::conn1_types::VerifyClientConsesnusState;
use crate::ics03_connection::conn1_types::VerifyClientFullState;
use crate::ics03_connection::conn1_types::VerifyConnectionState;
use crate::ics03_connection::event::create_open_ack_event;
