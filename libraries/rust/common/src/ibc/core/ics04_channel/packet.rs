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

#[derive(Clone, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Packet {
    pub sequence: Sequence,
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub data: Vec<u8>,
    pub timeout_height_on_b: TimeoutHeight,
    pub timeout_timestamp_on_b: Timestamp,
}

struct PacketData<'a>(&'a [u8]);

impl<'a> core::fmt::Debug for PacketData<'a> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(formatter, "{:?}", self.0)
    }
}

impl core::fmt::Debug for Packet {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        // Remember: if you alter the definition of `Packet`,
        // 1. update the formatter debug struct builder calls (return object of
        //    this function)
        // 2. update this destructuring assignment accordingly
        let Packet {
            sequence: _,
            port_id_on_a: _,
            chan_id_on_a: _,
            port_id_on_b: _,
            chan_id_on_b: _,
            data,
            timeout_height_on_b: _,
            timeout_timestamp_on_b: _,
        } = self;
        let data_wrapper = PacketData(data);

        formatter
            .debug_struct("Packet")
            .field("sequence", &self.sequence)
            .field("source_port", &self.port_id_on_a)
            .field("source_channel", &self.chan_id_on_a)
            .field("destination_port", &self.port_id_on_b)
            .field("destination_channel", &self.chan_id_on_b)
            .field("data", &data_wrapper)
            .field("timeout_height", &self.timeout_height_on_b)
            .field("timeout_timestamp", &self.timeout_timestamp_on_b)
            .finish()
    }
}

impl Packet {
    /// Checks whether a packet from a
    /// [`SendPacket`](crate::core::ics04_channel::events::SendPacket)
    /// event is timed-out relative to the current state of the
    /// destination chain.
    ///
    /// Checks both for time-out relative to the destination chain's
    /// current timestamp `dst_chain_ts` as well as relative to
    /// the height `dst_chain_height`.
    ///
    /// Note: a timed-out packet should result in a
    /// [`MsgTimeout`](crate::core::ics04_channel::msgs::timeout::MsgTimeout),
    /// instead of the common-case where it results in
    /// [`MsgRecvPacket`](crate::core::ics04_channel::msgs::recv_packet::MsgRecvPacket).
    pub fn timed_out(&self, dst_chain_ts: &Timestamp, dst_chain_height: Height) -> bool {
        let height_timed_out = self.timeout_height_on_b.has_expired(dst_chain_height);

        let timestamp_timed_out = self.timeout_timestamp_on_b != Timestamp::none()
            && dst_chain_ts.check_expiry(&self.timeout_timestamp_on_b) == Expired;

        height_timed_out || timestamp_timed_out
    }
}

/// Custom debug output to omit the packet data
impl core::fmt::Display for Packet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "seq:{}, path:{}/{}->{}/{}, toh:{}, tos:{})",
            self.sequence,
            self.chan_id_on_a,
            self.port_id_on_a,
            self.chan_id_on_b,
            self.port_id_on_b,
            self.timeout_height_on_b,
            self.timeout_timestamp_on_b
        )
    }
}

impl TryFrom<RawPacket> for Packet {
    type Error = PacketError;

    fn try_from(raw_pkt: RawPacket) -> Result<Self, Self::Error> {
        if Sequence::from(raw_pkt.sequence).is_zero() {
            return Err(PacketError::ZeroPacketSequence);
        }

        // Note: ibc-go currently (July 2022) incorrectly treats the timeout
        // heights `{revision_number : >0, revision_height: 0}` as valid
        // timeouts. However, heights with `revision_height == 0` are invalid in
        // Tendermint. We explicitly reject these values because they go against
        // the Tendermint spec, and shouldn't be used. To timeout on the next
        // revision_number as soon as the chain starts,
        // `{revision_number: old_rev + 1, revision_height: 1}`
        // should be used.
        let packet_timeout_height: TimeoutHeight = raw_pkt
            .timeout_height
            .try_into()
            .map_err(|_| PacketError::InvalidTimeoutHeight)?;

        if raw_pkt.data.is_empty() {
            return Err(PacketError::ZeroPacketData);
        }

        let timeout_timestamp_on_b = Timestamp::from_nanoseconds(raw_pkt.timeout_timestamp)
            .map_err(PacketError::InvalidPacketTimestamp)?;

        Ok(Packet {
            sequence: Sequence::from(raw_pkt.sequence),
            port_id_on_a: raw_pkt
                .source_port
                .parse()
                .map_err(PacketError::Identifier)?,
            chan_id_on_a: raw_pkt
                .source_channel
                .parse()
                .map_err(PacketError::Identifier)?,
            port_id_on_b: raw_pkt
                .destination_port
                .parse()
                .map_err(PacketError::Identifier)?,
            chan_id_on_b: raw_pkt
                .destination_channel
                .parse()
                .map_err(PacketError::Identifier)?,
            data: raw_pkt.data,
            timeout_height_on_b: packet_timeout_height,
            timeout_timestamp_on_b,
        })
    }
}

impl From<Packet> for RawPacket {
    fn from(packet: Packet) -> Self {
        RawPacket {
            sequence: packet.sequence.0,
            source_port: packet.port_id_on_a.to_string(),
            source_channel: packet.chan_id_on_a.to_string(),
            destination_port: packet.port_id_on_b.to_string(),
            destination_channel: packet.chan_id_on_b.to_string(),
            data: packet.data,
            timeout_height: packet.timeout_height_on_b.into(),
            timeout_timestamp: packet.timeout_timestamp_on_b.nanoseconds(),
        }
    }
}
