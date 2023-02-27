use cosmwasm_std::Storage;

use crate::{
    error::ContractError,
    state::CwCallservice,
    types::{call_request::CallRequest, request::CallServiceMessageRequest},
};

impl<'a> CwCallservice<'a> {
    pub fn query_last_sequence_no(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        let last_sequence = self.last_sequence_no().load(store)?;

        Ok(last_sequence)
    }

    pub fn increment_last_sequence_no(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        let sequence_no =
            self.last_sequence_no()
                .update(store, |mut seq| -> Result<_, ContractError> {
                    seq += 1;

                    Ok(seq)
                })?;
        Ok(sequence_no)
    }
    pub fn set_last_sequence_no(
        &self,
        store: &mut dyn Storage,
        sequence: u128,
    ) -> Result<u128, ContractError> {
        let req_id =
            self.last_sequence_no()
                .update(store, |mut seq| -> Result<_, ContractError> {
                    seq.clone_from(&sequence);
                    Ok(seq)
                })?;
        Ok(req_id)
    }

    pub fn query_last_request_id(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        let last_req_id = self.last_request_id().load(store)?;

        Ok(last_req_id)
    }

    pub fn increment_last_request_id(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        let req_id =
            self.last_request_id()
                .update(store, |mut req_id| -> Result<_, ContractError> {
                    req_id += 1;

                    Ok(req_id)
                })?;
        Ok(req_id)
    }
    pub fn set_last_request_id(
        &self,
        store: &mut dyn Storage,
        request_id: u128,
    ) -> Result<u128, ContractError> {
        let req_id =
            self.last_request_id()
                .update(store, |mut req_id| -> Result<_, ContractError> {
                    req_id.clone_from(&request_id);
                    Ok(req_id)
                })?;
        Ok(req_id)
    }

    pub fn set_call_request(
        &self,
        store: &mut dyn Storage,
        sequence: u128,
        call_request: CallRequest,
    ) -> Result<(), ContractError> {
        match self.requests().save(store, sequence, &call_request) {
            Ok(_) => Ok(()),
            Err(err) => Err(ContractError::Std(err)),
        }
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

    pub fn query_request(
        &self,
        store: &dyn Storage,
        sequence: u128,
    ) -> Result<CallRequest, ContractError> {
        match self.requests().may_load(store, sequence)? {
            Some(request) => Ok(request),
            None => Err(ContractError::InvalidSequenceId { id: sequence }),
        }
    }

    pub fn containes_request(&self, store: &dyn Storage, sequence: u128) -> bool {
        self.requests().load(store, sequence).is_ok()
    }

    pub fn remove_call_request(&self, store: &mut dyn Storage, sequence_no: u128) {
        self.requests().remove(store, sequence_no);
    }
}
