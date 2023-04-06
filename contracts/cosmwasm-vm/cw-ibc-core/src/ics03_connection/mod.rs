//! ICS 03: Connection implementation for connecting a client
pub mod connection;
pub mod delay;
pub mod event;
pub mod handler;
use crate::context::CwIbcCoreContext;
use crate::ics03_connection::event::create_open_init_event;
use crate::types::{ClientId, ConnectionId};
use crate::ContractError;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Event;
use cosmwasm_std::Response;
use cosmwasm_std::Storage;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics03_connection::error::ConnectionError;
use ibc::core::ics03_connection::{
    connection::State, msgs::conn_open_init::MsgConnectionOpenInit, version::Version,
};
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc::{
    core::ics03_connection::events::{
        CLIENT_ID_ATTRIBUTE_KEY, CONN_ID_ATTRIBUTE_KEY, COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
        COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
    },
    events::IbcEventType,
};
use ibc_proto::protobuf::Protobuf;
