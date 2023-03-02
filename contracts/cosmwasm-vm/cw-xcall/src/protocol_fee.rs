use cosmwasm_std::{DepsMut, MessageInfo, Response, Storage};

use crate::{error::ContractError, state::CwCallservice};

impl<'a> CwCallservice<'a> {
    pub fn set_protocolfee(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        value: u128,
    ) -> Result<Response, ContractError> {
        self.ensure_admin(deps.storage, info.sender)?;
        self.ensure_positive_value(value)?;
        self.add_fee(deps.storage, value)?;

        Ok(Response::new().add_attribute("method", "set_protocolfee"))
    }

    pub fn get_protocolfee(&self, deps:DepsMut)-> u128{
self.query_fee( deps.storage).unwrap()
    }

    fn add_fee(&self, store: &mut dyn Storage, value: u128) -> Result<(), ContractError> {
        match self.fee().save(store, &value) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    fn query_fee(&self, store: &mut dyn Storage) -> Result<u128, ContractError>{
        match self.fee().load(store){
            Ok(value) => Ok(value),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}
