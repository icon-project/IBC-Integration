use cosmwasm_std::{Deps, DepsMut, MessageInfo, Response, StdError};

use crate::{error::ContractError, state::CwCallservice, types::address::Address};

impl<'a> CwCallservice<'a> {
    pub fn query_owner(&self, deps: Deps) -> Result<Address, StdError> {
        let owner = self.owner().load(deps.storage)?;

        Ok(owner)
    }

    pub fn add_owner(&self, deps: DepsMut, owner: Address) -> Result<Response, ContractError> {
        self.owner().save(deps.storage, &owner)?;

        Ok(Response::new()
            .add_attribute("method", "add_owner")
            .add_attribute("owner", owner.to_string()))
    }

    pub fn update_owner(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        new_owner: Address,
    ) -> Result<Response, ContractError> {
        self.owner().update(
            deps.storage,
            |mut current_owner| -> Result<_, ContractError> {
                if info.sender == current_owner.to_string() {
                    if current_owner == new_owner {
                        Err(ContractError::OwnerAlreadyExist)
                    } else {
                        current_owner = new_owner.clone();
                        Ok(current_owner)
                    }
                } else {
                    Err(ContractError::Unauthorized {})
                }
            },
        )?;
        Ok(Response::new()
            .add_attribute("action", "update owner")
            .add_attribute("owner", new_owner.to_string()))
    }
}
