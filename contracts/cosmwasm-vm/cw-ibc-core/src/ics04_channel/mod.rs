//! ICS 04: Channel implementation that facilitates communication between
pub mod channel;
pub use channel::*;
pub mod events;
pub use events::*;
pub mod handler;
pub use super::*;
use crate::context::CwIbcCoreContext;
pub use crate::types::*;
pub use channel::*;
pub use handler::*;

use cosmwasm_std::Event;
use cosmwasm_std::Reply;
use cosmwasm_std::Storage;
use cosmwasm_std::{
    from_binary, to_binary, to_vec, Binary, CosmosMsg, Empty, MessageInfo, Response, SubMsg,
    WasmMsg,
};
use ibc::core::ics03_connection::connection::State as ConnectionState;
use ibc::core::ics03_connection::events::CONN_ID_ATTRIBUTE_KEY;
pub use ibc::core::ics04_channel::channel::Counterparty;
use ibc::core::ics04_channel::channel::Order;
pub use ibc::core::ics04_channel::channel::State;
pub use ibc::core::ics04_channel::events::*;
pub use ibc::core::ics04_channel::msgs::{
    chan_close_confirm::MsgChannelCloseConfirm, chan_close_init::MsgChannelCloseInit,
    chan_open_ack::MsgChannelOpenAck, chan_open_confirm::MsgChannelOpenConfirm,
    chan_open_init::MsgChannelOpenInit, chan_open_try::MsgChannelOpenTry, ChannelMsg,
};
use ibc::core::ics04_channel::packet::Packet;
use ibc::core::{
    ics04_channel::error::{ChannelError, PacketError},
    ContextError,
};
use ibc::events::IbcEventType;
pub use msg::LightClientMessage;
use std::str::FromStr;
pub use traits::*;

// Constants for Reply messages

pub const EXECUTE_ON_CHANNEL_OPEN_INIT: u64 = 41;
pub const EXECUTE_ON_CHANNEL_OPEN_TRY: u64 = 42;
pub const EXECUTE_ON_CHANNEL_OPEN_TRY_ON_LIGHT_CLIENT: u64 = 421;
pub const EXECUTE_ON_CHANNEL_OPEN_ACK_ON_LIGHT_CLIENT: u64 = 431;
pub const EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE: u64 = 432;
pub const EXECUTE_ON_CHANNEL_CLOSE_INIT: u64 = 45;
pub const EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_LIGHT_CLIENT: u64 = 461;
pub const EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE: u64 = 462;
pub const EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_LIGHT_CLIENT: u64 = 441;
pub const EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE: u64 = 442;

pub mod packet;
use ibc::core::ics04_channel::commitment::PacketCommitment;
pub use packet::*;

use ibc::{core::ics04_channel::timeout::TimeoutHeight, timestamp::Timestamp};
