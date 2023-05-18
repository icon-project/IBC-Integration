use cw_common::errors::CwErrors;
use ibc::core::ics03_connection::error::ConnectionError;

use super::*;

#[derive(Error, Debug)]
/// This code defines an error enum called `ContractError` that represents various errors that can occur
/// in a contract. Each variant of the enum represents a different type of error and includes additional
/// information about the error, such as an error message or an error code. The `#[error]` attribute is
/// used to specify the format of the error message for each variant. Some variants also include
/// associated data, such as a `String` for the `InvalidClientId` variant or a `PortError` for the
/// `IbcPortError` variant. The `#[from]` attribute is used to automatically convert errors of the
/// specified type into the `StdError` variant of `ContractError`.
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidCommitmentKey")]
    InvalidCommitmentKey,

    #[error("InvalidConnectiontId {connection_id}")]
    InvalidConnectiontId { connection_id: String },

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
    IbcPortError { error: String },

    #[error("IbcPacketError {error}")]
    IbcPacketError { error: String },

    #[error("IbcChannelError {error}")]
    IbcChannelError { error: String },

    #[error("IbcConnectionError {error}")]
    IbcConnectionError { error: String },

    #[error("IbcClientError {error}")]
    IbcClientError { error: String },

    #[error("IbcValidationError {error}")]
    IbcValidationError { error: String },

    #[error("ERR_REPLY_ERROR|{code:?}|{msg:?}")]
    ReplyError { code: u64, msg: String },

    #[error("InsufficientBalance")]
    InsufficientBalance {},
    #[error("IbcDecodeError {error}")]
    IbcRawConversionError { error: String },
    #[error("FailedConversion")]
    FailedConversion,
}

/// This code defines an implementation of the `From` trait for the `ContractError` enum, which allows
/// instances of the `CwErrors` enum to be converted into instances of the `ContractError` enum. The
/// implementation matches on the different variants of the `CwErrors` enum and constructs an
/// appropriate variant of the `ContractError` enum based on the error information contained in the
/// `CwErrors` variant. For example, if the `CwErrors` variant is `FailedToCreateClientId`, the
/// implementation constructs an `IbcClientError` variant of the `ContractError` enum with a
/// `ClientIdentifierConstructor` error from the `ibc::core::ics02_client::error::ClientError` enum.
impl From<CwErrors> for ContractError {
    fn from(value: CwErrors) -> Self {
        match value {
            CwErrors::FailedToCreateClientId {
                client_type,
                counter,
                validation_error,
            } => Self::IbcClientError {
                error: CLIENT_ERROR.to_owned(),
            },
            CwErrors::InvalidClientId(client_id, err) => Self::IbcDecodeError {
                error: err.to_string(),
            },
            CwErrors::DecodeError { error } => Self::IbcDecodeError { error },
            CwErrors::FailedToConvertToPacketDataResponse(_) => Self::FailedConversion,
        }
    }
}

impl From<ChannelError> for ContractError {
    fn from(value: ChannelError) -> Self {
        ContractError::IbcChannelError {
            error: CHANNEL_ERROR.to_owned(),
        }
    }
}

impl From<PacketError> for ContractError {
    fn from(value: PacketError) -> Self {
        ContractError::IbcPacketError {
            error: PACKET_ERROR.to_owned(),
        }
    }
}

impl From<ConnectionError> for ContractError {
    fn from(value: ConnectionError) -> Self {
        ContractError::IbcConnectionError {
            error: CONNECTION_ERROR.to_owned(),
        }
    }
}

impl From<PortError> for ContractError {
    fn from(value: PortError) -> Self {
        ContractError::IbcPortError {
            error: PORT_ERROR.to_owned(),
        }
    }
}

impl From<ValidationError> for ContractError {
    fn from(value: ValidationError) -> Self {
        ContractError::IbcValidationError {
            error: PORT_ERROR.to_owned(),
        }
    }
}

impl From<ClientError> for ContractError {
    fn from(value: ClientError) -> Self {
        ContractError::IbcClientError {
            error: PORT_ERROR.to_owned(),
        }
    }
}

pub fn decode_error(decode_type: &str) -> String {
    return String::from("failed to decode ".to_owned() + decode_type);
}
