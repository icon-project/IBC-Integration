use crate::ibc::core::ics04_channel::error::ChannelError;
use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::ibc::core::ics24_host::identifier::{ChannelId, PortId};
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;
use crate::{ibc::prelude::*, ibc::Height};

use ibc_proto::ibc::core::channel::v1::MsgChannelOpenConfirm as RawMsgChannelOpenConfirm;
use ibc_proto::protobuf::Protobuf;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenConfirm";

///
/// Message definition for the fourth step in the channel open handshake (`ChanOpenConfirm`
/// datagram).
/// Per our convention, this message is sent to chain B.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgChannelOpenConfirm {
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub proof_chan_end_on_a: CommitmentProofBytes,
    pub proof_height_on_a: Height,
    pub signer: Signer,
}

impl Msg for MsgChannelOpenConfirm {
    type Raw = RawMsgChannelOpenConfirm;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgChannelOpenConfirm> for MsgChannelOpenConfirm {}

impl TryFrom<RawMsgChannelOpenConfirm> for MsgChannelOpenConfirm {
    type Error = ChannelError;

    fn try_from(raw_msg: RawMsgChannelOpenConfirm) -> Result<Self, Self::Error> {
        Ok(MsgChannelOpenConfirm {
            port_id_on_b: raw_msg.port_id.parse().map_err(ChannelError::Identifier)?,
            chan_id_on_b: raw_msg
                .channel_id
                .parse()
                .map_err(ChannelError::Identifier)?,
            proof_chan_end_on_a: raw_msg
                .proof_ack
                .try_into()
                .map_err(|_| ChannelError::InvalidProof)?,
            proof_height_on_a: raw_msg
                .proof_height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or(ChannelError::MissingHeight)?,
            signer: raw_msg.signer.parse().map_err(ChannelError::Signer)?,
        })
    }
}

impl From<MsgChannelOpenConfirm> for RawMsgChannelOpenConfirm {
    fn from(domain_msg: MsgChannelOpenConfirm) -> Self {
        RawMsgChannelOpenConfirm {
            port_id: domain_msg.port_id_on_b.to_string(),
            channel_id: domain_msg.chan_id_on_b.to_string(),
            proof_ack: domain_msg.proof_chan_end_on_a.into(),
            proof_height: Some(domain_msg.proof_height_on_a.into()),
            signer: domain_msg.signer.to_string(),
        }
    }
}
