use super::*;

use crate::{
    error::ContractError,
    state::{CwCallService, MAX_DATA_SIZE, MAX_ROLLBACK_SIZE},
    types::{call_request::CallRequest, request::CallServiceMessageRequest},
};

impl<'a> CwCallService<'a> {
    pub fn ensure_caller_is_contract_and_rollback_is_null(
        &self,
        deps: Deps,
        address: Addr,
        rollback: Option<Vec<u8>>,
    ) -> Result<(), ContractError> {
        ensure!(
            (is_contract(deps.querier, address) || rollback.is_none()),
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

    pub fn ensure_call_request_not_null(
        &self,
        sequence_no: u128,
        message: &CallRequest,
    ) -> Result<(), ContractError> {
        let data = to_binary(message).unwrap();
        ensure!(
            !(data.is_empty()),
            ContractError::InvalidSequenceId { id: sequence_no }
        );

        Ok(())
    }

    pub fn ensure_rollback_enabled(&self, enabled: bool) -> Result<(), ContractError> {
        ensure!(enabled, ContractError::RollbackNotEnabled);

        Ok(())
    }

    pub fn ensure_owner(
        &self,
        store: &dyn Storage,
        info: &MessageInfo,
    ) -> Result<(), ContractError> {
        let owner = self.owner().load(store)?;

        ensure_eq!(
            info.sender,
            owner.to_string(),
            ContractError::Unauthorized {}
        );

        Ok(())
    }
    pub fn ensure_admin(&self, store: &dyn Storage, address: Addr) -> Result<(), ContractError> {
        let admin = self.query_admin(store)?;
        ensure_eq!(admin.to_string(), address, ContractError::OnlyAdmin);

        Ok(())
    }
}

fn is_contract(querier: QuerierWrapper, address: Addr) -> bool {
    querier.query_wasm_contract_info(address).is_ok()
}
