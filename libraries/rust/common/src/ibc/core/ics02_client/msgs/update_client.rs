//! Definition of domain type message `MsgUpdateAnyClient`.

use crate::ibc::prelude::*;

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgUpdateClient as RawMsgUpdateClient;
use ibc_proto::protobuf::Protobuf;

use crate::ibc::core::ics02_client::error::ClientError;
use crate::ibc::core::ics24_host::identifier::ClientId;
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;

pub const TYPE_URL: &str = "/ibc.core.client.v1.MsgUpdateClient";

/// A type of message that triggers the update of an on-chain (IBC) client with new headers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgUpdateClient {
    pub client_id: ClientId,
    pub header: Any,
    pub signer: Signer,
}

impl Msg for MsgUpdateClient {
    type Raw = RawMsgUpdateClient;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgUpdateClient> for MsgUpdateClient {}

impl TryFrom<RawMsgUpdateClient> for MsgUpdateClient {
    type Error = ClientError;

    fn try_from(raw: RawMsgUpdateClient) -> Result<Self, Self::Error> {
        Ok(MsgUpdateClient {
            client_id: raw
                .client_id
                .parse()
                .map_err(ClientError::InvalidMsgUpdateClientId)?,
            header: raw.header.ok_or(ClientError::MissingRawHeader)?,
            signer: raw.signer.parse().map_err(ClientError::Signer)?,
        })
    }
}

impl From<MsgUpdateClient> for RawMsgUpdateClient {
    fn from(ics_msg: MsgUpdateClient) -> Self {
        RawMsgUpdateClient {
            client_id: ics_msg.client_id.to_string(),
            header: Some(ics_msg.header),
            signer: ics_msg.signer.to_string(),
        }
    }
}
