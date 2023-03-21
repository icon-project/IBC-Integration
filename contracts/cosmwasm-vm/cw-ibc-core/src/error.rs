use cosmwasm_std::StdError;
use ibc::core::{ics04_channel::error::PacketError, ics05_port::error::PortError, ContextError};
use thiserror::Error;

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
    IbcContextError { error: ContextError },

    #[error("IbcDecodeError {error}")]
    IbcDecodeError { error: String },

    #[error("IbcPortError {error}")]
    IbcPortError { error: PortError },

    #[error("IbcPackketError {error}")]
    IbcPackketError { error: PacketError },
}
