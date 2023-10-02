use common::traits::AnyTypes;
use cosmwasm_schema::cw_serde;
use cw_common::cw_println;
use cw_common::ibc_types::IbcHeight;
use cw_light_client_common::traits::IQueryHandler;

#[cfg(feature = "mock")]
use crate::mock_client::MockClient;
use crate::query_handler::QueryHandler;

use common::icon::icon::types::v1::{MerkleProofs, SignedHeader};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use cw_common::raw_types::Any;

use crate::context::CwContext;
use crate::light_client::IconClient;
use crate::msg::{ContractResult, ExecuteMsg, InstantiateMsg, QueryMsg, QueryResponse};
use crate::traits::{IContext, ILightClient};
use crate::ContractError;
use prost::Message;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-icon-light-client";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
        .map_err(|_e| ContractError::FailedToInitContract)?;
    let client_id = "08-wasm-0";
    let mut context = CwContext::new(deps, env);
    let client_state = context.get_client_state(client_id)?;
    context.insert_blocknumber_at_height(client_id, client_state.latest_height)?;
    context.insert_timestamp_at_height(client_id, client_state.latest_height)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps_mut: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let data = process_message(deps_mut, env, info, msg)?;
    let mut response = Response::default();
    response.data = Some(data);
    Ok(response)
}

fn process_message(
    deps_mut: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Binary, ContractError> {
    let result: Result<Binary, ContractError> = match msg {
        ExecuteMsg::VerifyMembership(msg) => {
            let height = msg.height.revision_height;
            let client_id = "08-wasm-0";
            let proofs_decoded =
                MerkleProofs::decode(msg.proof.as_slice()).map_err(ContractError::DecodeError)?;
            let path = hex::decode(msg.path.key_path.join("")).unwrap();

            let _ok = QueryHandler::verify_membership(
                deps_mut.as_ref(),
                client_id,
                height,
                msg.delay_time_period,
                msg.delay_block_period,
                &proofs_decoded.proofs,
                &msg.value,
                &path,
            )
            .unwrap();

            Ok(to_binary(&ContractResult::success()).unwrap())
        }
        ExecuteMsg::VerifyNonMembership(msg) => {
            let height = msg.height.revision_height;
            let client_id = "08-wasm-0";
            let proofs_decoded =
                MerkleProofs::decode(msg.proof.as_slice()).map_err(ContractError::DecodeError)?;
            let path = hex::decode(msg.path.key_path.join("")).unwrap();

            let _ok = QueryHandler::verify_non_membership(
                deps_mut.as_ref(),
                client_id,
                height,
                msg.delay_time_period,
                msg.delay_block_period,
                &proofs_decoded.proofs,
                &path,
            )
            .unwrap();

            Ok(to_binary(&ContractResult::success()).unwrap())
        }
        ExecuteMsg::VerifyClientMessage(msg) => match msg.client_message {
            crate::msg::ClientMessageRaw::Header(wasmheader) => {
                let context = CwContext::new(deps_mut, env);
                let mut client = IconClient::new(context);
                let header_any = Any::decode(&*wasmheader.data).unwrap();
                let header =
                    SignedHeader::from_any(header_any).map_err(ContractError::DecodeError)?;
                let client_id = "08-wasm-0";
                let _update = client.update_client(info.sender, client_id, header)?;
                Ok(to_binary(&ContractResult::success()).unwrap())
            }
            crate::msg::ClientMessageRaw::Misbehaviour(_) => unimplemented!(),
        },
        ExecuteMsg::CheckForMisbehaviour(_) => todo!(),
        ExecuteMsg::UpdateStateOnMisbehaviour(_) => todo!(),
        ExecuteMsg::UpdateState(msg) => {
            cw_println!(deps_mut.api, "Received Header {:?}", msg);
            let context = CwContext::new(deps_mut, env);
            let mut client = IconClient::new(context);
            match msg.client_message {
                crate::msg::ClientMessageRaw::Header(wasmheader) => {
                    let header_any = Any::decode(&*wasmheader.data).unwrap();
                    let header =
                        SignedHeader::from_any(header_any).map_err(ContractError::DecodeError)?;
                    let client_id = "08-wasm-0";
                    let _update = client.update_client(info.sender, client_id, header)?;
                    Ok(to_binary(&ContractResult::success()).unwrap())
                }
                crate::msg::ClientMessageRaw::Misbehaviour(_) => {
                    Ok(to_binary(&ContractResult::success()).unwrap())
                }
            }
        }
        ExecuteMsg::CheckSubstituteAndUpdateState(_) => todo!(),
        ExecuteMsg::VerifyUpgradeAndUpdateState(_) => todo!(),
    };
    Ok(result.unwrap())
}

fn to_height_u64(height: &str) -> Result<u64, ContractError> {
    let heights = height.split('-').collect::<Vec<&str>>();
    if heights.len() != 2 {
        return Err(ContractError::InvalidHeight);
    }
    heights[1]
        .parse::<u64>()
        .map_err(|_e| ContractError::InvalidHeight)
}

fn to_ibc_height(height: u64) -> Result<IbcHeight, ContractError> {
    IbcHeight::new(0, height).map_err(|_e| ContractError::InvalidHeight)
}

pub fn any_from_byte(bytes: &[u8]) -> Result<Any, ContractError> {
    let any = Any::decode(bytes).map_err(ContractError::DecodeError)?;
    Ok(any)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let client_id = "08-wasm-0";
    match msg {
        QueryMsg::ClientTypeMsg(_) => todo!(),
        QueryMsg::GetLatestHeightsMsg(_) => todo!(),
        QueryMsg::ExportMetadata(_) => {
            let res = QueryHandler::get_genesis_metadata(deps.storage, client_id);
            to_binary(&QueryResponse::genesis_metadata(Some(res)))
        }
        QueryMsg::Status(_) => todo!(),
    }
}

#[cw_serde]
pub struct MigrateMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
        .map_err(ContractError::Std)?;
    Ok(Response::default().add_attribute("migrate", "successful"))
}
