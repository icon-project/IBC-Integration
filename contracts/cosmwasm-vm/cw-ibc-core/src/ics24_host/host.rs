use cosmwasm_std::Storage;

use crate::ContractError;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn store_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
        address: Vec<String>,
    ) -> Result<(), ContractError> {
        match self.ibc_store().capabilities().save(store, name, &address) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::IbcDecodeError {
                error: format!("FailedToStore {}", error),
            }),
        }
    }

    pub fn get_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
    ) -> Result<Vec<String>, ContractError> {
        Ok(self
            .ibc_store()
            .capabilities()
            .load(store, name)
            .map_err(|_| ContractError::IbcDecodeError {
                error: "CapabilityNotFound".into(),
            })?)
    }
}
