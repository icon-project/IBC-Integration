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
                error: ClientError::ClientIdentifierConstructor {
                    client_type: client_type.client_type(),
                    counter,
                    validation_error,
                },
            },
            CwErrors::InvalidClientId(client_id, err) => Self::IbcDecodeError {
                error: err.to_string(),
            },
            CwErrors::DecodeError { error } => Self::IbcDecodeError { error },
            CwErrors::FailedToConvertToPacketDataResponse(_) => todo!(),
        }
    }
}
