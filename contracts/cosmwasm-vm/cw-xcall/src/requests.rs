use cosmwasm_std::{Deps, DepsMut, Storage};

use crate::{
    error::ContractError, state::CwCallservice, types::request::CallServiceMessageRequest,
};

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

    pub fn contains_request(
        &self,
        store: &dyn Storage,
        request_id: u128,
    ) -> Result<(), ContractError> {
        match self.message_request().has(store, request_id) {
            true => Ok(()),
            false => Err(ContractError::InvalidRequestId { id: request_id }),
        }
    }

    pub fn query_message_request(
        &self,
        store: &dyn Storage,
        request_id: u128,
    ) -> Result<CallServiceMessageRequest, ContractError> {
        match self.message_request().load(store, request_id) {
            Ok(result) => Ok(result),
            Err(err) => Err(ContractError::Std(err)),
        }
    }

    pub fn insert_request(
        &self,
        store: &mut dyn Storage,
        request_id: u128,
        value: CallServiceMessageRequest,
    ) -> Result<(), ContractError> {
        match self.message_request().save(store, request_id, &value) {
            Ok(_) => Ok(()),
            Err(err) => Err(ContractError::Std(err)),
        }
    }

    pub fn remove_request(&self, store: &mut dyn Storage, request_id: u128) {
        self.message_request().remove(store, request_id);
    }
}
