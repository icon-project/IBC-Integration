use cosmwasm_std::StdError;
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
}
