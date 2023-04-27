pub mod contract;
pub mod errors;
pub mod helper;
pub mod state;
pub mod types;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_binary, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    Storage, WasmMsg,
};

pub use contract::*;
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
pub use errors::*;
pub use helper::*;
use state::CwMockService;
use thiserror::Error;
use types::InstantiateMsg;
pub use types::*;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let call_service = CwMockService::default();

    call_service.instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let call_service = CwMockService::default();
    match msg {
        ExecuteMsg::SendCallMessage { to, data, rollback } => {
            call_service.send_call_message(deps, info, to, data, rollback)
        }
        ExecuteMsg::HandleCallMessage { from, data } => {
            call_service.handle_call_message(deps, info, from, data)
        }
    }
}
