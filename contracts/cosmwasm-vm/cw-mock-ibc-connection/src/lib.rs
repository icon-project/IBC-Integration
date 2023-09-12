pub mod admin;
pub mod assertion;
pub mod contract;
pub mod error;
pub mod fee;
pub mod msg;
pub mod send_message;
pub mod state;
pub mod types;

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::CwIbcConnection,
    types::storage_keys::StorageKey,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdError, StdResult, Storage,
};
use cw2::set_contract_version;

use cw_storage_plus::Item;
use thiserror::Error;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let call_service = CwIbcConnection::default();

    call_service.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut call_service = CwIbcConnection::default();

    call_service.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let call_service = CwIbcConnection::default();

    call_service.query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cw_serde]
pub struct MigrateMsg {}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let call_service = CwIbcConnection::default();
    call_service.migrate(deps, env, msg)
}
