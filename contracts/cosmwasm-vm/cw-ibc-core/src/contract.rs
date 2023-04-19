use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
#[allow(unused_imports)]
use cw2::set_contract_version;

use crate::context::CwIbcCoreContext;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
#[allow(dead_code)]
const CONTRACT_NAME: &str = "crates.io:cw-ibc-core";
#[allow(dead_code)]
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(unused_variables)]
impl<'a> CwIbcCoreContext<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        todo!()
    }

    pub fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        todo!()
    }
}
