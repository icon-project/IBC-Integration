use crate::ibc::core::ics04_channel::channel::ChannelEnd;
use crate::ibc::core::ics04_channel::channel::Counterparty;
use crate::ibc::core::ics04_channel::channel::{Order, State};
use crate::ibc::core::ics04_channel::error::ChannelError;
use crate::ibc::core::ics04_channel::Version;
use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::ibc::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;
use crate::{ibc::prelude::*, ibc::Height};

use ibc_proto::ibc::core::channel::v1::MsgChannelOpenTry as RawMsgChannelOpenTry;
use ibc_proto::protobuf::Protobuf;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenTry";

///
/// Message definition for the second step in the channel open handshake (`ChanOpenTry` datagram).
/// Per our convention, this message is sent to chain B.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgChannelOpenTry {
    pub port_id_on_b: PortId,
    pub connection_hops_on_b: Vec<ConnectionId>,
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub version_supported_on_a: Version,
    pub proof_chan_end_on_a: CommitmentProofBytes,
    pub proof_height_on_a: Height,
    pub ordering: Order,
    pub signer: Signer,

    #[deprecated(since = "0.22.0")]
    /// Only kept here for proper conversion to/from the raw type
    pub previous_channel_id: String,
    #[deprecated(since = "0.22.0")]
    /// Only kept here for proper conversion to/from the raw type
    pub version_proposal: Version,
}

impl Msg for MsgChannelOpenTry {
    type Raw = RawMsgChannelOpenTry;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgChannelOpenTry> for MsgChannelOpenTry {}

impl TryFrom<RawMsgChannelOpenTry> for MsgChannelOpenTry {
    type Error = ChannelError;

    fn try_from(raw_msg: RawMsgChannelOpenTry) -> Result<Self, Self::Error> {
        let chan_end_on_b: ChannelEnd = raw_msg
            .channel
            .ok_or(ChannelError::MissingChannel)?
            .try_into()?;
        #[allow(deprecated)]
        let msg = MsgChannelOpenTry {
            port_id_on_b: raw_msg.port_id.parse().map_err(ChannelError::Identifier)?,
            ordering: chan_end_on_b.ordering,
            previous_channel_id: raw_msg.previous_channel_id,
            connection_hops_on_b: chan_end_on_b.connection_hops,
            port_id_on_a: chan_end_on_b.remote.port_id,
            chan_id_on_a: chan_end_on_b
                .remote
                .channel_id
                .ok_or(ChannelError::InvalidCounterpartyChannelId)?,
            version_supported_on_a: raw_msg.counterparty_version.into(),
            proof_chan_end_on_a: raw_msg
                .proof_init
                .try_into()
                .map_err(|_| ChannelError::InvalidProof)?,
            proof_height_on_a: raw_msg
                .proof_height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or(ChannelError::MissingHeight)?,
            signer: raw_msg.signer.parse().map_err(ChannelError::Signer)?,
            version_proposal: chan_end_on_b.version,
        };

        Ok(msg)
    }
}

impl From<MsgChannelOpenTry> for RawMsgChannelOpenTry {
    fn from(domain_msg: MsgChannelOpenTry) -> Self {
        let chan_end_on_b = ChannelEnd::new(
            State::Init,
            domain_msg.ordering,
            Counterparty::new(domain_msg.port_id_on_a, Some(domain_msg.chan_id_on_a)),
            domain_msg.connection_hops_on_b,
            Version::empty(), // Excessive field to satisfy the type conversion
        );
        #[allow(deprecated)]
        RawMsgChannelOpenTry {
            port_id: domain_msg.port_id_on_b.to_string(),
            previous_channel_id: domain_msg.previous_channel_id,
            channel: Some(chan_end_on_b.into()),
            counterparty_version: domain_msg.version_supported_on_a.to_string(),
            proof_init: domain_msg.proof_chan_end_on_a.clone().into(),
            proof_height: Some(domain_msg.proof_height_on_a.into()),
            signer: domain_msg.signer.to_string(),
        }
    }
}
