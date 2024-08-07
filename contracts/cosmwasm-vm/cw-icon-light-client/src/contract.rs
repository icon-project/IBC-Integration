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
    to_json_binary as to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
use cw2::set_contract_version;
use cw_common::client_response::{CreateClientResponse, UpdateClientResponse};
use cw_common::raw_types::Any;
use cw_common::types::VerifyChannelState;

use crate::constants::{CLIENT_STATE_HASH, CONSENSUS_STATE_HASH, HEIGHT};
use crate::error::ContractError;
use crate::light_client::IconClient;
use crate::state::CwContext;
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

#[cfg(test)]
mod tests {

    use common::icon::icon::types::v1::{BtpHeader, SignedHeader};
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_json_binary as to_binary, Addr, OwnedDeps, Response,
    };
    use cw2::get_contract_version;
    use cw_common::{client_msg::QueryMsg, raw_types::Any};
    use test_utils::{get_test_headers, get_test_signed_headers, to_attribute_map};

    use crate::{
        constants::{CLIENT_STATE_HASH, CONSENSUS_STATE_HASH},
        contract::{ensure_owner, query, to_height_u64},
        query_handler::QueryHandler,
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
        instantiate(deps.as_mut(), mock_env(), info, InstantiateMsg::default()).unwrap();
        let msg = ExecuteMsg::CreateClient {
            client_id: client_id.to_string(),
            client_state: client_state.to_any().encode_to_vec(),
            consensus_state: consensus_state.to_any().encode_to_vec(),
        };
        let info = mock_info("ibc_host", &[]);

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
        let env = mock_env();
        let msg = InstantiateMsg::default();

        let _res: Response = instantiate(deps.as_mut(), env, info, msg).unwrap();

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

        let info = mock_info("ibc_host", &[]);

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
        let msg = InstantiateMsg::default();
        let _result: Response = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("ibc_host", &[]);

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

    #[test]
    fn test_update_block_older_than_trusted_height() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, start_header, Some(100));

        let mut signed_header: SignedHeader = get_test_signed_headers()[1].clone();
        signed_header.trusted_height = 10;
        let mut btp_header = signed_header.header.clone().unwrap();
        btp_header.main_height = 9;
        signed_header.header = Some(btp_header);

        let info = mock_info(SENDER, &[]);
        let msg = InstantiateMsg::default();
        let _result: Response = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("ibc_host", &[]);

        let msg = ExecuteMsg::UpdateClient {
            client_id,
            signed_header: signed_header.to_any().encode_to_vec(),
        };
        let result = execute(deps.as_mut(), mock_env(), info, msg);

        assert_eq!(
            result,
            Err(ContractError::UpdateBlockOlderThanTrustedHeight)
        );
    }

    #[test]
    fn test_invalid_proof_context_hash() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, start_header, Some(1000000));

        let mut signed_header: SignedHeader = get_test_signed_headers()[1].clone();
        signed_header.current_validators = vec![vec![22, 33, 44]];
        let info = mock_info(SENDER, &[]);
        let msg = InstantiateMsg::default();
        let _result: Response = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("ibc_host", &[]);

        let msg = ExecuteMsg::UpdateClient {
            client_id,
            signed_header: signed_header.to_any().encode_to_vec(),
        };
        let result = execute(deps.as_mut(), mock_env(), info, msg);

        assert_eq!(result, Err(ContractError::InvalidProofContextHash));
    }
    #[test]
    fn test_update_block_too_old() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, start_header, Some(10));

        let mut signed_header: SignedHeader = get_test_signed_headers()[1].clone();
        signed_header.trusted_height = 8;
        let mut btp_header = signed_header.header.clone().unwrap();
        btp_header.main_height = 9;
        signed_header.header = Some(btp_header);
        let info = mock_info(SENDER, &[]);
        let msg = InstantiateMsg::default();
        let _result: Response = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("ibc_host", &[]);

        let msg = ExecuteMsg::UpdateClient {
            client_id,
            signed_header: signed_header.to_any().encode_to_vec(),
        };
        let result = execute(deps.as_mut(), mock_env(), info, msg);

        assert_eq!(result, Err(ContractError::UpdateBlockTooOld));
    }

    #[test]
    fn test_invalid_header_update() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, start_header, Some(1000000));

        let mut signed_header: SignedHeader = get_test_signed_headers()[1].clone();
        let mut btp_header = signed_header.header.clone().unwrap();
        btp_header.network_id = 1122;
        signed_header.header = Some(btp_header);
        let info = mock_info(SENDER, &[]);
        let msg = InstantiateMsg::default();
        let _result: Response = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("ibc_host", &[]);

        let msg = ExecuteMsg::UpdateClient {
            client_id,
            signed_header: signed_header.to_any().encode_to_vec(),
        };
        let result = execute(deps.as_mut(), mock_env(), info, msg);

        assert_eq!(
            result,
            Err(ContractError::InvalidHeaderUpdate(
                "network id mismatch".to_string()
            ))
        );
    }

    #[test]
    fn test_execute_same_update_client() {
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
        let _result = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

        let updated_client_state =
            QueryHandler::get_client_state(deps.as_ref().storage, &client_id).unwrap();

        let consensus_state =
            QueryHandler::get_consensus_state(deps.as_ref().storage, &client_id, block_height)
                .unwrap();

        assert_eq!(updated_client_state.latest_height, block_height);

        assert_eq!(
            consensus_state.message_root,
            signed_header.header.clone().unwrap().message_root
        );

        let second = execute(deps.as_mut(), mock_env(), info, msg);
        assert_eq!(
            second,
            Err(ContractError::HeightAlreadyUpdated { height: 82873 })
        );
    }

    #[test]
    fn test_query_client_state() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let deps = init_client(&client_id, start_header, None);

        let msg = QueryMsg::GetClientState {
            client_id: client_id.clone(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let state =
            QueryHandler::get_client_state_any(deps.as_ref().storage, client_id.as_str()).unwrap();
        assert_eq!(res, to_binary(&state).unwrap());
    }

    #[test]
    fn test_query_consensus_state() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let deps = init_client(&client_id, start_header, None);

        let msg = QueryMsg::GetConsensusState {
            client_id: client_id.clone(),
            height: start_header.main_height,
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let state = QueryHandler::get_consensus_state_any(
            deps.as_ref().storage,
            client_id.as_str(),
            start_header.main_height,
        )
        .unwrap();
        assert_eq!(res, to_binary(&state).unwrap());
    }

    #[test]
    fn test_query_latest_height() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let deps = init_client(&client_id, start_header, None);

        let msg = QueryMsg::GetLatestHeight {
            client_id: client_id.clone(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let height =
            QueryHandler::get_latest_height(deps.as_ref().storage, client_id.as_str()).unwrap();
        assert_eq!(res, to_binary(&height).unwrap());
    }

    #[test]
    fn test_query_latest_consensus_state() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let deps = init_client(&client_id, start_header, None);

        let msg = QueryMsg::GetLatestConsensusState {
            client_id: client_id.clone(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let state =
            QueryHandler::get_latest_consensus_state(deps.as_ref().storage, client_id.as_str())
                .unwrap();
        assert_eq!(res, to_binary(&state).unwrap());
    }

    #[test]
    #[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>")]
    fn test_query_latest_consensus_state_fail() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let deps = init_client(&client_id, start_header, None);

        let msg = QueryMsg::GetLatestConsensusState {
            client_id: "another_client".to_string(),
        };
        query(deps.as_ref(), mock_env(), msg).unwrap();
    }

    #[test]
    fn test_query_previous_consensus_state() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let deps = init_client(&client_id, start_header, None);

        let msg = QueryMsg::GetPreviousConsensusState {
            client_id: client_id.clone(),
            height: start_header.main_height,
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let state = QueryHandler::get_previous_consensus(
            deps.as_ref().storage,
            start_header.main_height,
            client_id,
        )
        .unwrap();
        assert_eq!(res, to_binary(&state).unwrap());
    }

    #[test]
    fn test_ensure_owner() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let deps = init_client(&client_id, start_header, None);
        let info = mock_info(SENDER, &[]);

        let res = ensure_owner(deps.as_ref(), &info);
        assert!(res.is_ok())
    }

    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn test_ensure_owner_unauthorized() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let deps = init_client(&client_id, start_header, None);
        let info = mock_info("not_owner", &[]);

        ensure_owner(deps.as_ref(), &info).unwrap()
    }
}
