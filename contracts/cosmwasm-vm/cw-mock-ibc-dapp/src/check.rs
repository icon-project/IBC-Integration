use crate::{error::ContractError, ibc::IBC_VERSION};

use cw_common::cw_types::CwOrder;

pub fn check_order(dapp_order: &CwOrder, order: &CwOrder) -> Result<(), ContractError> {
    if order != dapp_order {
        Err(ContractError::InvalidChannelOrder {})
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
