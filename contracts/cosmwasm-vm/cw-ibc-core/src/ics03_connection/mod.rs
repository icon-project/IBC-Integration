//! ICS 03: Connection implementation for connecting a client
pub mod conn2_types;
pub mod connection;
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
use std::str::FromStr;
use crate::ics03_connection::conn2_types::OpenConfirmResponse;
use crate::ics03_connection::conn2_types::VerifyConnectionState;
use crate::ics03_connection::event::create_open_confirm_event;
use crate::IbcConnectionId;
use cosmwasm_std::{from_binary, to_binary, to_vec, CosmosMsg, MessageInfo, Reply, SubMsg};
use ibc::core::ics03_connection::{
    connection::Counterparty, msgs::conn_open_confirm::MsgConnectionOpenConfirm,
};
