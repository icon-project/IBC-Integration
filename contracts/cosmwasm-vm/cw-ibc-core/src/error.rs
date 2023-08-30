use common::ibc::core::ics03_connection::error::ConnectionError;
use cw_common::errors::CwErrors;
use hex::FromHexError;
use prost::DecodeError;

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
    IbcDecodeError { error: DecodeError },

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

    #[error("IbcValidationError {error}")]
    IbcValidationError { error: ValidationError },

    #[error("ERR_REPLY_ERROR|{code:?}|{msg:?}")]
    ReplyError { code: u64, msg: String },

    #[error("InsufficientBalance")]
    InsufficientBalance {},
    #[error("IbcDecodeError {error}")]
    IbcRawConversionError { error: String },
    #[error("FailedConversion")]
    FailedConversion,

    #[error("Light Client Validation failed for {0}")]
    LightClientValidationFailed(String),

    #[error("Invalid EventType in {event} {event_type}")]
    InvalidEventType { event: String, event_type: String },

    #[error("InvalidHeight")]
    InvalidHeight,

    #[error("PacketNotExpired")]
    PacketNotExpired,
    #[error("CallAlreadyInProgress")]
    CallAlreadyInProgress,
}

impl From<FromHexError> for ContractError {
    fn from(value: FromHexError) -> Self {
        ContractError::IbcDecodeError {
            error: DecodeError::new(value.to_string()), //  "Hex String Decode Failed".to_owned(),
        }
    }
}

impl From<prost::DecodeError> for ContractError {
    fn from(value: prost::DecodeError) -> Self {
        ContractError::IbcDecodeError {
            error: value, // "Decode Failed".to_owned(),
        }
    }
}

/// This code defines an implementation of the `From` trait for the `ContractError` enum, which allows
/// instances of the `CwErrors` enum to be converted into instances of the `ContractError` enum. The
/// implementation matches on the different variants of the `CwErrors` enum and constructs an
/// appropriate variant of the `ContractError` enum based on the error information contained in the
/// `CwErrors` variant. For example, if the `CwErrors` variant is `FailedToCreateClientId`, the
/// implementation constructs an `IbcClientError` variant of the `ContractError` enum with a
/// `ClientIdentifierConstructor` error from the `common::ibc::core::ics02_client::error::ClientError` enum.
impl From<CwErrors> for ContractError {
    fn from(value: CwErrors) -> Self {
        match value {
            CwErrors::FailedToCreateClientId {
                client_type: _,
                counter: _,
                validation_error,
            } => Self::IbcValidationError {
                error: validation_error,
            },
            CwErrors::InvalidClientId(_client_id, err) => Self::IbcValidationError { error: err },
            CwErrors::DecodeError { error } => Self::IbcDecodeError {
                error: DecodeError::new(error),
            },
            CwErrors::FailedToConvertToPacketDataResponse(_) => Self::FailedConversion,
        }
    }
}

impl From<ChannelError> for ContractError {
    fn from(value: ChannelError) -> Self {
        ContractError::IbcChannelError { error: value }
    }
}

impl From<PacketError> for ContractError {
    fn from(value: PacketError) -> Self {
        ContractError::IbcPacketError { error: value }
    }
}

impl From<ConnectionError> for ContractError {
    fn from(value: ConnectionError) -> Self {
        ContractError::IbcConnectionError { error: value }
    }
}

impl From<PortError> for ContractError {
    fn from(value: PortError) -> Self {
        ContractError::IbcPortError { error: value }
    }
}

impl From<ValidationError> for ContractError {
    fn from(value: ValidationError) -> Self {
        ContractError::IbcValidationError { error: value }
    }
}

impl From<ClientError> for ContractError {
    fn from(value: ClientError) -> Self {
        ContractError::IbcClientError { error: value }
    }
}
