use cw_common::errors::CwErrors;
use ibc::core::ics03_connection::error::ConnectionError;

use super::*;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidClientId {client_id}")]
    InvalidClientId { client_id: String },

    #[error("InvalidClientType {client_type}")]
    InvalidClientType { client_type: String },

    #[error("InvalidNextClientSequence")]
    InvalidNextClientSequence {},

    #[error("IbcContextError {error}")]
    IbcContextError { error: String },

    #[error("IbcDecodeError {error}")]
    IbcDecodeError { error: String },

    #[error("IbcPortError {error}")]
    IbcPortError { error: PortError },

    #[error("IbcPacketError {error}")]
    IbcPacketError { error: PacketError },

    #[error("IbcChannelError {error}")]
    IbcChannelError { error: ChannelError },

    #[error("IbcConnectionError {error}")]
    IbcConnectionError { error: ConnectionError },

    #[error("IbcClientError {error}")]
    IbcClientError { error: ClientError },
}

impl From<CwErrors> for ContractError {
    fn from(value: CwErrors) -> Self {
        match value {
            CwErrors::FailedToCreateClientId {
                client_type,
                counter,
                validation_error,
            } => Self::IbcClientError {
                error: ClientError::ClientIdentifierConstructor {
                    client_type: client_type.client_type(),
                    counter,
                    validation_error,
                },
            },
            CwErrors::InvalidClientId(err) => Self::IbcDecodeError {
                error: err.to_string(),
            },
            CwErrors::DecodeError { error } => Self::IbcDecodeError { error },
        }
    }
}
