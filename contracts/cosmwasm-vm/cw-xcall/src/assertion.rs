use cosmwasm_std::{ensure, Addr, Deps, Querier, QuerierWrapper};

use crate::{
    error::ContractError,
    state::{CwCallservice, MAX_DATA_SIZE, MAX_ROLLBACK_SIZE},
};

impl<'a> CwCallservice<'a> {
    pub fn ensure_caller_is_contract_and_rollback_is_null(
        &self,
        deps: Deps,
        address: Addr,
        rollback: &[u8],
    ) -> Result<(), ContractError> {
        ensure!(
            (is_contract(deps.querier, address) || rollback.is_empty()),
            ContractError::RollbackNotPossible
        );

        Ok(())
    }

    pub fn ensure_data_length(&self, data_len: usize) -> Result<(), ContractError> {
        ensure!(
            data_len <= MAX_DATA_SIZE as usize,
            ContractError::MaxDataSizeExceeded
        );

        Ok(())
    }

    pub fn ensure_rollback_length(&self, rollback: &[u8]) -> Result<(), ContractError> {
        ensure!(
            rollback.is_empty() || rollback.len() <= MAX_ROLLBACK_SIZE as usize,
            ContractError::MaxRollbackSizeExceeded
        );

        Ok(())
    }
}

fn is_contract(querier: QuerierWrapper, address: Addr) -> bool {
    match querier.query_wasm_contract_info(address) {
        Ok(_) => true,
        Err(_) => false,
    }
}
