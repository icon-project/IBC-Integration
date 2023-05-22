pub mod contract;
pub mod errors;
pub mod helper;
pub mod msg;
pub mod state;
pub mod types;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdError, StdResult, Storage, SubMsg, WasmMsg,
};

pub use contract::*;
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
pub use errors::*;
pub use helper::*;
use msg::QueryMsg;
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
        ExecuteMsg::XCallMessage { data } => Ok(Response::new()
            .add_attribute("action", "success execute call")
            .set_data(data)),
        ExecuteMsg::SuccessCall {} => {
            let resukt = call_service.increment_sequence(deps.storage)?;
            Ok(Response::new().add_attribute("sequence", resukt.to_string()))
        }
        ExecuteMsg::FailureCall {} => Err(ContractError::ModuleAddressNotFound),
        ExecuteMsg::TestCall {
            success_addr,
            fail_addr,
        } => {
            let success = ExecuteMsg::SuccessCall {};
            let fail = ExecuteMsg::FailureCall {};
            let success_wasm = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: success_addr,
                msg: to_binary(&success).map_err(ContractError::Std)?,
                funds: info.funds.clone(),
            });
            let fail_wasm = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: fail_addr,
                msg: to_binary(&fail).map_err(ContractError::Std)?,
                funds: info.funds.clone(),
            });
            let submessages = vec![
                SubMsg {
                    msg: success_wasm.clone(),
                    gas_limit: None,
                    id: 2,
                    reply_on: cosmwasm_std::ReplyOn::Never,
                },
                SubMsg {
                    msg: fail_wasm,
                    gas_limit: None,
                    id: 6,
                    reply_on: cosmwasm_std::ReplyOn::Never,
                },
                SubMsg {
                    msg: success_wasm,
                    gas_limit: None,
                    id: 2,
                    reply_on: cosmwasm_std::ReplyOn::Never,
                },
            ];

            Ok(Response::new().add_submessages(submessages))
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
