use super::*;

pub fn check_order(order: &IbcOrder) -> Result<(), ContractError> {
    if order != &APP_ORDER {
        Err(ContractError::OrderedChannel {})
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
