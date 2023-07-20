//! ICS 03: Connection implementation for connecting a client
pub mod connection;
pub mod delay;
pub mod event;
pub mod handler;
use crate::context::CwIbcCoreContext;
use crate::ics03_connection::event::create_open_ack_event;
use crate::ics03_connection::event::create_open_confirm_event;
// use crate::ics03_connection::event::create_open_init_event;
use crate::ics03_connection::event::create_open_try_event;
use crate::ContractError;
use common::ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
use cosmwasm_std::DepsMut;
use cosmwasm_std::Event;
use cosmwasm_std::Response;
use cosmwasm_std::Storage;
use cosmwasm_std::{to_vec, MessageInfo};
use cw_common::client_msg::{
    VerifyClientConsensusState, VerifyClientFullState, VerifyConnectionState,
};

pub use super::*;
use common::ibc::core::ics03_connection::connection::ConnectionEnd;
use common::ibc::core::ics03_connection::error::ConnectionError;
use common::ibc::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck;
use common::ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry;
pub use common::ibc::core::ics03_connection::{
    connection::{Counterparty, State},
    msgs::{conn_open_confirm::MsgConnectionOpenConfirm, conn_open_init::MsgConnectionOpenInit},
    version::Version,
};
use common::ibc::core::ics23_commitment::commitment::CommitmentPrefix;
pub use common::ibc::core::ics24_host::identifier::ConnectionId as IbcConnectionId;
use common::ibc::Height;
use common::ibc::{
    core::ics03_connection::events::{
        CLIENT_ID_ATTRIBUTE_KEY, CONN_ID_ATTRIBUTE_KEY, COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
        COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
    },
    events::IbcEventType,
};

use cw_common::commitment;
use cw_common::raw_types::Protobuf;
