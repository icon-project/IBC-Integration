use std::time::Duration;

use cosmwasm_std::{Storage, Timestamp};

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
    pub fn set_expected_time_per_block(
        &self,
        store: &mut dyn Storage,
        expected_time_per_block: u64,
    ) -> Result<(), ContractError> {
        self.ibc_store()
            .expected_time_per_block()
            .save(store, &expected_time_per_block)?;

        Ok(())
    }

    pub fn get_expected_time_per_block(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().expected_time_per_block().may_load(store)? {
            Some(time) => Ok(time),
            None => Err(ContractError::IbcDecodeError {
                error: "NotFound".to_string(),
            }),
        }
    }

    pub fn calculate_block_delay(timestamp: Timestamp) -> u64 {
        //TODO :Validate Block delay
        Duration::from_secs(timestamp.plus_seconds(300).seconds()).as_secs()
    }
}
