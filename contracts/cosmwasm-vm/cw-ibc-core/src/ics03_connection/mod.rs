//! ICS 03: Connection implementation for connecting a client
pub mod connection;
pub mod event;
use crate::context::CwIbcCoreContext;
use crate::types::{ClientId, ConnectionId};
use crate::ContractError;
use cosmwasm_std::Event;
use cosmwasm_std::Storage;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics03_connection::events::COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY;
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc::{
    core::ics03_connection::events::{
        CLIENT_ID_ATTRIBUTE_KEY, CONN_ID_ATTRIBUTE_KEY, COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
    },
    events::IbcEventType,
};
use ibc_proto::protobuf::Protobuf;
