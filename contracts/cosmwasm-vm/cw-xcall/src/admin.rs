use cosmwasm_std::{Deps, DepsMut, MessageInfo, Response, StdError};

use crate::{error::ContractError, state::CwCallservice, types::address::Address};

impl<'a> CwCallservice<'a> {
    pub fn query_admin(&self, deps: Deps) -> Result<Address, StdError> {
        let admin = self.admin().load(deps.storage)?;

        Ok(admin)
    }

    pub fn set_admin(
        &mut self,
        deps: DepsMut,
        info: MessageInfo,
        new_admin: Address,
    ) -> Result<Response, ContractError> {
        let owner = self.owner().load(deps.storage)?;
        self.admin().update(
            deps.storage,
            |mut current_admin| -> Result<_, ContractError> {
                if info.sender.to_string() == owner.to_string() {
                    if current_admin == new_admin {
                        Err(ContractError::OwnerAlreadyExist)
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
}
