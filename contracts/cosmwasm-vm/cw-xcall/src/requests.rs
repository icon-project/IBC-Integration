use cosmwasm_std::{Deps, DepsMut};

use crate::{error::ContractError, state::CwCallservice};

impl<'a> CwCallservice<'a> {
    pub fn query_last_sequence_no(&self, deps: Deps) -> Result<u128, ContractError> {
        let last_sequence = self.last_sequence_no().load(deps.storage)?;

        Ok(last_sequence)
    }

    pub fn increment_last_sequence_no(&self, deps: DepsMut) -> Result<(), ContractError> {
        self.last_sequence_no()
            .update(deps.storage, |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            })?;
        Ok(())
    }
    pub fn set_last_sequence_no(&self, deps: DepsMut, sequence: u128) -> Result<(), ContractError> {
        self.last_sequence_no()
            .update(deps.storage, |mut seq| -> Result<_, ContractError> {
                seq.clone_from(&sequence);
                Ok(seq)
            })?;
        Ok(())
    }

    pub fn query_last_request_id(&self, deps: Deps) -> Result<u128, ContractError> {
        let last_req_id = self.last_request_id().load(deps.storage)?;

        Ok(last_req_id)
    }

    pub fn increment_last_request_id(&self, deps: DepsMut) -> Result<(), ContractError> {
        self.last_request_id()
            .update(deps.storage, |mut req_id| -> Result<_, ContractError> {
                req_id += 1;

                Ok(req_id)
            })?;
        Ok(())
    }
    pub fn set_last_request_id(
        &self,
        deps: DepsMut,
        request_id: u128,
    ) -> Result<(), ContractError> {
        self.last_request_id()
            .update(deps.storage, |mut req_id| -> Result<_, ContractError> {
                req_id.clone_from(&request_id);
                Ok(req_id)
            })?;
        Ok(())
    }
}
