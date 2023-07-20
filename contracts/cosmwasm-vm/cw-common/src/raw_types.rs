pub mod client {

    pub use ibc_proto::ibc::core::client::v1::{
        MsgCreateClient as RawMsgCreateClient, MsgSubmitMisbehaviour as RawMsgSubmitMisbehaviour,
        MsgUpdateClient as RawMsgUpdateClient, MsgUpgradeClient as RawMsgUpgradeClient,
    };
}

pub mod connection {
    pub use ibc_proto::ibc::core::connection::v1::Counterparty as RawCounterpartyConnection;
    pub use ibc_proto::ibc::core::connection::v1::{
        ConnectionEnd as RawConnectionEnd, Counterparty as RawCounterparty,
        IdentifiedConnection as RawIdentifiedConnection,
    };
    pub use ibc_proto::ibc::core::connection::v1::{
        MsgConnectionOpenAck as RawMsgConnectionOpenAck,
        MsgConnectionOpenConfirm as RawMsgConnectionOpenConfirm,
        MsgConnectionOpenInit as RawMsgConnectionOpenInit,
        MsgConnectionOpenTry as RawMsgConnectionOpenTry,
    };
}

pub mod channel {
    pub use ibc_proto::ibc::core::channel::v1::Channel as RawChannel;
    pub use ibc_proto::ibc::core::channel::v1::IdentifiedChannel as RawIdentifiedChannel;
    pub use ibc_proto::ibc::core::channel::v1::MsgAcknowledgement as RawMsgAcknowledgement;
    pub use ibc_proto::ibc::core::channel::v1::MsgChannelCloseInit as RawMsgChannelCloseInit;
    pub use ibc_proto::ibc::core::channel::v1::MsgRecvPacket as RawMsgRecvPacket;
    pub use ibc_proto::ibc::core::channel::v1::MsgTimeout as RawMsgTimeout;
    pub use ibc_proto::ibc::core::channel::v1::MsgTimeoutOnClose as RawMsgTimeoutOnClose;
    pub use ibc_proto::ibc::core::channel::v1::{
        MsgAcknowledgement as RawMessageAcknowledgement,
        MsgChannelCloseConfirm as RawMsgChannelCloseConfirm,
        MsgChannelOpenAck as RawMsgChannelOpenAck,
        MsgChannelOpenConfirm as RawMsgChannelOpenConfirm,
        MsgChannelOpenInit as RawMsgChannelOpenInit, MsgChannelOpenTry as RawMsgChannelOpenTry,
        MsgRecvPacket as RawMessageRecvPacket, MsgTimeout as RawMessageTimeout,
        MsgTimeoutOnClose as RawMessageTimeoutOnclose, Packet as RawPacket,
    };
    pub use ibc_proto::ibc::core::{
        channel::v1::Counterparty as RawCounterparty,
        commitment::v1::MerklePrefix as RawMerklePrefix,
    };
}

use common::ibc::Height;
pub use ibc_proto::google::protobuf::Any;
pub use ibc_proto::ibc::core::client::v1::Height as RawHeight;
pub use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
pub use ibc_proto::ibc::core::connection::v1::Version as RawVersion;
pub use ibc_proto::ics23::CommitmentProof as RawCommitmentProof;
pub use ibc_proto::protobuf::Protobuf;

use self::channel::RawPacket;
use crate::cw_types::CwPacket;

pub fn to_raw_packet(packet: CwPacket) -> RawPacket {
    let timestamp = packet.timeout.timestamp().map(|t| t.nanos()).unwrap_or(0);

    let timeout_height = packet.timeout.block().map(|b| RawHeight {
        revision_height: b.height,
        revision_number: b.revision,
    });
    return RawPacket {
        sequence: packet.sequence,
        source_port: packet.src.port_id,
        source_channel: packet.src.channel_id,
        destination_port: packet.dest.port_id,
        destination_channel: packet.dest.channel_id,
        data: packet.data.0,
        timeout_height,
        timeout_timestamp: timestamp,
    };
}
