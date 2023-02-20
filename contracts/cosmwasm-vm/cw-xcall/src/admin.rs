use cosmwasm_std::{Deps, DepsMut, MessageInfo, Response, StdError};

use crate::{error::ContractError, state::CwCallservice, types::address::Address};

impl<'a> CwCallservice<'a> {
    pub fn query_admin(&self, deps: Deps) -> Result<Address, StdError> {
        let admin = self.admin().load(deps.storage)?;

        Ok(admin)
    }

    pub fn add_admin(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        admin: Address,
    ) -> Result<Response, ContractError> {
        match self.owner().may_load(deps.storage)? {
            Some(owner) => {
                if info.sender == owner.to_string() {
                    self.admin().save(deps.storage, &admin)?;
                } else {
                    return Err(ContractError::Unauthorized {});
                }
            }
            None => return Err(ContractError::Unauthorized {}),
        };

        Ok(Response::new()
            .add_attribute("method", "add_admin")
            .add_attribute("admin", admin.to_string()))
    }

    pub fn update_admin(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        new_admin: Address,
    ) -> Result<Response, ContractError> {
        let owner = self.owner().load(deps.storage)?;
        self.admin().update(
            deps.storage,
            |mut current_admin| -> Result<_, ContractError> {
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
            },
        )?;
        Ok(Response::new()
            .add_attribute("action", "update admin")
            .add_attribute("admin", new_admin.to_string()))
    }

    pub fn remove_admin(
        &self,
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let owner = self.owner().load(deps.storage)?;

        if info.sender == owner.to_string() {
            self.admin().remove(deps.storage);
            Ok(Response::new().add_attribute("method", "remove_admin"))
        } else {
            Err(ContractError::Unauthorized {})
        }
    }
}
