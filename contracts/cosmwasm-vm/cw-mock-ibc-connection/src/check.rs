use crate::{
    error::ContractError,
    ibc::{APP_ORDER, IBC_VERSION},
};

use cw_common::cw_types::CwOrder;

pub fn check_order(order: &CwOrder) -> Result<(), ContractError> {
    if order != &APP_ORDER {
        Err(ContractError::UnOrderedChannel {})
    } else {
        Ok(())
    }
}

pub fn check_version(version: &str) -> Result<(), ContractError> {
    if version != IBC_VERSION {
        Err(ContractError::InvalidVersion {
            actual: version.to_string(),
            expected: IBC_VERSION.to_string(),
        })
    } else {
        Ok(())
    }
}
