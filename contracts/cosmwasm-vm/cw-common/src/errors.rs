use crate::types::ClientType;
use common::ibc::core::ics24_host::error::ValidationError;
use cosmwasm_std::StdError;

#[derive(Debug)]
pub enum CwErrors {
    FailedToCreateClientId {
        client_type: ClientType,
        counter: u64,
        validation_error: ValidationError,
    },
    InvalidClientId(String, ValidationError),
    DecodeError {
        error: String,
    },
    FailedToConvertToPacketDataResponse(StdError),
}
