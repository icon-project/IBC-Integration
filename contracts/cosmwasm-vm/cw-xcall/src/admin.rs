use super::*;

impl<'a> CwCallService<'a> {
    pub fn query_admin(&self, store: &dyn Storage) -> Result<Address, ContractError> {
        let admin = self.admin().load(store)?;

        Ok(admin)
    }

    pub fn add_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        admin: Address,
    ) -> Result<Response, ContractError> {
        match self.owner().may_load(store)? {
            Some(owner) => {
                if info.sender == owner.to_string() {
                    self.admin().save(store, &admin)?;
                    Ok(Response::new()
                        .add_attribute("method", "add_admin")
                        .add_attribute("admin", admin.to_string()))
                } else {
                    Err(ContractError::Unauthorized {})
                }
            }
            None => Err(ContractError::Unauthorized {}),
        }
    }

    pub fn update_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        new_admin: Address,
    ) -> Result<Response, ContractError> {
        let owner = self.owner().load(store)?;

        self.admin()
            .update(store, |mut current_admin| -> Result<_, ContractError> {
                if info.sender == owner.to_string() {
                    if current_admin == new_admin {
                        Err(ContractError::AdminAlreadyExist)
                    } else {
                        current_admin = new_admin.clone();
                        Ok(current_admin)
                    }
                } else {
                    Err(ContractError::Unauthorized {})
                }
            })?;

        Ok(Response::new()
            .add_attribute("action", "update admin")
            .add_attribute("admin", new_admin.to_string()))
    }

    pub fn remove_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let owner = self.owner().load(store)?;

        if info.sender == owner.to_string() {
            self.admin().remove(store);
            Ok(Response::new().add_attribute("method", "remove_admin"))
        } else {
            Err(ContractError::Unauthorized {})
        }
    }
}
