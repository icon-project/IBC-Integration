use crate::ibc::core::ics02_client::error as client_error;
use crate::ibc::core::ics03_connection::version::Version;
use crate::ibc::core::ics24_host::error::ValidationError;
use crate::ibc::core::ics24_host::identifier::{ClientId, ConnectionId};
use crate::ibc::signer::SignerError;
use crate::ibc::timestamp::TimestampOverflowError;
use crate::ibc::Height;

use displaydoc::Display;
use prost::alloc::string::String;

#[derive(Debug, Display)]
pub enum ConnectionError {
    /// client error: `{0}`
    Client(client_error::ClientError),
    /// connection state is unknown: `{state}`
    InvalidState { state: i32 },
    /// connection end for identifier `{connection_id}` was never initialized
    ConnectionMismatch { connection_id: ConnectionId },
    /// consensus height claimed by the client on the other party is too advanced: `{target_height}` (host chain current height: `{current_height}`)
    InvalidConsensusHeight {
        target_height: Height,
        current_height: Height,
    },
    /// identifier error: `{0}`
    InvalidIdentifier(ValidationError),
    /// ConnectionEnd domain object could not be constructed out of empty proto object
    EmptyProtoConnectionEnd,
    /// empty supported versions
    EmptyVersions,
    /// empty supported features
    EmptyFeatures,
    /// no common version
    NoCommonVersion,
    /// version \"`{version}`\" not supported
    VersionNotSupported { version: Version },
    /// missing proof height
    MissingProofHeight,
    /// missing consensus height
    MissingConsensusHeight,
    /// invalid connection proof error
    InvalidProof,
    /// verifying connection state error: `{0}`
    VerifyConnectionState(client_error::ClientError),
    /// invalid signer error: `{0}`
    Signer(SignerError),
    /// no connection was found for the previous connection id provided `{connection_id}`
    ConnectionNotFound { connection_id: ConnectionId },
    /// invalid counterparty
    InvalidCounterparty,
    /// missing counterparty
    MissingCounterparty,
    /// missing client state
    MissingClientState,
    /// the consensus proof verification failed (height: `{height}`), client error: `{client_error}`
    ConsensusStateVerificationFailure {
        height: Height,
        client_error: client_error::ClientError,
    },
    /// the client state proof verification failed for client id `{client_id}`, client error: `{client_error}`
    ClientStateVerificationFailure {
        // TODO: use more specific error source
        client_id: ClientId,
        client_error: client_error::ClientError,
    },
    /// invalid client state: `{reason}`
    InvalidClientState { reason: String },
    /// other error: `{description}`
    Other { description: String },
    /// some error
    NotEnoughTimeElapsed {
        current_host_time: crate::ibc::timestamp::Timestamp,
        earliest_valid_time: crate::ibc::timestamp::Timestamp,
    },
    /// some error
    NotEnoughBlocksElapsed {
        current_host_height: Height,
        earliest_valid_height: Height,
    },
    /// some error
    TimestampOverflow(TimestampOverflowError),
    /// Connection already exists `{0}`
    ConnectionExists(String),
}

#[cfg(feature = "std")]
impl std::error::Error for ConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Self::Client(e) => Some(e),
            Self::InvalidIdentifier(e) => Some(e),
            Self::VerifyConnectionState(e) => Some(e),
            Self::Signer(e) => Some(e),
            Self::ConsensusStateVerificationFailure {
                client_error: e, ..
            } => Some(e),
            Self::ClientStateVerificationFailure {
                client_error: e, ..
            } => Some(e),
            _ => None,
        }
    }
}
