use common::constants::ICON_CLIENT_TYPE;
use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};
use common::traits::AnyTypes;
use cosmwasm_schema::cw_serde;
use cw_common::ibc_types::IbcHeight;

use debug_print::debug_println;

#[cfg(feature = "mock")]
use crate::mock_client::MockClient;
use common::icon::icon::types::v1::{MerkleProofs, SignedHeader};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_slice, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw_common::client_response::{
    CreateClientResponse, LightClientResponse, PacketDataResponse, UpdateClientResponse,
};
use cw_common::raw_types::Any;
use cw_common::types::{PacketData, VerifyChannelState};

use crate::constants::{
    CLIENT_STATE_HASH, CLIENT_STATE_VALID, CONNECTION_STATE_VALID, CONSENSUS_STATE_HASH, HEIGHT,
    MEMBERSHIP, NON_MEMBERSHIP,
};
use crate::error::ContractError;
use crate::light_client::IconClient;
use crate::state::{CwContext, QueryHandler};
use crate::traits::{Config, IContext, ILightClient};
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
    let config = Config::new(info.sender, msg.ibc_host);
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
    let mut context = CwContext::new(deps_mut, _env);
    let mut client = IconClient::new(&mut context);
    match msg {
        ExecuteMsg::CreateClient {
            client_id,
            client_state,
            consensus_state,
        } => {
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
            debug_println!("[CreateClient]: create client called with id {}", client_id);

            Ok(response)
        }
        ExecuteMsg::UpdateClient {
            client_id,
            signed_header,
        } => {
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
        ExecuteMsg::VerifyMembership {
            client_id,
            message_bytes,
            proofs,
            path,
            height,
            delay_time_period,
            delay_block_period,
        } => {
            let proofs_decoded =
                MerkleProofs::decode(proofs.as_slice()).map_err(ContractError::DecodeError)?;
            let result = client.verify_membership(
                &client_id,
                height,
                delay_time_period,
                delay_block_period,
                &proofs_decoded.proofs,
                &message_bytes,
                &path,
            )?;
            Ok(Response::new().add_attribute(MEMBERSHIP, result.to_string()))
        }
        ExecuteMsg::VerifyNonMembership {
            client_id,

            proofs,
            path,
            height,
            delay_time_period,
            delay_block_period,
        } => {
            let proofs_decoded =
                MerkleProofs::decode(proofs.as_slice()).map_err(ContractError::DecodeError)?;
            let result = client.verify_non_membership(
                &client_id,
                height,
                delay_time_period,
                delay_block_period,
                &proofs_decoded.proofs,
                &path,
            )?;
            Ok(Response::new().add_attribute(NON_MEMBERSHIP, result.to_string()))
        }
        ExecuteMsg::VerifyPacketData {
            client_id,
            verify_packet_data,
            packet_data,
        } => {
            let proofs_decoded = MerkleProofs::decode(verify_packet_data.proof.as_slice())
                .map_err(ContractError::DecodeError)?;
            let height = to_height_u64(&verify_packet_data.height)?;
            let result = client.verify_membership(
                &client_id,
                height,
                0,
                0,
                &proofs_decoded.proofs,
                &verify_packet_data.commitment_path,
                &verify_packet_data.commitment,
            )?;
            let packet_data: PacketData = from_slice(&packet_data).map_err(ContractError::Std)?;
            let data =
                to_binary(&PacketDataResponse::from(packet_data)).map_err(ContractError::Std)?;

            Ok(Response::new()
                .add_attribute(MEMBERSHIP, result.to_string())
                .set_data(data))
        }
        ExecuteMsg::VerifyPacketAcknowledgement {
            client_id,
            verify_packet_acknowledge,
            packet_data,
        } => {
            let proofs_decoded = MerkleProofs::decode(verify_packet_acknowledge.proof.as_slice())
                .map_err(ContractError::DecodeError)?;
            let height = to_height_u64(&verify_packet_acknowledge.height)?;
            let result = client.verify_membership(
                &client_id,
                height,
                0,
                0,
                &proofs_decoded.proofs,
                &verify_packet_acknowledge.ack_path,
                &verify_packet_acknowledge.ack,
            )?;
            let packet_data: PacketData = from_slice(&packet_data).map_err(ContractError::Std)?;
            let data =
                to_binary(&PacketDataResponse::from(packet_data)).map_err(ContractError::Std)?;

            Ok(Response::new()
                .add_attribute(MEMBERSHIP, result.to_string())
                .set_data(data))
        }
        ExecuteMsg::VerifyOpenConfirm {
            client_id,
            verify_connection_state,
            expected_response,
        } => {
            let result = validate_connection_state(&client_id, &client, &verify_connection_state)?;
            let data = to_binary(&expected_response).map_err(ContractError::Std)?;

            Ok(Response::new()
                .add_attribute(MEMBERSHIP, result.to_string())
                .set_data(data))
        }
        ExecuteMsg::VerifyConnectionOpenTry(state) => {
            println!("checking all the valid state ");
            let client_valid =
                validate_client_state(&state.client_id, &client, &state.verify_client_full_state)?;
            println!(" is valid clientstate  {client_valid:?}");

            let connection_valid = validate_connection_state(
                &state.client_id,
                &client,
                &state.verify_connection_state,
            )?;
            println!("is  valid connection state {connection_valid:?}");

            Ok(Response::new()
                .add_attribute(CLIENT_STATE_VALID, client_valid.to_string())
                .add_attribute(CONNECTION_STATE_VALID, connection_valid.to_string())
                // .add_attribute(CONSENSUS_STATE_VALID, consensus_valid.to_string())
                .set_data(to_binary(&state.expected_response).map_err(ContractError::Std)?))
        }
        ExecuteMsg::VerifyConnectionOpenAck(state) => {
            let connection_valid = validate_connection_state(
                &state.client_id,
                &client,
                &state.verify_connection_state,
            )?;
            let client_valid =
                validate_client_state(&state.client_id, &client, &state.verify_client_full_state)?;

            Ok(Response::new()
                .add_attribute(CLIENT_STATE_VALID, client_valid.to_string())
                .add_attribute(CONNECTION_STATE_VALID, connection_valid.to_string())
                // .add_attribute(CONSENSUS_STATE_VALID, consensus_valid.to_string())
                .set_data(to_binary(&state.expected_response).map_err(ContractError::Std)?))
        }

        ExecuteMsg::VerifyChannel {
            verify_channel_state,
            message_info,
            endpoint,
        } => {
            // fix once we receive client id
            let result = validate_channel_state(
                &verify_channel_state.client_id,
                &client,
                &verify_channel_state,
            )?;

            let data = to_binary(&LightClientResponse {
                message_info,
                ibc_endpoint: endpoint,
            })
            .map_err(ContractError::Std)?;
            Ok(Response::new()
                .add_attribute(MEMBERSHIP, result.to_string())
                .set_data(data))
        }
        ExecuteMsg::PacketTimeout {
            client_id,
            next_seq_recv_verification_result,
        } => {
            // let is_channel_valid =
            //     validate_channel_state(&client_id, &client, &verify_channel_state)?;
            let _sequence_valid =
                validate_next_seq_recv(&client, &client_id, &next_seq_recv_verification_result)?;
            let packet_res: PacketDataResponse = next_seq_recv_verification_result.try_into()?;

            Ok(Response::new().set_data(to_binary(&packet_res).map_err(ContractError::Std)?))
        }
        ExecuteMsg::TimeoutOnCLose {
            client_id,
            verify_channel_state,
            next_seq_recv_verification_result,
        } => {
            let is_channel_valid =
                validate_channel_state(&client_id, &client, &verify_channel_state)?;
            let _sequence_valid =
                validate_next_seq_recv(&client, &client_id, &next_seq_recv_verification_result)?;
            let packet_res: PacketDataResponse = next_seq_recv_verification_result.try_into()?;

            Ok(Response::new()
                .add_attribute(MEMBERSHIP, is_channel_valid.to_string())
                .set_data(to_binary(&packet_res).map_err(ContractError::Std)?))
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
    client: &IconClient,
    state: &VerifyChannelState,
) -> Result<bool, ContractError> {
    let proofs_decoded =
        MerkleProofs::decode(state.proof.as_slice()).map_err(ContractError::DecodeError)?;
    let height = to_height_u64(&state.proof_height)?;
    let result = client.verify_membership(
        client_id,
        height,
        0,
        0,
        &proofs_decoded.proofs,
        &state.counterparty_chan_end_path,
        &state.expected_counterparty_channel_end,
    )?;
    Ok(result)
}

pub fn validate_connection_state(
    client_id: &str,
    client: &IconClient,
    state: &VerifyConnectionState,
) -> Result<bool, ContractError> {
    let proofs_decoded =
        MerkleProofs::decode(state.proof.as_slice()).map_err(ContractError::DecodeError)?;
    let height = to_height_u64(&state.proof_height)?;

    let result = client.verify_membership(
        client_id,
        height,
        0,
        0,
        &proofs_decoded.proofs,
        &state.counterparty_conn_end_path,
        &state.expected_counterparty_connection_end,
    )?;
    Ok(result)
}

pub fn validate_client_state(
    client_id: &str,
    client: &IconClient,
    state: &VerifyClientFullState,
) -> Result<bool, ContractError> {
    let proofs_decoded = MerkleProofs::decode(state.client_state_proof.as_slice())
        .map_err(ContractError::DecodeError)?;
    println!("starting validating client state");
    let height = to_height_u64(&state.proof_height)?;
    let result = client.verify_membership(
        client_id,
        height,
        0,
        0,
        &proofs_decoded.proofs,
        &state.client_state_path,
        &state.expected_client_state,
    )?;
    Ok(result)
}

pub fn validate_consensus_state(
    client_id: &str,
    client: &IconClient,
    state: &VerifyClientConsensusState,
) -> Result<bool, ContractError> {
    let proofs_decoded = MerkleProofs::decode(state.consensus_state_proof.as_slice())
        .map_err(ContractError::DecodeError)?;
    let height = to_height_u64(&state.proof_height)?;
    let result = client.verify_membership(
        client_id,
        height,
        0,
        0,
        &proofs_decoded.proofs,
        &state.conesenus_state_path,
        &state.expected_conesenus_state,
    )?;
    Ok(result)
}

pub fn validate_next_seq_recv(
    client: &IconClient,
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
            packet_data: _,
        } => {
            let proofs_decoded =
                MerkleProofs::decode(proof.as_slice()).map_err(ContractError::DecodeError)?;
            let height = to_height_u64(height)?;

            client.verify_membership(
                client_id,
                height,
                0,
                0,
                &proofs_decoded.proofs,
                seq_recv_path,
                sequence.to_be_bytes().as_ref(),
            )?
        }
        LightClientPacketMessage::VerifyPacketReceiptAbsence {
            height,
            prefix: _,
            proof,
            root: _,
            receipt_path,
            packet_data: _,
        } => {
            let proofs_decoded =
                MerkleProofs::decode(proof.as_slice()).map_err(ContractError::DecodeError)?;
            let height = to_height_u64(height)?;

            client.verify_non_membership(
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

pub fn to_packet_response(packet_data: &[u8]) -> Result<Binary, ContractError> {
    let packet_data: PacketData = from_slice(packet_data).map_err(ContractError::Std)?;
    let data = to_binary(&PacketDataResponse::from(packet_data)).map_err(ContractError::Std)?;
    Ok(data)
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

#[cw_serde]
pub struct MigrateMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default().add_attribute("migrate", "successful"))
}

pub fn get_light_client<'a>(
    context: &'a mut CwContext<'_>,
) -> impl ILightClient<Error = ContractError> + 'a {
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

#[cfg(test)]
mod tests {

    use common::icon::icon::types::v1::{BtpHeader, SignedHeader};
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, OwnedDeps, Response,
    };
    use cw2::get_contract_version;
    use cw_common::raw_types::Any;
    use test_utils::{get_test_headers, get_test_signed_headers, to_attribute_map};

    use crate::{
        constants::{CLIENT_STATE_HASH, CONSENSUS_STATE_HASH},
        contract::to_height_u64,
        state::QueryHandler,
        ContractError,
    };
    use common::traits::AnyTypes;
    use cw_common::client_msg::ExecuteMsg;
    use prost::Message;

    use super::{execute, instantiate, Config, InstantiateMsg, CONTRACT_NAME, CONTRACT_VERSION};
    const SENDER: &str = "sender";

    fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            ibc_host: Addr::unchecked("ibc_host"),
        };
        let info = mock_info(SENDER, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        deps
    }

    fn init_client(
        client_id: &str,
        header: &BtpHeader,
        trusting_period: Option<u64>,
    ) -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = setup();
        let client_state = header.to_client_state(trusting_period.unwrap_or(1000000), 0);
        let consensus_state = header.to_consensus_state();
        let info = mock_info(SENDER, &[]);
        let msg = ExecuteMsg::CreateClient {
            client_id: client_id.to_string(),
            client_state: client_state.to_any().encode_to_vec(),
            consensus_state: consensus_state.to_any().encode_to_vec(),
        };

        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        deps
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(SENDER, &[]);

        let msg = InstantiateMsg::default();

        let res: Response = instantiate(deps.as_mut(), env, info.clone(), msg.clone()).unwrap();

        assert_eq!(0, res.messages.len());

        let config = Config::new(info.sender, msg.ibc_host);

        let stored_config = QueryHandler::get_config(deps.as_ref().storage).unwrap();
        assert_eq!(config, stored_config);

        let version = get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(version.version, CONTRACT_VERSION);
        assert_eq!(version.contract, CONTRACT_NAME);
    }

    #[test]
    fn test_execute_create_client() {
        let client_id = "test_client".to_string();
        let mut deps = setup();
        let info = mock_info(SENDER, &[]);
        let start_header = &get_test_headers()[0];
        let client_state = start_header.to_client_state(1000000, 0);
        let consensus_state = start_header.to_consensus_state();
        let client_state_any: Any = client_state.to_any();
        let consensus_state_any: Any = consensus_state.to_any();
        let msg = ExecuteMsg::CreateClient {
            client_id: client_id.clone(),
            client_state: client_state_any.encode_to_vec(),
            consensus_state: consensus_state_any.encode_to_vec(),
        };

        let result = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

        let attributes = to_attribute_map(&result.attributes);
        assert_eq!(
            attributes.get(CLIENT_STATE_HASH).unwrap(),
            &client_state.get_keccak_hash_string()
        );
        assert_eq!(
            attributes.get(CONSENSUS_STATE_HASH).unwrap(),
            &consensus_state.get_keccak_hash_string()
        );

        let stored_client_state =
            QueryHandler::get_client_state(deps.as_ref().storage, &client_id).unwrap();

        assert_eq!(client_state, stored_client_state);
        let result = execute(deps.as_mut(), mock_env(), info, msg);
        assert_eq!(
            result,
            Err(ContractError::ClientStateAlreadyExists(client_id))
        );
    }

    #[test]
    fn test_execute_update_client_with_invalid_trusting_period() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, start_header, Some(100));

        let signed_header = &get_test_signed_headers()[1];
        let info = mock_info(SENDER, &[]);
        let msg = InstantiateMsg {
            ibc_host: Addr::unchecked(SENDER),
        };
        let _result: Response = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::UpdateClient {
            client_id: client_id.clone(),
            signed_header: signed_header.to_any().encode_to_vec(),
        };
        let result = execute(deps.as_mut(), mock_env(), info, msg);
        let stored_client_state =
            QueryHandler::get_client_state(deps.as_ref().storage, &client_id).unwrap();
        assert_eq!(
            result,
            Err(ContractError::TrustingPeriodElapsed {
                trusted_height: stored_client_state.latest_height,
                update_height: signed_header.header.clone().unwrap().main_height
            })
        );
    }

    #[test]
    fn test_execute_update_client() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, start_header, None);

        let signed_header: &SignedHeader = &get_test_signed_headers()[1].clone();
        let header_any: Any = signed_header.to_any();
        let block_height = signed_header.header.clone().unwrap().main_height;
        let info = mock_info(SENDER, &[]);
        let msg = InstantiateMsg {
            ibc_host: Addr::unchecked(SENDER),
        };
        let _result: Response = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::UpdateClient {
            client_id: client_id.clone(),
            signed_header: header_any.encode_to_vec(),
        };
        let _result = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let updated_client_state =
            QueryHandler::get_client_state(deps.as_ref().storage, &client_id).unwrap();

        let consensus_state =
            QueryHandler::get_consensus_state(deps.as_ref().storage, &client_id, block_height)
                .unwrap();

        assert_eq!(updated_client_state.latest_height, block_height);

        assert_eq!(
            consensus_state.message_root,
            signed_header.header.clone().unwrap().message_root
        )
    }
    #[test]
    fn test_to_height_u64() {
        // write test for to_height_u64
        let height = "1-1";
        let height_u64 = to_height_u64(height).unwrap();
        assert_eq!(height_u64, 1);
    }
}
