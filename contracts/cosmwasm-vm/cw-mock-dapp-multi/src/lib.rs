pub mod contract;
pub mod errors;
pub mod helper;
pub mod msg;
pub mod state;
pub mod types;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdError, StdResult, Storage, WasmMsg,
};

pub use contract::*;
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
pub use errors::*;
pub use helper::*;
use msg::{ExecuteMsg, QueryMsg};
use state::{Connection, CwMockService};
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
        ExecuteMsg::HandleCallMessage {
            from,
            data,
            protocols,
        } => call_service.handle_call_message(info, from, data, protocols),
        ExecuteMsg::AddConnection {
            src_endpoint,
            dest_endpoint,
            network_id,
        } => {
            call_service.add_connection(
                deps.storage,
                network_id,
                Connection {
                    src_endpoint,
                    dest_endpoint,
                },
            )?;
            Ok(Response::new().add_attribute("action", "add_connection"))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let call_service = CwMockService::default();
    match msg {
        QueryMsg::GetSequence {} => to_binary(&call_service.get_sequence(deps.storage).unwrap()),
    }
}
