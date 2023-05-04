use super::*;

impl<'a> CwCallService<'a> {
    pub fn query_admin(&self, store: &dyn Storage) -> Result<String, ContractError> {
        let admin = self
            .admin()
            .load(store)
            .map_err(|_| ContractError::AdminNotExist)?;

        Ok(admin)
    }

    pub fn add_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        admin: String,
    ) -> Result<Response, ContractError> {
        if admin.is_empty() {
            return Err(ContractError::AdminAddressCannotBeNull {});
        }

        let owner = self
            .owner()
            .load(store)
            .map_err(|_| ContractError::Unauthorized {})?;

        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }

        match self.admin().may_load(store)? {
            Some(_) => Err(ContractError::AdminAlreadyExist),
            None => {
                self.admin().save(store, &admin)?;
                Ok(Response::new()
                    .add_attribute("method", "add_admin")
                    .add_attribute("admin", admin.to_string()))
            }
        }
    }

    pub fn update_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        new_admin: String,
    ) -> Result<Response, ContractError> {
        if new_admin.is_empty() {
            return Err(ContractError::AdminAddressCannotBeNull {});
        }

        if !new_admin.to_string().chars().all(|x| x.is_alphanumeric()) {
            return Err(ContractError::InvalidAddress {
                address: new_admin.to_string(),
            });
        }

        self.ensure_owner(store, &info)?;

        self.admin()
            .update(store, |mut current_admin| -> Result<_, ContractError> {
                if current_admin == new_admin {
                    Err(ContractError::AdminAlreadyExist)
                } else {
                    current_admin = new_admin.clone();
                    Ok(current_admin)
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
        self.ensure_owner(store, &info)?;

        self.admin().remove(store);
        Ok(Response::new().add_attribute("method", "remove_admin"))
    }

    pub fn validate_address(api: &dyn Api, address: &str) -> Result<String, ContractError> {
        if !address.chars().all(|x| x.is_alphanumeric()) {
            return Err(ContractError::InvalidAddress {
                address: address.to_string(),
            });
        }

        let validated_address = api.addr_validate(address).map_err(ContractError::Std)?;

        Ok(validated_address.as_str().into())
    }
}
