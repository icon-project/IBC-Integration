use super::*;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Invalid Address {address}")]
    InvalidAddress { address: String },
    #[error("Only Relayer(Admin)")]
    OnlyAdmin,
    #[error("Only XCall")]
    OnlyXCall,
    #[error("Duplicate Message")]
    DuplicateMessage,
    #[error("InsufficientFunds")]
    InsufficientFunds,
    #[error("ERR_REPLY_ERROR|{code:?}|{msg:?}")]
    ReplyError { code: u64, msg: String },
}
