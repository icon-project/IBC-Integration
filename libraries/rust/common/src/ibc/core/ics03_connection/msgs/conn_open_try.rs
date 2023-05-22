use crate::ibc::prelude::*;

use core::{
    convert::{TryFrom, TryInto},
    time::Duration,
};

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry as RawMsgConnectionOpenTry;
use ibc_proto::protobuf::Protobuf;

use crate::ibc::core::ics03_connection::connection::Counterparty;
use crate::ibc::core::ics03_connection::error::ConnectionError;
use crate::ibc::core::ics03_connection::version::Version;
use crate::ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use crate::ibc::core::ics24_host::identifier::ClientId;
use crate::ibc::signer::Signer;
use crate::ibc::tx_msg::Msg;
use crate::ibc::Height;

pub const TYPE_URL: &str = "/ibc.core.connection.v1.MsgConnectionOpenTry";

/// Per our convention, this message is sent to chain B.
/// The handler will check proofs of chain A.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgConnectionOpenTry {
    /// ClientId on B that the connection is being opened for
    pub client_id_on_b: ClientId,
    /// ClientState of client tracking chain B on chain A
    pub client_state_of_b_on_a: Any,
    /// ClientId, ConnectionId and prefix of chain A
    pub counterparty: Counterparty,
    /// Versions supported by chain A
    pub versions_on_a: Vec<Version>,
    /// proof of ConnectionEnd stored on Chain A during ConnOpenInit
    pub proof_conn_end_on_a: CommitmentProofBytes,
    /// proof that chain A has stored ClientState of chain B on its client
    pub proof_client_state_of_b_on_a: CommitmentProofBytes,
    /// proof that chain A has stored ConsensusState of chain B on its client
    pub proof_consensus_state_of_b_on_a: CommitmentProofBytes,
    /// Height at which all proofs in this message were taken
    pub proofs_height_on_a: Height,
    /// height of latest header of chain A that updated the client on chain B
    pub consensus_height_of_b_on_a: Height,
    pub delay_period: Duration,
    pub signer: Signer,

    #[deprecated(since = "0.22.0")]
    /// Only kept here for proper conversion to/from the raw type
    previous_connection_id: String,
}

impl Msg for MsgConnectionOpenTry {
    type Raw = RawMsgConnectionOpenTry;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgConnectionOpenTry> for MsgConnectionOpenTry {}

impl TryFrom<RawMsgConnectionOpenTry> for MsgConnectionOpenTry {
    type Error = ConnectionError;

    fn try_from(msg: RawMsgConnectionOpenTry) -> Result<Self, Self::Error> {
        let counterparty_versions = msg
            .counterparty_versions
            .into_iter()
            .map(Version::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        if counterparty_versions.is_empty() {
            return Err(ConnectionError::EmptyVersions);
        }

        // We set the deprecated `previous_connection_id` field so that we can
        // properly convert `MsgConnectionOpenTry` into its raw form
        #[allow(deprecated)]
        Ok(Self {
            previous_connection_id: msg.previous_connection_id,
            client_id_on_b: msg
                .client_id
                .parse()
                .map_err(ConnectionError::InvalidIdentifier)?,
            client_state_of_b_on_a: msg
                .client_state
                .ok_or(ConnectionError::MissingClientState)?,
            counterparty: msg
                .counterparty
                .ok_or(ConnectionError::MissingCounterparty)?
                .try_into()?,
            versions_on_a: counterparty_versions,
            proof_conn_end_on_a: msg
                .proof_init
                .try_into()
                .map_err(|_| ConnectionError::InvalidProof)?,
            proof_client_state_of_b_on_a: msg
                .proof_client
                .try_into()
                .map_err(|_| ConnectionError::InvalidProof)?,
            proof_consensus_state_of_b_on_a: msg
                .proof_consensus
                .try_into()
                .map_err(|_| ConnectionError::InvalidProof)?,
            proofs_height_on_a: msg
                .proof_height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or(ConnectionError::MissingProofHeight)?,
            consensus_height_of_b_on_a: msg
                .consensus_height
                .and_then(|raw_height| raw_height.try_into().ok())
                .ok_or(ConnectionError::MissingConsensusHeight)?,
            delay_period: Duration::from_nanos(msg.delay_period),
            signer: msg.signer.parse().map_err(ConnectionError::Signer)?,
        })
    }
}

impl From<MsgConnectionOpenTry> for RawMsgConnectionOpenTry {
    fn from(msg: MsgConnectionOpenTry) -> Self {
        #[allow(deprecated)]
        RawMsgConnectionOpenTry {
            client_id: msg.client_id_on_b.as_str().to_string(),
            previous_connection_id: msg.previous_connection_id,
            client_state: Some(msg.client_state_of_b_on_a),
            counterparty: Some(msg.counterparty.into()),
            delay_period: msg.delay_period.as_nanos() as u64,
            counterparty_versions: msg.versions_on_a.iter().map(|v| v.clone().into()).collect(),
            proof_height: Some(msg.proofs_height_on_a.into()),
            proof_init: msg.proof_conn_end_on_a.into(),
            proof_client: msg.proof_client_state_of_b_on_a.into(),
            proof_consensus: msg.proof_consensus_state_of_b_on_a.into(),
            consensus_height: Some(msg.consensus_height_of_b_on_a.into()),
            signer: msg.signer.to_string(),
        }
    }
}
