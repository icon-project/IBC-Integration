use cosmwasm_std::{Storage, MessageInfo};

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
    pub fn claim_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
        address: String,
    ) -> Result<(), ContractError> {
        let mut cap = self.get_capability(store, name.clone())?;
        cap.push(address);
        self.store_capability(store, name, cap)
    }

    pub fn authenticate_capability(&self, store: &mut dyn Storage,info: MessageInfo, name: Vec<u8> ) -> bool {
        let caller = info.sender.to_string();
        let cap = self.get_capability(store, name).unwrap();
        if cap.contains(&caller) {
            return true;
        }
        return false;
    }

    pub fn lookup_modules(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
    ) -> Result<Vec<String>, ContractError> {
        let capabilities = self.get_capability(store, name)?;
        match capabilities.len() > 0 {
            true => return Ok(capabilities),
            false => return Err(ContractError::Unauthorized {}),
        }
    }

    pub fn set_expected_time_per_block(
        &self,
        store: &mut dyn Storage,
        expected_time_per_block: u128,
    ) -> Result<(), ContractError> {
        match self.ibc_store().expected_time_per_block().save(store,  &expected_time_per_block) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::IbcDecodeError { error: format!("FailedToStore {}", error) }),
        }
    }

    pub fn get_expected_time_per_block(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        match self.ibc_store().expected_time_per_block().load(store) {
            Ok(result) => Ok(result),
            Err(error) => Err(ContractError::Std(error)),
        }
}
}
