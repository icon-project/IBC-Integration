use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::CwCallservice,
    types::address::Address,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-xcall";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwCallservice<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        self.add_owner(deps, Address::from_string(info.sender.to_string()))?;

        Ok(Response::new().add_attribute("method", "instantiate"))
    }

    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        unimplemented!()
    }
}
