use cosmwasm_std::{ensure, to_binary, Addr, Deps, QuerierWrapper};

use crate::{
    error::ContractError,
    state::{CwCallservice, MAX_DATA_SIZE, MAX_ROLLBACK_SIZE},
    types::{message, request::CallServiceMessageRequest},
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

    pub fn ensure_request_not_null(
        &self,
        req_id: u128,
        message: &CallServiceMessageRequest,
    ) -> Result<(), ContractError> {
        let data = to_binary(message).unwrap();
        ensure!(
            !(data.is_empty()),
            ContractError::InvalidRequestId { id: req_id }
        );

        Ok(())
    }
}

fn is_contract(querier: QuerierWrapper, address: Addr) -> bool {
    querier.query_wasm_contract_info(address).is_ok()
}
