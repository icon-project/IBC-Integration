use std::cell::RefCell;

use common::icon::icon::types::v1::MerkleNode;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    
};
use cw2::set_contract_version;
use ibc_proto::google::protobuf::Any;

use crate::error::ContractError;
use crate::light_client::IconClient;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CwContext, QueryHandler};
use crate::traits::{Config, IContext, ILightClient};
use prost::Message;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-icon-light-client";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let _ = set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
        .map_err(|_e| ContractError::FailedToInitContract)?;
    let config = Config::new(
        msg.src_network_id,
        msg.network_id,
        msg.network_type_id,
        info.sender,
    );
    let context = CwContext::new(RefCell::new(deps), _env);
    context.insert_config(&config)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps_mut: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let context = CwContext::new(RefCell::new(deps_mut), _env);
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
        } => {
            let (state_byte, update) =
                client.update_client(&client_id, any_from_byte(&signed_header))?;
            Ok(Response::new()
                .add_attribute("client_state_hash", hex::encode(state_byte))
                .add_attribute(
                    "consesus_state_commitment",
                    hex::encode(update.consensus_state_commitment),
                )
                .add_attribute("height", update.height.to_string()))
        }
        ExecuteMsg::VerifyMembership {
            client_id,
            message_bytes,
            proofs,
            path,
            height,
        } => {
           let proofs=proofs.into_iter().map(|bytes|{
              let node= MerkleNode::decode(bytes.as_slice()).unwrap();
              node
           }).collect::<Vec<MerkleNode>>();
           let result =client.verify_membership(
            &client_id, height, 0, 0, &proofs, &message_bytes, &path)?;
            Ok(Response::new().add_attribute("membership", result.to_string()))
        },
        ExecuteMsg::VerifyNonMembership {
            client_id,
           
            proofs,
            path,
            height,
        } => {
           let proofs=proofs.into_iter().map(|bytes|{
              let node= MerkleNode::decode(bytes.as_slice()).unwrap();
              node
           }).collect::<Vec<MerkleNode>>();
           let result =client.verify_non_membership(
            &client_id, height, 0, 0, &proofs, &path)?;
            Ok(Response::new().add_attribute("non-membership", result.to_string()))
        }
    }
}

pub fn any_from_byte(bytes: &[u8]) -> Any {
    let any = Any::decode(bytes).unwrap();
    any
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetClientState { client_id } => {
            let res = QueryHandler::get_client_state_any(deps.storage, &client_id).unwrap();
            to_binary(&res)
        }
        QueryMsg::GetConsensusState { client_id, height } => to_binary(
            &QueryHandler::get_consensus_state_any(deps.storage, &client_id, height).unwrap(),
        ),

        QueryMsg::GetLatestHeight { client_id } => {
            to_binary(&QueryHandler::get_latest_height(deps.storage, &client_id).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {}
