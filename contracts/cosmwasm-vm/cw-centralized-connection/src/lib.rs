pub mod contract;
pub mod errors;
pub mod helper;
pub mod msg;
pub mod state;
pub mod types;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Reply,
    Response, StdError, StdResult, Storage, SubMsg, WasmMsg,
};

pub use contract::*;
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
pub use errors::*;
pub use helper::*;
use msg::{ExecuteMsg, MigrateMsg, QueryMsg};
use state::CwCentralizedConnection;
use thiserror::Error;
pub use types::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut centralized_connection = CwCentralizedConnection::default();

    centralized_connection.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut centralized_connection = CwCentralizedConnection::default();
    match msg {
        ExecuteMsg::SendMessage { to, sn, msg } => {
            centralized_connection.send_message(deps, info, to, sn, msg)
        }
        ExecuteMsg::RecvMessage {
            src_network,
            conn_sn,
            msg,
        } => centralized_connection.recv_message(deps, info, src_network, conn_sn, msg),
        ExecuteMsg::ClaimFees {} => centralized_connection.claim_fees(deps, env, info),
        ExecuteMsg::RevertMessage { sn } => centralized_connection.revert_message(deps, info, sn),
        ExecuteMsg::SetAdmin { address } => centralized_connection.set_admin(deps, info, address),
        ExecuteMsg::SetFee {
            network_id,
            message_fee,
            response_fee,
        } => centralized_connection.set_fee(deps, info, network_id, message_fee, response_fee),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let centralized_connection = CwCentralizedConnection::default();
    match msg {
        QueryMsg::GetFee { nid, response } => to_binary(
            &centralized_connection
                .get_fee(deps.storage, nid, response)
                .unwrap(),
        ),

        QueryMsg::GetReceipt {
            src_network,
            conn_sn,
        } => to_binary(&centralized_connection.get_receipt(deps.storage, src_network, conn_sn)),

        QueryMsg::Admin {} => {
            to_binary(&centralized_connection.admin().load(deps.storage).unwrap())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let centralized_connection = CwCentralizedConnection::default();

    centralized_connection.reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let centralized_connection = CwCentralizedConnection::default();
    centralized_connection.migrate(deps, _env, _msg)
}
