//! ICS 03: Connection implementation for connecting a client
pub mod conn_event;
pub mod connection;
use crate::context::CwIbcCoreContext;
use crate::types::{ClientId, ConnectionId};
use crate::ContractError;
use cosmwasm_std::Storage;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc::{
    core::ics03_connection::{events::OpenInit, msgs::conn_open_init::MsgConnectionOpenInit},
    events::IbcEvent,
};
use ibc_proto::protobuf::Protobuf;
