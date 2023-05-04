use super::*;

impl<'a> CwCallService<'a> {
    pub fn query_owner(&self, store: &dyn Storage) -> Result<String, StdError> {
        let owner = self.owner().load(store)?;

        Ok(owner)
    }

    pub fn add_owner(
        &self,
        store: &mut dyn Storage,
        owner: String,
    ) -> Result<Response, ContractError> {
        match self.owner().may_load(store)? {
            Some(address) => {
                if address != owner {
                    self.owner().save(store, &owner)?;
                } else {
                    return Err(ContractError::OwnerAlreadyExist);
                }
            }
            None => {
                self.owner().save(store, &owner)?;
            }
        };

        Ok(Response::new()
            .add_attribute("method", "add_owner")
            .add_attribute("owner", owner.to_string()))
    }

    pub fn update_owner(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        new_owner: String,
    ) -> Result<Response, ContractError> {
        self.owner()
            .update(store, |mut current_owner| -> Result<_, ContractError> {
                if info.sender == current_owner {
                    if current_owner == new_owner {
                        Err(ContractError::OwnerAlreadyExist)
                    } else {
                        current_owner = new_owner.clone();
                        Ok(current_owner)
                    }
                } else {
                    Err(ContractError::Unauthorized {})
                }
            })?;

        Ok(Response::new()
            .add_attribute("action", "update owner")
            .add_attribute("owner", new_owner.to_string()))
    }
}
