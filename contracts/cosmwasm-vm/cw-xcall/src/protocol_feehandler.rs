use std::ops::Add;

use cosmwasm_std::{Deps, DepsMut, MessageInfo, Response};

use crate::{error::ContractError, state::CwCallservice, types::address::Address};

impl<'a> CwCallservice<'a> {
    pub fn setprotocol_feehandler(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        address: Address,
    ) -> Result<Response, ContractError> {
        self.ensure_admin_call_or_not(deps.storage, info.sender);
        let f = CwCallservice::new()
            .fee_handler()
            .save(deps.storage, &address);
       

       Ok(Response::new())

    }

    pub fn get_balance(&self, user : Address, tokenName: String) -> Result<Response,ContractError>{


Ok(Response::new())
    }

    pub fn get_protocol_feehandler(&self, deps: DepsMut) -> Result<Response, ContractError> {
        let get = CwCallservice::new().fee_handler().load(deps.storage);
        Ok(Response::new()
            .add_attribute("action", "get")
            .add_attribute("method", "get_protocol_feehandler"))
    }
}
