use crate::ibc::core::ics04_channel::channel::ChannelEnd;
use crate::ibc::core::ics04_channel::channel::Counterparty;
use crate::ibc::core::ics04_channel::channel::{Order, State};
use crate::ibc::core::ics04_channel::error::ChannelError;
use crate::ibc::core::ics04_channel::Version;
use crate::ibc::core::ics24_host::identifier::{ConnectionId, PortId};
use crate::ibc::prelude::*;
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;

use ibc_proto::ibc::core::channel::v1::MsgChannelOpenInit as RawMsgChannelOpenInit;
use ibc_proto::protobuf::Protobuf;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenInit";

///
/// Message definition for the first step in the channel open handshake (`ChanOpenInit` datagram).
/// Per our convention, this message is sent to chain A.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgChannelOpenInit {
    pub port_id_on_a: PortId,
    pub connection_hops_on_a: Vec<ConnectionId>,
    pub port_id_on_b: PortId,
    pub ordering: Order,
    pub signer: Signer,
    /// Allow a relayer to specify a particular version by providing a non-empty version string
    pub version_proposal: Version,
}

impl Msg for MsgChannelOpenInit {
    type Raw = RawMsgChannelOpenInit;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgChannelOpenInit> for MsgChannelOpenInit {}

impl TryFrom<RawMsgChannelOpenInit> for MsgChannelOpenInit {
    type Error = ChannelError;

    fn try_from(raw_msg: RawMsgChannelOpenInit) -> Result<Self, Self::Error> {
        let chan_end_on_a: ChannelEnd = raw_msg
            .channel
            .ok_or(ChannelError::MissingChannel)?
            .try_into()?;
        Ok(MsgChannelOpenInit {
            port_id_on_a: raw_msg.port_id.parse().map_err(ChannelError::Identifier)?,
            connection_hops_on_a: chan_end_on_a.connection_hops,
            port_id_on_b: chan_end_on_a.remote.port_id,
            ordering: chan_end_on_a.ordering,
            signer: raw_msg.signer.parse().map_err(ChannelError::Signer)?,
            version_proposal: chan_end_on_a.version,
        })
    }
}
impl From<MsgChannelOpenInit> for RawMsgChannelOpenInit {
    fn from(domain_msg: MsgChannelOpenInit) -> Self {
        let chan_end_on_a = ChannelEnd::new(
            State::Init,
            domain_msg.ordering,
            Counterparty::new(domain_msg.port_id_on_b, None),
            domain_msg.connection_hops_on_a,
            domain_msg.version_proposal,
        );
        RawMsgChannelOpenInit {
            port_id: domain_msg.port_id_on_a.to_string(),
            channel: Some(chan_end_on_a.into()),
            signer: domain_msg.signer.to_string(),
        }
    }
}
