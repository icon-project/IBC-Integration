use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::{ibc::prelude::*, ibc::Height};

use ibc_proto::protobuf::Protobuf;

use ibc_proto::ibc::core::channel::v1::MsgChannelCloseConfirm as RawMsgChannelCloseConfirm;

use crate::ibc::core::ics04_channel::error::ChannelError;
use crate::ibc::core::ics24_host::identifier::{ChannelId, PortId};
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelCloseConfirm";

///
/// Message definition for the second step in the channel close handshake (the `ChanCloseConfirm`
/// datagram).
/// Per our convention, this message is sent to chain B.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgChannelCloseConfirm {
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub proof_chan_end_on_a: CommitmentProofBytes,
    pub proof_height_on_a: Height,
    pub signer: Signer,
}

impl Msg for MsgChannelCloseConfirm {
    type Raw = RawMsgChannelCloseConfirm;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgChannelCloseConfirm> for MsgChannelCloseConfirm {}

impl TryFrom<RawMsgChannelCloseConfirm> for MsgChannelCloseConfirm {
    type Error = ChannelError;

    fn try_from(raw_msg: RawMsgChannelCloseConfirm) -> Result<Self, Self::Error> {
        Ok(MsgChannelCloseConfirm {
            port_id_on_b: raw_msg.port_id.parse().map_err(ChannelError::Identifier)?,
            chan_id_on_b: raw_msg
                .channel_id
                .parse()
                .map_err(ChannelError::Identifier)?,
            proof_chan_end_on_a: raw_msg
                .proof_init
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

impl From<MsgChannelCloseConfirm> for RawMsgChannelCloseConfirm {
    fn from(domain_msg: MsgChannelCloseConfirm) -> Self {
        RawMsgChannelCloseConfirm {
            port_id: domain_msg.port_id_on_b.to_string(),
            channel_id: domain_msg.chan_id_on_b.to_string(),
            proof_init: domain_msg.proof_chan_end_on_a.clone().into(),
            proof_height: Some(domain_msg.proof_height_on_a.into()),
            signer: domain_msg.signer.to_string(),
        }
    }
}
