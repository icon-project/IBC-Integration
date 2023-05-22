use crate::ibc::prelude::*;

use ibc_proto::protobuf::Protobuf;

use ibc_proto::ibc::core::channel::v1::MsgTimeout as RawMsgTimeout;

use crate::ibc::core::ics04_channel::error::PacketError;
use crate::ibc::core::ics04_channel::packet::{Packet, Sequence};
use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;
use crate::ibc::Height;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgTimeout";

///
/// Message definition for packet timeout domain type,
/// which is sent on chain A and needs to prove that a previously sent packet was not received on chain B
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgTimeout {
    pub packet: Packet,
    pub next_seq_recv_on_b: Sequence,
    pub proof_unreceived_on_b: CommitmentProofBytes,
    pub proof_height_on_b: Height,
    pub signer: Signer,
}

impl Msg for MsgTimeout {
    type Raw = RawMsgTimeout;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgTimeout> for MsgTimeout {}

impl TryFrom<RawMsgTimeout> for MsgTimeout {
    type Error = PacketError;

    fn try_from(raw_msg: RawMsgTimeout) -> Result<Self, Self::Error> {
        // TODO: Domain type verification for the next sequence: this should probably be > 0.
        Ok(MsgTimeout {
            packet: raw_msg
                .packet
                .ok_or(PacketError::MissingPacket)?
                .try_into()?,
            next_seq_recv_on_b: Sequence::from(raw_msg.next_sequence_recv),
            proof_unreceived_on_b: raw_msg
                .proof_unreceived
                .try_into()
                .map_err(|_| PacketError::InvalidProof)?,
            proof_height_on_b: raw_msg
                .proof_height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or(PacketError::MissingHeight)?,
            signer: raw_msg.signer.parse().map_err(PacketError::Signer)?,
        })
    }
}

impl From<MsgTimeout> for RawMsgTimeout {
    fn from(domain_msg: MsgTimeout) -> Self {
        RawMsgTimeout {
            packet: Some(domain_msg.packet.into()),
            proof_unreceived: domain_msg.proof_unreceived_on_b.into(),
            proof_height: Some(domain_msg.proof_height_on_b.into()),
            next_sequence_recv: domain_msg.next_seq_recv_on_b.into(),
            signer: domain_msg.signer.to_string(),
        }
    }
}
