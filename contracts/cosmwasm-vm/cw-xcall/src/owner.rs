use cosmwasm_std::{Deps, DepsMut, MessageInfo, Response, StdError};

use crate::{error::ContractError, state::CwCallservice, types::address::Address};

impl<'a> CwCallservice<'a> {
    pub fn query_owner(&self, deps: Deps) -> Result<Address, StdError> {
        let owner = self.owner().load(deps.storage)?;

        Ok(owner)
    }

    pub fn set_owner(
        &mut self,
        deps: DepsMut,
        info: MessageInfo,
        new_owner: Address,
    ) -> Result<Response, ContractError> {
        self.owner().update(
            deps.storage,
            |mut current_owner| -> Result<_, ContractError> {
                if info.sender.to_string() == current_owner.to_string() {
                    if current_owner == new_owner {
                        return Err(ContractError::OwnerAlreadyExist);
                    } else {
                        current_owner = new_owner.clone();
                        return Ok(current_owner);
                    }
                } else {
                    return Err(ContractError::Unauthorized {});
                }
            },
        )?;
        Ok(Response::new()
            .add_attribute("action", "update owner")
            .add_attribute("owner", new_owner.to_string()))
    }
}
