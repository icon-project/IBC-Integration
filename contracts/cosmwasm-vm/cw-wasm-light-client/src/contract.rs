use common::traits::AnyTypes;
use cosmwasm_schema::cw_serde;
use cw_common::cw_println;

use cw_light_client_common::traits::IQueryHandler;

use crate::constants::CLIENT_ID;
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

    let mut context = CwContext::new(deps, env);
    cw_println!(context.api(), "[WasmClient]: Contract Init Called");
    let client_state = context.get_client_state(CLIENT_ID)?;
    context.insert_blocknumber_at_height(CLIENT_ID, client_state.latest_height)?;
    context.insert_timestamp_at_height(CLIENT_ID, client_state.latest_height)?;
    cw_println!(context.api(), "[WasmClient]: Contract Init Complete");

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
    cw_println!(
        deps_mut.api,
        "[WasmClient]: Contract Execute Called with {:?}",
        msg
    );
    match msg {
        ExecuteMsg::VerifyMembership(msg) => {
            cw_println!(deps_mut.api, "[WasmClient]: Verify Membership called");
            let height = msg.height.revision_height;
            let client_id = CLIENT_ID;
            let proofs_decoded =
                MerkleProofs::decode(msg.proof.as_slice()).map_err(ContractError::DecodeError)?;
            let path = hex::decode(msg.path.key_path.join("")).unwrap();

            QueryHandler::verify_membership(
                deps_mut.as_ref(),
                client_id,
                height,
                msg.delay_time_period,
                msg.delay_block_period,
                &proofs_decoded.proofs,
                &msg.value,
                &path,
            )?;
            cw_println!(deps_mut.api, "[WasmClient]: Verify Membership Complete");

            Ok(to_binary(&ContractResult::success()).map_err(ContractError::Std)?)
        }
        ExecuteMsg::VerifyNonMembership(msg) => {
            cw_println!(deps_mut.api, "[WasmClient]: Verify Non Membership Called");
            let height = msg.height.revision_height;
            let client_id = CLIENT_ID;
            let proofs_decoded =
                MerkleProofs::decode(msg.proof.as_slice()).map_err(ContractError::DecodeError)?;
            let path = hex::decode(msg.path.key_path.join("")).unwrap();

            QueryHandler::verify_non_membership(
                deps_mut.as_ref(),
                client_id,
                height,
                msg.delay_time_period,
                msg.delay_block_period,
                &proofs_decoded.proofs,
                &path,
            )?;
            cw_println!(deps_mut.api, "[WasmClient]: Verify Non Membership Complete");

            Ok(to_binary(&ContractResult::success()).map_err(ContractError::Std)?)
        }
        ExecuteMsg::VerifyClientMessage(msg) => match msg.client_message {
            crate::msg::ClientMessageRaw::Header(wasmheader) => {
                let context = CwContext::new(deps_mut, env);

                cw_println!(context.api(), "[WasmClient]: Verify Clientmessage called");
                let mut client = IconClient::new(context);
                let header_any = Any::decode(&*wasmheader.data).unwrap();
                let header =
                    SignedHeader::from_any(header_any).map_err(ContractError::DecodeError)?;
                client.update_client(info.sender, CLIENT_ID, header)?;
                Ok(to_binary(&ContractResult::success()).map_err(ContractError::Std)?)
            }
            crate::msg::ClientMessageRaw::Misbehaviour(_) => unimplemented!(),
        },
        ExecuteMsg::UpdateState(msg) => {
            cw_println!(deps_mut.api, "Received Header {:?}", &msg);

            match msg.client_message {
                crate::msg::ClientMessageRaw::Header(wasmheader) => {
                    let context = CwContext::new(deps_mut, env);
                    let mut client = IconClient::new(context);
                    let header_any = Any::decode(&*wasmheader.data).unwrap();
                    let header =
                        SignedHeader::from_any(header_any).map_err(ContractError::DecodeError)?;
                    client.update_client(info.sender, CLIENT_ID, header)?;
                    Ok(to_binary(&ContractResult::success()).map_err(ContractError::Std)?)
                }
                crate::msg::ClientMessageRaw::Misbehaviour(_) => {
                    Ok(to_binary(&ContractResult::success()).map_err(ContractError::Std)?)
                }
            }
        }
        _ => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ClientTypeMsg(_) => todo!(),
        QueryMsg::GetLatestHeightsMsg(_) => todo!(),
        QueryMsg::ExportMetadata(msg) => {
            cw_println!(deps.api, "Export metadata called {:?}", &msg);
            let res = QueryHandler::get_genesis_metadata(deps.storage, CLIENT_ID);
            to_binary(&QueryResponse::genesis_metadata(res.ok()))
        }
        QueryMsg::Status(msg) => {
            cw_println!(deps.api, "Export metadata called {:?}", &msg);
            QueryHandler::get_client_status(deps)
        }
        QueryMsg::GetClientState {} => {
            to_binary(&QueryHandler::get_client_state(deps.storage, CLIENT_ID).unwrap())
        }
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
