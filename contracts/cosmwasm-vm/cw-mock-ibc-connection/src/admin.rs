use cosmwasm_std::{MessageInfo, Response, Storage};

use crate::{error::ContractError, state::CwIbcConnection};

impl<'a> CwIbcConnection<'a> {
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
        self.admin().save(store, &admin)?;
        Ok(Response::new()
            .add_attribute("method", "add_admin")
            .add_attribute("admin", admin.to_string()))
    }

}
