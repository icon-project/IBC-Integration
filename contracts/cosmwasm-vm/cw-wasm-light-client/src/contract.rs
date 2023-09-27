use common::constants::ICON_CLIENT_TYPE;
use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};
use common::traits::AnyTypes;
use cosmwasm_schema::cw_serde;
use cw_common::ibc_types::IbcHeight;
use cw_common::to_checked_address;

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
use crate::light_client::IconClient;
use crate::state::CwContext;
use crate::traits::{Config, IContext, ILightClient};
use crate::ContractError;
use cw_common::client_msg::{
    ExecuteMsg, InstantiateMsg, LightClientPacketMessage, QueryMsg, VerifyClientConsensusState,
    VerifyClientFullState, VerifyConnectionState,
};
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
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
        .map_err(|_e| ContractError::FailedToInitContract)?;
    let ibc_host = to_checked_address(deps.as_ref(), msg.ibc_host.as_ref());
    let config = Config::new(info.sender, ibc_host);
    let mut context = CwContext::new(deps, _env);
    context.insert_config(&config)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps_mut: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateClient {
            client_id,
            client_state,
            consensus_state,
        } => {
            let context = CwContext::new(deps_mut, _env);
            let mut client = IconClient::new(context);
            let client_state_any =
                Any::decode(client_state.as_slice()).map_err(ContractError::DecodeError)?;
            let consensus_state_any =
                Any::decode(consensus_state.as_slice()).map_err(ContractError::DecodeError)?;
            let client_state = ClientState::from_any(client_state_any.clone())
                .map_err(ContractError::DecodeError)?;
            let consensus_state = ConsensusState::from_any(consensus_state_any.clone())
                .map_err(ContractError::DecodeError)?;
            let update =
                client.create_client(info.sender, &client_id, client_state, consensus_state)?;

            let mut response = Response::new()
                .add_attribute(
                    CLIENT_STATE_HASH,
                    hex::encode(update.client_state_commitment),
                )
                .add_attribute(
                    CONSENSUS_STATE_HASH,
                    hex::encode(update.consensus_state_commitment),
                )
                .add_attribute(HEIGHT, update.height.to_string());

            let client_response = CreateClientResponse::new(
                ICON_CLIENT_TYPE.to_string(),
                IbcHeight::new(1, update.height).unwrap().to_string(),
                update.client_state_commitment.to_vec(),
                update.consensus_state_commitment.into(),
                client_state_any.encode_to_vec(),
                consensus_state_any.encode_to_vec(),
            );

            response.data = to_binary(&client_response).ok();

            Ok(response)
        }
        ExecuteMsg::UpdateClient {
            client_id,
            signed_header,
        } => {
            let context = CwContext::new(deps_mut, _env);
            let mut client = IconClient::new(context);
            let header_any = Any::decode(signed_header.as_slice()).unwrap();
            let header = SignedHeader::from_any(header_any).map_err(ContractError::DecodeError)?;
            let update = client.update_client(info.sender, &client_id, header)?;
            let response_data = to_binary(&UpdateClientResponse {
                height: to_ibc_height(update.height).map(|h| h.to_string())?,
                client_id,
                client_state_commitment: update.client_state_commitment.to_vec(),
                consensus_state_commitment: update.consensus_state_commitment.to_vec(),
                client_state_bytes: ClientState::any_from_value(&update.client_state_bytes)
                    .encode_to_vec(),
                consensus_state_bytes: ConsensusState::any_from_value(
                    &update.consensus_state_bytes,
                )
                .encode_to_vec(),
            })
            .map_err(ContractError::Std)?;
            Ok(Response::new()
                .add_attribute(
                    CLIENT_STATE_HASH,
                    hex::encode(update.client_state_commitment),
                )
                .add_attribute(
                    CONSENSUS_STATE_HASH,
                    hex::encode(update.consensus_state_commitment),
                )
                .add_attribute(HEIGHT, update.height.to_string())
                .set_data(response_data))
        }
        ExecuteMsg::Misbehaviour {
            client_id: _,
            misbehaviour: _,
        } => {
            todo!()
        }

        ExecuteMsg::UpgradeClient {
            upgraded_client_state: _,
            upgraded_consensus_state: _,
            proof_upgrade_client: _,
            proof_upgrade_consensus_state: _,
        } => {
            todo!()
        }
    }
}

pub fn validate_channel_state(
    client_id: &str,
    deps: Deps,
    state: &VerifyChannelState,
) -> Result<bool, ContractError> {
    let proofs_decoded =
        MerkleProofs::decode(state.proof.as_slice()).map_err(ContractError::DecodeError)?;
    let height = to_height_u64(&state.proof_height)?;
    let result = QueryHandler::verify_membership(
        deps,
        client_id,
        height,
        0,
        0,
        &proofs_decoded.proofs,
        &state.expected_counterparty_channel_end,
        &state.counterparty_chan_end_path,
    )?;
    Ok(result)
}

pub fn validate_connection_state(
    client_id: &str,
    deps: Deps,
    state: &VerifyConnectionState,
) -> Result<bool, ContractError> {
    let proofs_decoded =
        MerkleProofs::decode(state.proof.as_slice()).map_err(ContractError::DecodeError)?;
    let height = to_height_u64(&state.proof_height)?;

    let result = QueryHandler::verify_membership(
        deps,
        client_id,
        height,
        0,
        0,
        &proofs_decoded.proofs,
        &state.expected_counterparty_connection_end,
        &state.counterparty_conn_end_path,
    )?;
    Ok(result)
}

pub fn validate_client_state(
    client_id: &str,
    deps: Deps,
    state: &VerifyClientFullState,
) -> Result<bool, ContractError> {
    let proofs_decoded = MerkleProofs::decode(state.client_state_proof.as_slice())
        .map_err(ContractError::DecodeError)?;
    println!("starting validating client state");
    let height = to_height_u64(&state.proof_height)?;
    let result = QueryHandler::verify_membership(
        deps,
        client_id,
        height,
        0,
        0,
        &proofs_decoded.proofs,
        &state.expected_client_state,
        &state.client_state_path,
    )?;
    Ok(result)
}

pub fn validate_consensus_state(
    client_id: &str,
    deps: Deps,
    state: &VerifyClientConsensusState,
) -> Result<bool, ContractError> {
    let proofs_decoded = MerkleProofs::decode(state.consensus_state_proof.as_slice())
        .map_err(ContractError::DecodeError)?;
    let height = to_height_u64(&state.proof_height)?;
    let result = QueryHandler::verify_membership(
        deps,
        client_id,
        height,
        0,
        0,
        &proofs_decoded.proofs,
        &state.expected_conesenus_state,
        &state.conesenus_state_path,
    )?;
    Ok(result)
}

pub fn validate_next_seq_recv(
    deps: Deps,
    client_id: &str,
    state: &LightClientPacketMessage,
) -> Result<bool, ContractError> {
    let result = match state {
        LightClientPacketMessage::VerifyNextSequenceRecv {
            height,
            prefix: _,
            proof,
            root: _,
            seq_recv_path,
            sequence,
        } => {
            let proofs_decoded =
                MerkleProofs::decode(proof.as_slice()).map_err(ContractError::DecodeError)?;
            let height = to_height_u64(height)?;

            QueryHandler::verify_membership(
                deps,
                client_id,
                height,
                0,
                0,
                &proofs_decoded.proofs,
                sequence.to_be_bytes().as_ref(),
                seq_recv_path,
            )?
        }
        LightClientPacketMessage::VerifyPacketReceiptAbsence {
            height,
            prefix: _,
            proof,
            root: _,
            receipt_path,
        } => {
            let proofs_decoded =
                MerkleProofs::decode(proof.as_slice()).map_err(ContractError::DecodeError)?;
            let height = to_height_u64(height)?;

            QueryHandler::verify_non_membership(
                deps,
                client_id,
                height,
                0,
                0,
                &proofs_decoded.proofs,
                receipt_path,
            )?
        }
    };
    Ok(result)
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
        QueryMsg::VerifyMembership {
            client_id,
            message_bytes,
            proofs,
            path,
            height,
            delay_time_period,
            delay_block_period,
        } => {
            let proofs_decoded = MerkleProofs::decode(proofs.as_slice())
                .map_err(|e| StdError::GenericErr { msg: e.to_string() })?;
            let result = QueryHandler::verify_membership(
                deps,
                &client_id,
                height,
                delay_time_period,
                delay_block_period,
                &proofs_decoded.proofs,
                &message_bytes,
                &path,
            )
            .unwrap_or(false);
            to_binary(&result)
        }
        QueryMsg::VerifyNonMembership {
            client_id,

            proofs,
            path,
            height,
            delay_time_period,
            delay_block_period,
        } => {
            let proofs_decoded = MerkleProofs::decode(proofs.as_slice())
                .map_err(|e| StdError::GenericErr { msg: e.to_string() })?;
            let result = QueryHandler::verify_non_membership(
                deps,
                &client_id,
                height,
                delay_time_period,
                delay_block_period,
                &proofs_decoded.proofs,
                &path,
            )
            .unwrap_or(false);
            to_binary(&result)
        }
        QueryMsg::VerifyPacketData {
            client_id,
            verify_packet_data,
            // packet_data,
        } => {
            let proofs_decoded = MerkleProofs::decode(verify_packet_data.proof.as_slice())
                .map_err(|e| StdError::GenericErr { msg: e.to_string() })?;
            let height = to_height_u64(&verify_packet_data.height).unwrap();
            let result = QueryHandler::verify_membership(
                deps,
                &client_id,
                height,
                0,
                0,
                &proofs_decoded.proofs,
                &verify_packet_data.commitment,
                &verify_packet_data.commitment_path,
            )
            .unwrap_or(false);

            to_binary(&result)
        }
        QueryMsg::VerifyPacketAcknowledgement {
            client_id,
            verify_packet_acknowledge,
            // packet_data,
        } => {
            let proofs_decoded = MerkleProofs::decode(verify_packet_acknowledge.proof.as_slice())
                .map_err(|e| StdError::GenericErr { msg: e.to_string() })?;
            let height = to_height_u64(&verify_packet_acknowledge.height).unwrap();
            let result = QueryHandler::verify_membership(
                deps,
                &client_id,
                height,
                0,
                0,
                &proofs_decoded.proofs,
                &verify_packet_acknowledge.ack,
                &verify_packet_acknowledge.ack_path,
            )
            .unwrap_or(false);

            to_binary(&result)
        }
        QueryMsg::VerifyOpenConfirm {
            client_id,
            verify_connection_state,
            //  expected_response,
        } => {
            let result = validate_connection_state(&client_id, deps, &verify_connection_state)
                .unwrap_or(false);
            to_binary(&result)
        }
        QueryMsg::VerifyConnectionOpenTry(state) => {
            println!("checking all the valid state ");
            let client_valid =
                validate_client_state(&state.client_id, deps, &state.verify_client_full_state)
                    .unwrap_or(false);
            println!(" is valid clientstate  {client_valid:?}");

            let connection_valid =
                validate_connection_state(&state.client_id, deps, &state.verify_connection_state)
                    .unwrap_or(false);
            to_binary(&(client_valid && connection_valid))
        }
        QueryMsg::VerifyConnectionOpenAck(state) => {
            let connection_valid =
                validate_connection_state(&state.client_id, deps, &state.verify_connection_state)
                    .unwrap();
            let client_valid =
                validate_client_state(&state.client_id, deps, &state.verify_client_full_state)
                    .unwrap();

            to_binary(&(client_valid && connection_valid))
        }

        QueryMsg::VerifyChannel {
            verify_channel_state,
            // message_info,
            // endpoint,
        } => {
            // fix once we receive client id
            let result = validate_channel_state(
                &verify_channel_state.client_id,
                deps,
                &verify_channel_state,
            )
            .unwrap();

            to_binary(&result)
        }
        QueryMsg::PacketTimeout {
            client_id,
            next_seq_recv_verification_result,
        } => {
            let _sequence_valid =
                validate_next_seq_recv(deps, &client_id, &next_seq_recv_verification_result)
                    .unwrap();
            to_binary(&_sequence_valid)
        }
        QueryMsg::TimeoutOnCLose {
            client_id,
            verify_channel_state,
            next_seq_recv_verification_result,
        } => {
            let is_channel_valid =
                validate_channel_state(&client_id, deps, &verify_channel_state).unwrap();
            let _sequence_valid =
                validate_next_seq_recv(deps, &client_id, &next_seq_recv_verification_result)
                    .unwrap();

            to_binary(&(is_channel_valid && _sequence_valid))
        }
        QueryMsg::GetPreviousConsensusState { client_id, height } => {
            let res: Vec<u64> =
                QueryHandler::get_previous_consensus(deps.storage, height, client_id).unwrap();
            to_binary(&res)
        }
        QueryMsg::GetTimestampAtHeight {
            client_id: _,
            height: _,
        } => to_binary(&0_u64),
        QueryMsg::GetLatestConsensusState { client_id } => {
            let res = QueryHandler::get_latest_consensus_state(deps.storage, &client_id).unwrap();
            to_binary(&res)
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

pub fn get_light_client(context: CwContext<'_>) -> impl ILightClient<Error = ContractError> + '_ {
    #[cfg(feature = "mock")]
    return MockClient::new(context);
    #[cfg(not(feature = "mock"))]
    return IconClient::new(context);
}

pub fn ensure_owner(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
    let config = QueryHandler::get_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}
