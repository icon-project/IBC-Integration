use crate::ibc::prelude::*;

use ibc_proto::protobuf::Protobuf;

use ibc_proto::ibc::core::channel::v1::MsgChannelCloseInit as RawMsgChannelCloseInit;

use crate::ibc::core::ics04_channel::error::ChannelError;
use crate::ibc::core::ics24_host::identifier::{ChannelId, PortId};
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelCloseInit";

///
/// Message definition for the first step in the channel close handshake (`ChanCloseInit` datagram).
/// Per our convention, this message is sent to chain A.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgChannelCloseInit {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub signer: Signer,
}

impl Msg for MsgChannelCloseInit {
    type Raw = RawMsgChannelCloseInit;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgChannelCloseInit> for MsgChannelCloseInit {}

impl TryFrom<RawMsgChannelCloseInit> for MsgChannelCloseInit {
    type Error = ChannelError;

    fn try_from(raw_msg: RawMsgChannelCloseInit) -> Result<Self, Self::Error> {
        Ok(MsgChannelCloseInit {
            port_id_on_a: raw_msg.port_id.parse().map_err(ChannelError::Identifier)?,
            chan_id_on_a: raw_msg
                .channel_id
                .parse()
                .map_err(ChannelError::Identifier)?,
            signer: raw_msg.signer.parse().map_err(ChannelError::Signer)?,
        })
    }
}

impl From<MsgChannelCloseInit> for RawMsgChannelCloseInit {
    fn from(domain_msg: MsgChannelCloseInit) -> Self {
        RawMsgChannelCloseInit {
            port_id: domain_msg.port_id_on_a.to_string(),
            channel_id: domain_msg.chan_id_on_a.to_string(),
            signer: domain_msg.signer.to_string(),
        }
    }
}
