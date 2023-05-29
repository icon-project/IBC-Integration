use crate::ibc::prelude::*;

use ibc_proto::protobuf::Protobuf;

use ibc_proto::ibc::core::channel::v1::MsgRecvPacket as RawMsgRecvPacket;

use crate::ibc::core::ics04_channel::error::PacketError;
use crate::ibc::core::ics04_channel::packet::Packet;
use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;
use crate::ibc::Height;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgRecvPacket";

///
/// Message definition for the "packet receiving" datagram.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgRecvPacket {
    /// The packet to be received
    pub packet: Packet,
    /// Proof of packet commitment on the sending chain
    pub proof_commitment_on_a: CommitmentProofBytes,
    /// Height at which the commitment proof in this message were taken
    pub proof_height_on_a: Height,
    /// The signer of the message
    pub signer: Signer,
}

impl Msg for MsgRecvPacket {
    type Raw = RawMsgRecvPacket;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgRecvPacket> for MsgRecvPacket {}

impl TryFrom<RawMsgRecvPacket> for MsgRecvPacket {
    type Error = PacketError;

    fn try_from(raw_msg: RawMsgRecvPacket) -> Result<Self, Self::Error> {
        Ok(MsgRecvPacket {
            packet: raw_msg
                .packet
                .ok_or(PacketError::MissingPacket)?
                .try_into()?,
            proof_commitment_on_a: raw_msg
                .proof_commitment
                .try_into()
                .map_err(|_| PacketError::InvalidProof)?,
            proof_height_on_a: raw_msg
                .proof_height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or(PacketError::MissingHeight)?,
            signer: raw_msg.signer.parse().map_err(PacketError::Signer)?,
        })
    }
}

impl From<MsgRecvPacket> for RawMsgRecvPacket {
    fn from(domain_msg: MsgRecvPacket) -> Self {
        RawMsgRecvPacket {
            packet: Some(domain_msg.packet.into()),
            proof_commitment: domain_msg.proof_commitment_on_a.into(),
            proof_height: Some(domain_msg.proof_height_on_a.into()),
            signer: domain_msg.signer.to_string(),
        }
    }
}
