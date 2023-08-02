use crate::ibc::prelude::*;

use core::str::FromStr;

use ibc_proto::ibc::core::channel::v1::Packet as RawPacket;
use serde::{Deserialize, Serialize};

use super::timeout::TimeoutHeight;
use crate::ibc::core::ics04_channel::error::{ChannelError, PacketError};
use crate::ibc::core::ics24_host::identifier::{ChannelId, PortId};
use crate::ibc::timestamp::{Expiry::Expired, Timestamp};
use crate::ibc::Height;

/// Enumeration of proof carrying ICS4 message, helper for relayer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketMsgType {
    Recv,
    Ack,
    TimeoutUnordered,
    TimeoutOrdered,
    TimeoutOnClose,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Receipt {
    Ok,
}

impl core::fmt::Display for PacketMsgType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PacketMsgType::Recv => write!(f, "(PacketMsgType::Recv)"),
            PacketMsgType::Ack => write!(f, "(PacketMsgType::Ack)"),
            PacketMsgType::TimeoutUnordered => write!(f, "(PacketMsgType::TimeoutUnordered)"),
            PacketMsgType::TimeoutOrdered => write!(f, "(PacketMsgType::TimeoutOrdered)"),
            PacketMsgType::TimeoutOnClose => write!(f, "(PacketMsgType::TimeoutOnClose)"),
        }
    }
}

/// The sequence number of a packet enforces ordering among packets from the same source.
#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct Sequence(u64);

impl FromStr for Sequence {
    type Err = ChannelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.parse::<u64>().map_err(|e| {
            ChannelError::InvalidStringAsSequence {
                value: s.to_string(),
                error: e,
            }
        })?))
    }
}

impl Sequence {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn increment(&self) -> Sequence {
        Sequence(self.0 + 1)
    }
}

impl From<u64> for Sequence {
    fn from(seq: u64) -> Self {
        Sequence(seq)
    }
}

impl From<Sequence> for u64 {
    fn from(s: Sequence) -> u64 {
        s.0
    }
}

impl core::fmt::Display for Sequence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
