use crate::ibc::prelude::*;

use core::time::Duration;

use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit as RawMsgConnectionOpenInit;
use ibc_proto::protobuf::Protobuf;

use crate::ibc::core::ics03_connection::connection::Counterparty;
use crate::ibc::core::ics03_connection::error::ConnectionError;
use crate::ibc::core::ics03_connection::version::Version;
use crate::ibc::core::ics24_host::identifier::ClientId;
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;

pub const TYPE_URL: &str = "/ibc.core.connection.v1.MsgConnectionOpenInit";

/// Per our convention, this message is sent to chain A.
/// The handler will check proofs of chain B.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgConnectionOpenInit {
    /// ClientId on chain A that the connection is being opened for
    pub client_id_on_a: ClientId,
    pub counterparty: Counterparty,
    pub version: Option<Version>,
    pub delay_period: Duration,
    pub signer: Signer,
}

impl Msg for MsgConnectionOpenInit {
    type Raw = RawMsgConnectionOpenInit;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgConnectionOpenInit> for MsgConnectionOpenInit {}

impl TryFrom<RawMsgConnectionOpenInit> for MsgConnectionOpenInit {
    type Error = ConnectionError;

    fn try_from(msg: RawMsgConnectionOpenInit) -> Result<Self, Self::Error> {
        Ok(Self {
            client_id_on_a: msg
                .client_id
                .parse()
                .map_err(ConnectionError::InvalidIdentifier)?,
            counterparty: msg
                .counterparty
                .ok_or(ConnectionError::MissingCounterparty)?
                .try_into()?,
            version: msg.version.map(|version| version.try_into()).transpose()?,
            delay_period: Duration::from_nanos(msg.delay_period),
            signer: msg.signer.parse().map_err(ConnectionError::Signer)?,
        })
    }
}

impl From<MsgConnectionOpenInit> for RawMsgConnectionOpenInit {
    fn from(ics_msg: MsgConnectionOpenInit) -> Self {
        RawMsgConnectionOpenInit {
            client_id: ics_msg.client_id_on_a.as_str().to_string(),
            counterparty: Some(ics_msg.counterparty.into()),
            version: ics_msg.version.map(|version| version.into()),
            delay_period: ics_msg.delay_period.as_nanos() as u64,
            signer: ics_msg.signer.to_string(),
        }
    }
}
