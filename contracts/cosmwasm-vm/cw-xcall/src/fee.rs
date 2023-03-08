use super::*;

impl<'a> CwCallService<'a> {
    pub fn set_protocol_fee(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        value: u128,
    ) -> Result<Response, ContractError> {
        self.ensure_admin(deps.storage, info.sender)?;
        self.add_fee(deps.storage, value)?;

        Ok(Response::new().add_attribute("method", "set_protocolfee"))
    }

    pub fn get_protocol_fee(&self, deps: Deps) -> u128 {
        self.query_fee(deps.storage).unwrap()
    }
}

impl<'a> CwCallService<'a> {
    fn add_fee(&self, store: &mut dyn Storage, value: u128) -> Result<(), ContractError> {
        match self.fee().save(store, &value) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    fn query_fee(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        match self.fee().load(store) {
            Ok(value) => Ok(value),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}
