use crate::ibc::core::ics04_channel::error::ChannelError;
use crate::ibc::core::ics04_channel::Version;
use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::ibc::core::ics24_host::identifier::{ChannelId, PortId};
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;
use crate::{ibc::prelude::*, ibc::Height};

use ibc_proto::ibc::core::channel::v1::MsgChannelOpenAck as RawMsgChannelOpenAck;
use ibc_proto::protobuf::Protobuf;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenAck";

///
/// Per our convention, this message is sent to chain A.
/// Message definition for the third step in the channel open handshake (`ChanOpenAck` datagram).
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgChannelOpenAck {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub chan_id_on_b: ChannelId,
    pub version_on_b: Version,
    pub proof_chan_end_on_b: CommitmentProofBytes,
    pub proof_height_on_b: Height,
    pub signer: Signer,
}

impl Msg for MsgChannelOpenAck {
    type Raw = RawMsgChannelOpenAck;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgChannelOpenAck> for MsgChannelOpenAck {}

impl TryFrom<RawMsgChannelOpenAck> for MsgChannelOpenAck {
    type Error = ChannelError;

    fn try_from(raw_msg: RawMsgChannelOpenAck) -> Result<Self, Self::Error> {
        Ok(MsgChannelOpenAck {
            port_id_on_a: raw_msg.port_id.parse().map_err(ChannelError::Identifier)?,
            chan_id_on_a: raw_msg
                .channel_id
                .parse()
                .map_err(ChannelError::Identifier)?,
            chan_id_on_b: raw_msg
                .counterparty_channel_id
                .parse()
                .map_err(ChannelError::Identifier)?,
            version_on_b: raw_msg.counterparty_version.into(),
            proof_chan_end_on_b: raw_msg
                .proof_try
                .try_into()
                .map_err(|_| ChannelError::InvalidProof)?,
            proof_height_on_b: raw_msg
                .proof_height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or(ChannelError::MissingHeight)?,
            signer: raw_msg.signer.parse().map_err(ChannelError::Signer)?,
        })
    }
}

impl From<MsgChannelOpenAck> for RawMsgChannelOpenAck {
    fn from(domain_msg: MsgChannelOpenAck) -> Self {
        RawMsgChannelOpenAck {
            port_id: domain_msg.port_id_on_a.to_string(),
            channel_id: domain_msg.chan_id_on_a.to_string(),
            counterparty_channel_id: domain_msg.chan_id_on_b.to_string(),
            counterparty_version: domain_msg.version_on_b.to_string(),
            proof_try: domain_msg.proof_chan_end_on_b.into(),
            proof_height: Some(domain_msg.proof_height_on_b.into()),
            signer: domain_msg.signer.to_string(),
        }
    }
}
