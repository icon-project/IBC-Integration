use std::cell::RefCell;

use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};
use common::icon::icon::types::v1::{MerkleProofs, SignedHeader};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw_common::client_response::CreateClientResponse;
use ibc_proto::google::protobuf::Any;

use crate::constants::{
    CLIENT_STATE_HASH, CONSENSUS_STATE_HASH, HEIGHT, MEMBERSHIP, NON_MEMBERSHIP,
};
use crate::error::ContractError;
use crate::light_client::IconClient;
use crate::state::{CwContext, QueryHandler};
use crate::traits::{Config, IContext, ILightClient};
use cw_common::client_msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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
            client_state,
            consensus_state,
        } => {
            let client_state = ClientState::decode(client_state.as_slice())
                .map_err(|e| ContractError::DecodeError(e))?;
            let consensus_state = ConsensusState::decode(consensus_state.as_slice())
                .map_err(|e| ContractError::DecodeError(e))?;
            let (state_byte, update) =
                client.create_client(&client_id, client_state, consensus_state)?;

            let mut response = Response::new()
                .add_attribute(CLIENT_STATE_HASH, hex::encode(state_byte.clone()))
                .add_attribute(
                    CONSENSUS_STATE_HASH,
                    hex::encode(update.consensus_state_commitment),
                )
                .add_attribute(HEIGHT, update.height.to_string());

            let client_response = CreateClientResponse::new(
                "icon-light-client".to_string(),
                update.height.to_string(),
                state_byte,
                update.consensus_state_commitment.into(),
            );

            response.data = to_binary(&client_response).ok();

            Ok(response)
        }
        ExecuteMsg::UpdateClient {
            client_id,
            signed_header,
        } => {
            let header_any = SignedHeader::decode(signed_header.as_slice()).unwrap();
            let (state_byte, update) = client.update_client(&client_id, header_any)?;
            Ok(Response::new()
                .add_attribute(CLIENT_STATE_HASH, hex::encode(state_byte))
                .add_attribute(
                    CONSENSUS_STATE_HASH,
                    hex::encode(update.consensus_state_commitment),
                )
                .add_attribute(HEIGHT, update.height.to_string()))
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
            let proofs_decoded = MerkleProofs::decode(proofs.as_slice())
                .map_err(|e| ContractError::DecodeError(e))?;
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
            let proofs_decoded = MerkleProofs::decode(proofs.as_slice())
                .map_err(|e| ContractError::DecodeError(e))?;
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
        _ => {
            todo!()
        }
    }
}

pub fn any_from_byte(bytes: &[u8]) -> Result<Any, ContractError> {
    let any = Any::decode(bytes).map_err(|e| ContractError::DecodeError(e))?;
    Ok(any)
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
mod tests {

    use std::result;

    use common::icon::icon::types::v1::BtpHeader;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        OwnedDeps, Response,
    };
    use cw2::get_contract_version;
    use test_utils::{get_test_headers, to_attribute_map, get_test_signed_headers};

    use crate::traits::AnyTypes;
    use crate::{
        constants::{CLIENT_STATE_HASH, CONSENSUS_STATE_HASH},
        state::QueryHandler,
        ContractError,
    };
    use cw_common::client_msg::ExecuteMsg;
    use prost::Message;

    use super::{execute, instantiate, Config, InstantiateMsg, CONTRACT_NAME, CONTRACT_VERSION};
    const SENDER: &str = "sender";

    fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg::default();
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
        let client_state = header.to_client_state(trusting_period.unwrap_or(1000000) , 0);
        let consensus_state = header.to_consensus_state();
        let info = mock_info(SENDER, &[]);
        let msg = ExecuteMsg::CreateClient {
            client_id: client_id.to_string(),
            client_state: client_state.encode_to_vec(),
            consensus_state: consensus_state.encode_to_vec(),
        };

        execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        deps
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(SENDER, &[]);
        let msg = InstantiateMsg::default();

        let res: Response =
            instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

        assert_eq!(0, res.messages.len());

        let config = Config::new("0x3.icon".to_string(), 1, 1, info.sender);

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
        let msg = ExecuteMsg::CreateClient {
            client_id: client_id.clone(),
            client_state: client_state.encode_to_vec(),
            consensus_state: consensus_state.encode_to_vec(),
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
            Err(ContractError::ClientStateAlreadyExists(client_id.clone()))
        );
    }

    #[test]
    fn test_execute_update_client_with_invalid_trusting_period() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, &start_header, Some(100));

        let signed_header = &get_test_signed_headers()[1];
        let info = mock_info(SENDER, &[]);
        let msg = ExecuteMsg::UpdateClient { 
            client_id: client_id.clone(), 
            signed_header: signed_header.encode_to_vec()
        };
        let result = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        let stored_client_state = QueryHandler::get_client_state(deps.as_ref().storage, &client_id).unwrap();
        assert_eq!(
            result,
            Err(ContractError::TrustingPeriodElapsed{saved_height: stored_client_state.latest_height, update_height: signed_header.header.clone().unwrap().main_height})
        );
    }

    #[test]
    fn test_execute_update_client_with_non_consecutive_header() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, &start_header, None);

        let random_signed_header = &get_test_signed_headers()[2];
        let info = mock_info(SENDER, &[]);
        let msg = ExecuteMsg::UpdateClient { 
            client_id: client_id.clone(), 
            signed_header: random_signed_header.encode_to_vec()
        };
        let result = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert_eq!(
            result,
            Err(ContractError::InvalidHeaderUpdate("network section mismatch".to_string()))
        );
    }

    #[test]
    fn test_execute_update_client() {
        let start_header = &get_test_headers()[0];
        let client_id = "test_client".to_string();
        let mut deps = init_client(&client_id, &start_header, None);

        let signed_header = &get_test_signed_headers()[1];
        let block_height =  signed_header.header.clone().unwrap().main_height;
        let info = mock_info(SENDER, &[]);
        let msg = ExecuteMsg::UpdateClient { 
            client_id: client_id.clone(), 
            signed_header: signed_header.encode_to_vec()
        };
        let result = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

        let updated_client_state =
            QueryHandler::get_client_state(deps.as_ref().storage, &client_id).unwrap();
        
        let consensus_state = 
            QueryHandler::get_consensus_state(deps.as_ref().storage, &client_id, block_height).unwrap();
        
        assert_eq!(
            updated_client_state.latest_height,
            block_height
        );

        assert_eq!(
            updated_client_state.network_section_hash,
            signed_header.header.clone().unwrap().get_network_section_hash().to_vec()
        );

        assert_eq!(
            consensus_state.message_root,
            signed_header.header.clone().unwrap().message_root
        )
        
    }
}
