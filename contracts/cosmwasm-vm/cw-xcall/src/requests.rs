use cosmwasm_std::Storage;

use crate::{error::ContractError, state::CwCallservice, types::call_request::CallRequest};

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
}
