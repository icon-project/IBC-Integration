use common::constants::ICON_CLIENT_TYPE;
use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};
use common::traits::AnyTypes;
use cosmwasm_schema::cw_serde;
use cw_common::ibc_types::IbcHeight;
use cw_common::{cw_println, to_checked_address};

#[cfg(feature = "mock")]
use crate::mock_client::MockClient;
use crate::query_handler::QueryHandler;
use common::icon::icon::types::v1::{MerkleProofs, SignedHeader};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use cw_common::client_response::{CreateClientResponse, UpdateClientResponse};
use cw_common::raw_types::Any;
use cw_common::types::VerifyChannelState;

use crate::constants::{CLIENT_STATE_HASH, CONSENSUS_STATE_HASH, HEIGHT};
use crate::context::CwContext;
use crate::light_client::IconClient;
use crate::msg::{ContractResult, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::traits::{Config, IContext, ILightClient};
use crate::ContractError;
use prost::Message;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-icon-light-client";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
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
) -> Result<Binary, ContractError> {
    match msg {
        ExecuteMsg::VerifyMembership(_) => todo!(),
        ExecuteMsg::VerifyNonMembership(_) => todo!(),
        ExecuteMsg::VerifyClientMessage(_) => todo!(),
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
                    let _update = client.update_client(info.sender, &client_id, header)?;
                    Ok(to_binary(&ContractResult::success()).unwrap())
                }
                crate::msg::ClientMessageRaw::Misbehaviour(_) => {
                    Ok(to_binary(&ContractResult::success()).unwrap())
                }
            }
        }
        ExecuteMsg::CheckSubstituteAndUpdateState(_) => todo!(),
        ExecuteMsg::VerifyUpgradeAndUpdateState(_) => todo!(),
    }
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

// pub fn to_packet_response(packet_data: &[u8]) -> Result<Binary, ContractError> {
//     let packet_data: PacketData = from_slice(packet_data).map_err(ContractError::Std)?;
//     let data = to_binary(&PacketDataResponse::from(packet_data)).map_err(ContractError::Std)?;
//     Ok(data)
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ClientTypeMsg(_) => todo!(),
        QueryMsg::GetLatestHeightsMsg(_) => todo!(),
        QueryMsg::ExportMetadata(_) => todo!(),
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
