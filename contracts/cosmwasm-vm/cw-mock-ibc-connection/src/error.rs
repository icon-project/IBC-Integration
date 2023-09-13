use super::*;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Admin Already Exist")]
    AdminAlreadyExist,
    #[error("OwnerAlreadyExist")]
    AdminNotExist,
    #[error("OnlyAdmin")]
    OnlyAdmin,
    #[error("AdminAddressCannotBeNull")]
    AdminAddressCannotBeNull {},
    #[error("OnlyXcallHandler")]
    OnlyXcallHandler {},
    #[error("InsufficientFunds")]
    InsufficientFunds,
}
