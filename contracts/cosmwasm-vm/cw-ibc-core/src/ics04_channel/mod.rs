//! ICS 04: Channel implementation that facilitates communication between
pub mod channel;
pub use channel::*;
pub mod events;
pub use events::*;
pub mod handler;
pub use handler::*;
pub mod chan_types;
pub use chan_types::*;

pub use super::*;
use crate::context::CwIbcCoreContext;
pub use channel::*;
use cosmwasm_std::Event;
use cosmwasm_std::Storage;
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
pub use traits::*;
pub use crate::types::*;

use ibc::core::ics03_connection::connection::State as ConnectionState;
// Constants for Reply messages
pub const EXECUTE_ON_CHANNEL_OPEN_INIT: u64 = 41;
pub const EXECUTE_ON_CHANNEL_OPEN_TRY: u64 = 42;
pub const EXECUTE_ON_CHANNEL_OPEN_TRY_ON_LIGHT_CLIENT: u64 = 421;
