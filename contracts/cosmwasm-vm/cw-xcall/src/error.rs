use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("ERR_REPLY_ERROR|{code:?}|{msg:?}")]
    ReplyError { code: u64, msg: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("only unordered channels are supported")]
    OrderedChannel {},

    #[error("invalid IBC channel version. Got ({actual}), expected ({expected})")]
    InvalidVersion { actual: String, expected: String },

    #[error("Admin Already Exist")]
    AdminAlreadyExist,
    #[error("Owner Already Exist")]
    OwnerAlreadyExist,
    #[error("Admin Not Exist")]
    AdminNotExist,
    #[error("RollbackNotPossible")]
    RollbackNotPossible,
    #[error("MaxDataSizeExceeded")]
    MaxDataSizeExceeded,
    #[error("MaxRollbackSizeExceeded")]
    MaxRollbackSizeExceeded,
    #[error("NotExistRequestId {id}")]
    NotExistRequestId { id: u128 },
    #[error("InvalidRequestId {id}")]
    InvalidRequestId { id: u128 },
    #[error("RollbackNotEnabled")]
    RollbackNotEnabled,
    #[error("InvalidSequenceId {id}")]
    InvalidSequenceId { id: u128 },
}
