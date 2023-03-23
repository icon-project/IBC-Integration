use std::cell::RefCell;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use ibc_proto::google::protobuf::Any;
use ibc_proto::protobuf::Protobuf;

use crate::error::ContractError;
use crate::light_client::IconClient;
use crate::msg::{CreateClientResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::CwContext;
use crate::traits::ILightClient;
use bytes::Buf;
use prost::Message;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-icon-light-client";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let context = CwContext::new(RefCell::new(deps), _env);
    let client = IconClient::new(&context);
    match msg {
        ExecuteMsg::CreateClient {
            client_id,
            client_state_bytes,
            consensus_state_bytes,
        } => {
            let client_state_any = any_from_byte(&client_state_bytes);
            let consensus_state_any = any_from_byte(&consensus_state_bytes);
            let (state_byte, update) =
                client.create_client(&client_id, client_state_any, consensus_state_any)?;

            Ok(Response::new()
                .add_attribute("client_state_hash", hex::encode(state_byte))
                .add_attribute(
                    "consesus_state_commitment",
                    hex::encode(update.consensus_state_commitment),
                )
                .add_attribute("height", update.height.to_string()))
        }
        ExecuteMsg::UpdateClient {
            client_id,
            signed_header,
        } => Ok(Response::new()),
        ExecuteMsg::VerifyMembership {
            message_bytes,
            proofs,
            height,
        } => Ok(Response::new()),
    }
}

pub fn any_from_byte(bytes: &[u8]) -> Any {
    let any = Any::decode(bytes).unwrap();
    any
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
