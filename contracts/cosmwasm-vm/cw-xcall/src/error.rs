use std::fmt;

use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("only unordered channels are supported")]
    OrderedChannel {},

    #[error("invalid IBC channel version. Got ({actual}), expected ({expected})")]
    InvalidVersion { actual: String, expected: String },
}

#[derive(Error, Debug)]
pub enum CallServiceError {
    #[error("Admin Already Exist")]
    AdminAlreadyExist,
    #[error("Owner Already Exist")]
    OwnerAlreadyExist,
    #[error("Admin Not Exist")]
    AdminNotExist,
}
