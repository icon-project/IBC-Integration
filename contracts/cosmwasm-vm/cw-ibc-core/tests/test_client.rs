pub mod setup;

use std::str::FromStr;

use cosmwasm_std::Timestamp;
use std::time::Duration;

use common::client_state::{get_default_icon_client_state, IClientState};

use common::ibc::{signer::Signer, Height};
use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};
use common::traits::AnyTypes;
use common::utils::keccak256;
use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_std::{to_binary, to_vec, Addr, Event, Reply, SubMsgResponse};
use cw_common::client_response::{
    MisbehaviourResponse, UpdateClientResponse, UpgradeClientResponse,
};

use cw_common::commitment;
use cw_common::raw_types::client::{
    RawMsgCreateClient, RawMsgSubmitMisbehaviour, RawMsgUpdateClient, RawMsgUpgradeClient,
};
use cw_common::raw_types::Any;

use common::ibc::core::ics02_client::client_type::ClientType;
use common::ibc::core::ics24_host::identifier::ClientId;
use common::ibc::mock::header::MockHeader;
use cw_ibc_core::conversions::{to_ibc_client_id, to_ibc_connection_id};
use cw_ibc_core::light_client::light_client::LightClient;
use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics02_client::events::{
        client_misbehaviour_event, create_client_event, generated_client_id_event,
        upgrade_client_event,
    },
    traits::IbcClient,
};
use cw_ibc_core::{EXECUTE_UPDATE_CLIENT, EXECUTE_UPGRADE_CLIENT};

use prost::Message;
use setup::*;

#[test]
fn get_client_next_seq_on_a() {
    let mut mock = deps();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(mock.as_mut().storage, 0)
        .unwrap();

    let result = contract.client_counter(mock.as_ref().storage).unwrap();

    assert_eq!(result, 0)
}

#[test]
fn increment_next_client_seq_on_a() {
    let mut mock = deps();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(mock.as_mut().storage, 0)
        .unwrap();

    let increment = contract
        .increase_client_counter(mock.as_mut().storage)
        .unwrap();

    let result = contract.client_counter(mock.as_ref().storage).unwrap();

    assert_eq!(increment, result)
}

#[test]
fn store_client_implement_success() {
    let mut mock = deps();
    let contract = CwIbcCoreContext::default();

    let client_type = ClientType::new("new_client_type".to_string());

    let client_id = ClientId::new(client_type, 1).unwrap();

    let light_client_address = LightClient::new("light-client".to_string());

    contract
        .store_client_implementations(
            mock.as_mut().storage,
            &client_id,
            light_client_address.clone(),
        )
        .unwrap();

    let result = contract
        .get_client_implementations(mock.as_ref().storage, &client_id)
        .unwrap();

    assert_eq!(light_client_address, result)
}

#[test]
#[should_panic(expected = "InvalidClientId { client_id: \"new_client_type-1\" }")]
fn store_client_implement_failure() {
    let mock = deps();
    let contract = CwIbcCoreContext::default();

    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 1).unwrap();

    contract
        .get_client_implementations(mock.as_ref().storage, &client_id)
        .unwrap();
}

#[test]
fn store_client_into_registry() {
    let mut mock = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_client_type".to_string());
    let light_client_address = "light-client".to_string();
    contract
        .store_client_into_registry(
            mock.as_mut().storage,
            client_type.clone(),
            light_client_address.clone(),
        )
        .unwrap();

    let result = contract
        .get_client_from_registry(mock.as_ref().storage, client_type)
        .unwrap();

    assert_eq!(light_client_address, result);
}
#[test]
#[should_panic(expected = "InvalidClientType { client_type: \"new_client_type\" }")]
fn fails_on_querying_client_from_registry() {
    let mock = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_client_type".to_string());
    contract
        .get_client_from_registry(mock.as_ref().storage, client_type)
        .unwrap();
}

#[test]
fn test_create_client_event() {
    let height = Height::new(15, 10).unwrap();

    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 1).unwrap();
    let result = create_client_event(
        client_id.as_str(),
        client_type.as_str(),
        &height.to_string(),
    );

    assert_eq!("create_client", result.ty)
}

#[test]
fn test_upgrade_client_event() {
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();
    let signer = get_dummy_account_id();

    let height = Height::new(1, 1).unwrap();
    let mock_height = to_mock_height(height);

    let client_state: Any = MockClientState::new(MockHeader::new(mock_height)).into();
    let consensus_state: Any = MockConsensusState::new(MockHeader::new(mock_height)).into();

    let proof = get_dummy_merkle_proof();

    let _msg = RawMsgUpgradeClient {
        client_id: client_id.to_string(),
        client_state: Some(client_state),
        consensus_state: Some(consensus_state),
        proof_upgrade_client: proof.encode_to_vec(),
        proof_upgrade_consensus_state: proof.encode_to_vec(),
        signer: signer.to_string(),
    };

    let event = upgrade_client_event(client_type, height, client_id);

    assert_eq!("upgrade_client", event.ty);

    assert_eq!(event.attributes[0].value, "new_client_type-10")
}

#[test]
fn create_misbehaviour_event_test() {
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();

    let event = client_misbehaviour_event(client_id.as_str(), client_type.as_str());

    assert_eq!("client_misbehaviour", event.ty)
}

#[test]
fn store_client_type_success() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("icon_client".to_string());

    let client_id = ClientId::new(client_type.clone(), 10).unwrap();

    contract
        .store_client_type(deps.as_mut().storage, &client_id, client_type.clone())
        .unwrap();
    let result = contract
        .get_client_type(deps.as_ref().storage, &client_id)
        .unwrap();

    assert_eq!(client_type, result)
}

#[test]
#[should_panic(expected = "InvalidClientId { client_id: \"icon_client-10\" }")]
fn fail_to_query_client_type() {
    let deps = deps();

    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("icon_client".to_string());

    let client_id = ClientId::new(client_type, 10).unwrap();

    contract
        .get_client_type(deps.as_ref().storage, &client_id)
        .unwrap();
}

#[test]
fn check_for_genereted_client_id_event() {
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 10).unwrap();
    let event = generated_client_id_event(client_id.clone());

    assert_eq!("client_id_created", event.ty);

    assert_eq!(event.attributes[0].value, client_id.as_str())
}

#[test]
fn check_for_create_client_message() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    ctx.init_client_state(deps.as_mut(), &contract);

    let create_client_message = RawMsgCreateClient {
        client_state: Some(ctx.client_state.unwrap().to_any()),
        consensus_state: Some(ctx.consensus_state.unwrap().to_any()),
        signer: "new_signer".to_string(),
    };

    let response = contract
        .create_client(deps.as_mut(), info, mock_env(), create_client_message)
        .unwrap();

    assert_eq!(response.messages[0].id, 21);
    assert_eq!(response.attributes[1].value, "create_client");
}

#[test]
fn check_for_client_state_from_storage() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    ctx.init_client_state(deps.as_mut(), &contract);

    let client_state: ClientState = ctx.client_state.clone().unwrap();
    let consenus_state: ConsensusState = ctx.consensus_state.clone().unwrap();

    let create_client_message = RawMsgCreateClient {
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consenus_state.to_any()),
        signer: "new_signer".to_string(),
    };

    contract
        .create_client(deps.as_mut(), info, mock_env(), create_client_message)
        .unwrap();

    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let client_state = contract
        .client_state(deps.as_ref(), &ctx.client_id)
        .unwrap();
    assert_eq!(client_state.client_type().as_str(), "iconclient");
}

#[test]
fn check_for_consensus_state_from_storage() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    ctx.init_client_state(deps.as_mut(), &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let height = &ctx.client_state.unwrap().latest_height();
    let consensus_state_result = contract.consensus_state(deps.as_ref(), &ctx.client_id, &height);

    assert!(consensus_state_result.is_ok());
    assert_eq!(
        "message_root".as_bytes().to_vec(),
        consensus_state_result.unwrap().root().as_bytes()
    )
}

#[test]
#[should_panic(expected = "InvalidNextClientSequence")]
fn fails_on_create_client_message_without_proper_initialisation() {
    let ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    let msg = RawMsgCreateClient {
        client_state: Some(ctx.client_state.unwrap().to_any()),
        consensus_state: Some(ctx.consensus_state.unwrap().to_any()),
        signer: "new_signer".to_string(),
    };
    contract
        .create_client(deps.as_mut(), info, mock_env(), msg)
        .unwrap();
}

#[test]
fn check_for_update_client_message() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    let client_state: ClientState = ctx.client_state.clone().unwrap();
    let consenus_state: ConsensusState = ctx.consensus_state.clone().unwrap();
    ctx.init_client_state(deps.as_mut(), &contract);

    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let create_client_message = RawMsgCreateClient {
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consenus_state.to_any()),
        signer: "signer".to_string(),
    };
    let response = contract
        .create_client(
            deps.as_mut(),
            info.clone(),
            mock_env(),
            create_client_message,
        )
        .unwrap();
    assert_eq!(response.messages[0].id, 21);

    assert_eq!(response.attributes[1].value, "create_client");

    let msg = RawMsgCreateClient {
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consenus_state.to_any()),
        signer: "signer".to_string(),
    };
    contract
        .create_client(deps.as_mut(), info.clone(), mock_env(), msg)
        .unwrap();

    let update_client_message = RawMsgUpdateClient {
        client_id: ctx.client_id.to_string(),
        header: Some(client_state.to_any()),
        signer: "signer".to_string(),
    };
    let result = contract
        .update_client(deps.as_mut(), info, update_client_message)
        .unwrap();

    assert_eq!(ctx.client_id.as_str(), result.attributes[1].value);

    let mock_reponse_data = UpdateClientResponse::new(
        "10-15".to_string(),
        ctx.client_id.as_str().to_string(),
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.encode_to_vec(),
        consenus_state.encode_to_vec(),
    );
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");
    let reply_message = Reply {
        id: EXECUTE_UPDATE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    let update_response =
        contract.execute_update_client_reply(deps.as_mut(), get_mock_env(), reply_message);

    assert!(update_response.is_ok());

    let result = update_response.unwrap();

    assert_eq!("execute_update_client_reply", result.attributes[0].value);
    assert_eq!("10-15", result.attributes[1].value);
    assert_eq!("update_client", result.events[0].ty);
    assert_eq!("iconclient-0", result.events[0].attributes[0].value)
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: ClientSpecific { description: \"LightclientNotFount\" } }"
)]
fn fails_on_updating_non_existing_client() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    let client_state: ClientState = get_dummy_client_state();

    let client_id = ClientId::from_str("iconclient-0").unwrap();
    let signer = Signer::from_str("new_signer").unwrap();
    let update_client_message = RawMsgUpdateClient {
        client_id: client_id.to_string(),
        header: Some(client_state.to_any()),
        signer: signer.to_string(),
    };

    contract
        .update_client(deps.as_mut(), info, update_client_message)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: ClientFrozen { client_id: ClientId(\"iconclient-0\") } }"
)]
fn fails_update_client_for_frozen_client() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::new();
    let info = create_mock_info("alice", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 10;
    }

    ctx.init_client_state(deps.as_mut(), &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let msg = RawMsgUpdateClient {
        client_id: ctx.client_id.to_string(),
        header: Some(get_dummy_client_state().to_any()),
        signer: "new_signer".to_string(),
    };
    contract.update_client(deps.as_mut(), info, msg).unwrap();
}

#[test]
#[should_panic(expected = "IbcClientError { error: Other { description: \"response_error\" } }")]
fn fails_on_error_ressponse() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    let reply_message = Reply {
        id: EXECUTE_UPDATE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Err("response_error".to_string()),
    };
    contract
        .execute_update_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();
}

#[test]
fn check_for_upgrade_client() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.trusting_period = 2000000000;
    }

    let client_state = ctx.client_state.clone().unwrap();
    let consenus_state: ConsensusState = ctx.consensus_state.clone().unwrap();
    ctx.init_client_state(deps.as_mut(), &contract);

    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let msg = RawMsgCreateClient {
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consenus_state.to_any()),
        signer: "signer".to_string(),
    };
    contract
        .create_client(deps.as_mut(), info.clone(), mock_env(), msg)
        .unwrap();

    let upgrdade_client_message = RawMsgUpgradeClient {
        client_id: ctx.client_id.to_string(),
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consenus_state.to_any()),
        proof_upgrade_client: get_dummy_merkle_proof().encode_to_vec(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof().encode_to_vec(),
        signer: "new_signer".to_string(),
    };

    let result = contract
        .upgrade_client(deps.as_mut(), info, ctx.env, upgrdade_client_message)
        .unwrap();

    assert_eq!("upgrade_client", result.attributes[0].value)
}

#[test]
#[should_panic(expected = "IbcClientError { error: HeaderNotWithinTrustPeriod")]
fn fails_on_upgrade_client_invalid_trusting_period() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    ctx.init_client_state(deps.as_mut(), &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let upgrade_client_state: Any = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 200000000,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,

        ..get_default_icon_client_state()
    }
    .to_any();

    let upgrdade_client_message = RawMsgUpgradeClient {
        client_id: ctx.client_id.to_string(),
        client_state: upgrade_client_state.into(),
        consensus_state: Some(ctx.consensus_state.unwrap().to_any()),
        proof_upgrade_client: get_dummy_merkle_proof().encode_to_vec(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof().encode_to_vec(),
        signer: "new_signer".to_string(),
    };
    contract
        .upgrade_client(deps.as_mut(), info, ctx.env, upgrdade_client_message)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidConsensusStateTimestamp")]
fn fails_on_upgrade_client_invalid_consensus_state_timestamp() {
    let mut ctx = TestContext::default(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    let block_time = Duration::from_secs(1);
    ctx.env.block.time = Timestamp::from_nanos(block_time.as_nanos() as u64);

    ctx.init_client_state(deps.as_mut(), &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let upgrdade_client_message = RawMsgUpgradeClient {
        client_id: ctx.client_id.to_string(),
        client_state: Some(ctx.client_state.unwrap().to_any()),
        consensus_state: Some(ctx.consensus_state.unwrap().to_any()),
        proof_upgrade_client: get_dummy_merkle_proof().encode_to_vec(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof().encode_to_vec(),
        signer: "signer".to_string(),
    };
    contract
        .upgrade_client(deps.as_mut(), info, ctx.env, upgrdade_client_message)
        .unwrap();
}

#[test]
#[should_panic(
    expected = " IbcClientError { error: ClientFrozen { client_id: ClientId(\"iconclient-0\") } }"
)]
fn fails_on_upgrade_client_frozen_client() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);
    let contract = CwIbcCoreContext::default();

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 3;
    }

    ctx.init_client_state(deps.as_mut(), &contract);
    let client_state = ctx.client_state.clone().unwrap();
    let consensus_state: ConsensusState = ctx.consensus_state.clone().unwrap();

    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let msg = RawMsgCreateClient {
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consensus_state.to_any()),
        signer: "signer".to_string(),
    };
    contract
        .create_client(deps.as_mut(), info.clone(), mock_env(), msg)
        .unwrap();

    let upgrdade_client_message = RawMsgUpgradeClient {
        client_id: ctx.client_id.to_string(),
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consensus_state.to_any()),
        proof_upgrade_client: get_dummy_merkle_proof().encode_to_vec(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof().encode_to_vec(),
        signer: "new_signer".to_string(),
    };

    contract
        .upgrade_client(deps.as_mut(), info, ctx.env, upgrdade_client_message)
        .unwrap();
}

#[test]
fn check_for_execute_upgrade_client() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);
    let contract = CwIbcCoreContext::default();

    if let Some(client_state) = &mut ctx.client_state {
        client_state.trusting_period = 2000000000;
    }

    let client_state = ctx.client_state.clone().unwrap();
    let consensus_state: ConsensusState = ctx.consensus_state.clone().unwrap();
    ctx.init_client_state(deps.as_mut(), &contract);

    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let msg = RawMsgCreateClient {
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consensus_state.to_any()),
        signer: "signer".to_string(),
    };
    contract
        .create_client(deps.as_mut(), info.clone(), mock_env(), msg)
        .unwrap();

    let upgrdade_client_message = RawMsgUpgradeClient {
        client_id: ctx.client_id.to_string(),
        client_state: Some(client_state.clone().to_any()),
        consensus_state: Some(consensus_state.clone().to_any()),
        proof_upgrade_client: get_dummy_merkle_proof().encode_to_vec(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof().encode_to_vec(),
        signer: "new_signer".to_string(),
    };

    contract
        .upgrade_client(deps.as_mut(), info, ctx.env, upgrdade_client_message)
        .unwrap();

    let upgrade_client_response = UpgradeClientResponse::new(
        keccak256(&client_state.encode_to_vec()).to_vec(),
        client_state.encode_to_vec(),
        keccak256(&consensus_state.encode_to_vec()).to_vec(),
        consensus_state.encode_to_vec(),
        ctx.client_id.to_string(),
        "0-100".to_string(),
    );

    let event = Event::new("empty");
    let mock_data_binary = to_binary(&upgrade_client_response).unwrap();
    let reply_message = Reply {
        id: EXECUTE_UPGRADE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    let result = contract
        .execute_upgrade_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    assert_eq!("iconclient-0", result.attributes[1].value);
    assert_eq!("upgrade_client", result.events[0].ty)
}

#[test]
#[should_panic(
    expected = "IbcValidationError { error: InvalidLength { id: \"hello\", length: 5, min: 9, max: 64 } }"
)]
fn fails_on_invalid_client_identifier_on_execute_upgrade_client() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    ctx.init_client_state(deps.as_mut(), &contract);

    let upgrade_client_response = UpgradeClientResponse::new(
        ctx.client_state.clone().unwrap().get_keccak_hash().to_vec(),
        ctx.client_state.unwrap().encode_to_vec(),
        ctx.consensus_state
            .clone()
            .unwrap()
            .get_keccak_hash()
            .to_vec(),
        ctx.consensus_state.unwrap().encode_to_vec(),
        "hello".to_string(),
        "0-100".to_string(),
    );

    let event = Event::new("empty");
    let mock_data_binary = to_binary(&upgrade_client_response).unwrap();
    let reply_message = Reply {
        id: EXECUTE_UPGRADE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    contract
        .execute_upgrade_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcClientError { error: Other { description: \"UnknownResponse\" } }")]
fn fails_on_unknown_response_on_execute_upgrade_client() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    ctx.init_client_state(deps.as_mut(), &contract);

    let reply_message = Reply {
        id: EXECUTE_UPGRADE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Err("UnknownResponse".to_string()),
    };
    contract
        .execute_upgrade_client_reply(deps.as_mut(), ctx.env, reply_message)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: Other { description: \"Invalid Response Data\" } }"
)]
fn fails_on_null_response_data_on_execute_upgrade_client() {
    let ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    let event = Event::new("empty");
    let reply_message = Reply {
        id: EXECUTE_UPGRADE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: None,
        }),
    };

    contract
        .execute_upgrade_client_reply(deps.as_mut(), ctx.env, reply_message)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: Other { description: \"Client Implementation Already Exist\" } }"
)]
fn fails_on_storing_already_registered_client_into_registry() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    ctx.init_client_state(deps.as_mut(), &contract);

    let lightclient = ctx.lightclient.unwrap().get_address();
    let result = contract
        .get_client_from_registry(deps.as_ref().storage, ctx.client_type.clone())
        .unwrap();
    assert_eq!(lightclient, result);

    contract
        .register_client(deps.as_mut(), ctx.client_type, Addr::unchecked(lightclient))
        .unwrap();
}

#[test]
fn success_on_getting_client() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    ctx.init_client_state(deps.as_mut(), &contract);

    let result = contract
        .get_light_client(deps.as_ref().storage, &ctx.client_id)
        .unwrap();

    assert_eq!(result, ctx.lightclient.unwrap())
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: ClientSpecific { description: \"LightclientNotFount\" } }"
)]
fn fails_on_getting_client_empty_client() {
    let mock_deps = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 0).unwrap();

    contract
        .get_light_client(mock_deps.as_ref().storage, &client_id)
        .unwrap();
}

#[test]
fn test_get_commitment() {
    let ctx = TestContext::default(mock_env());
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::new();

    let client_state_hash = to_vec(&ctx.client_state.clone().unwrap()).unwrap();
    let client_key = commitment::client_state_commitment_key(&ctx.client_id);

    ctx.save_client_commitment(deps.as_mut().storage, &contract, &client_state_hash);

    let res = contract
        .get_commitment(deps.as_ref().storage, client_key)
        .unwrap();
    assert_eq!(res, client_state_hash);
}

#[test]
#[should_panic(expected = "InvalidCommitmentKey")]
fn fail_test_get_commitment() {
    let ctx = TestContext::default(mock_env());
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::new();
    let client_state_hash = to_vec(&get_dummy_client_state()).unwrap();

    ctx.save_client_commitment(deps.as_mut().storage, &contract, &client_state_hash);

    let rand_client = to_ibc_client_id("randclient").unwrap();
    let client_key = commitment::client_state_commitment_key(&rand_client);

    let res = contract
        .get_commitment(deps.as_ref().storage, client_key)
        .unwrap();
    assert_eq!(res, client_state_hash);
}

#[test]
fn test_get_connection() {
    let ctx = TestContext::default(mock_env());
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::new();

    ctx.save_connection(deps.as_mut().storage, &contract);

    let res = contract.get_connection(deps.as_ref().storage, &ctx.connection_id);
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "InvalidConnectiontId { connection_id: \"ibcconnection\" }")]
fn fail_test_get_connection() {
    let ctx = TestContext::default(mock_env());
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::new();

    ctx.save_connection(deps.as_mut().storage, &contract);

    let connection_id = to_ibc_connection_id("ibcconnection").unwrap();
    contract
        .get_connection(deps.as_ref().storage, &connection_id)
        .unwrap();
}

#[test]
fn test_client_state_any() {
    let mut ctx = TestContext::for_client_state(mock_env());
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::new();

    ctx.init_client_state(deps.as_mut(), &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let res = contract
        .client_state_any(deps.as_ref(), &ctx.client_id)
        .unwrap();
    assert_eq!(res, ctx.client_state.unwrap().to_any());
}

#[test]
fn success_on_getting_client_state() {
    let mut ctx = TestContext::for_client_state(mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    ctx.init_client_state(deps.as_mut(), &contract);

    let create_client_message = RawMsgCreateClient {
        client_state: Some(ctx.client_state.unwrap().to_any()),
        consensus_state: Some(ctx.consensus_state.unwrap().to_any()),
        signer: "new_signer".to_string(),
    };
    contract
        .create_client(deps.as_mut(), info, mock_env(), create_client_message)
        .unwrap();

    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let state = contract
        .client_state(deps.as_ref(), &ctx.client_id)
        .unwrap();
    assert_eq!(None, state.frozen_height())
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: ClientSpecific { description: \"LightclientNotFount\" } }"
)]
fn fails_on_getting_client_state() {
    let deps = deps();
    let contract = CwIbcCoreContext::default();

    let client_id = ClientId::from_str("iconclient-0").unwrap();

    let _state = contract.client_state(deps.as_ref(), &client_id).unwrap();
}

#[test]
fn success_on_misbehaviour_validate() {
    let mut ctx = TestContext::for_client_state(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    ctx.init_client_state(deps.as_mut(), &contract);

    let height = mock_height(10, 15).unwrap();
    let mock_header = MockHeader::new(height);

    let misbehaviour = common::ibc::mock::misbehaviour::Misbehaviour {
        client_id: to_mock_client_id(&ctx.client_id),
        header1: mock_header,
        header2: mock_header,
    };
    let misbehaviour: Any = misbehaviour.into();

    let misbehaviour_message = RawMsgSubmitMisbehaviour {
        client_id: ctx.client_id.to_string(),
        misbehaviour: Some(misbehaviour),
        signer: get_dummy_account_id().to_string(),
    };

    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let result = contract.misbehaviour(deps.as_mut(), info, misbehaviour_message);
    assert!(result.is_ok())
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: ClientFrozen { client_id: ClientId(\"default-0\") } }"
)]
fn fails_on_frozen_client_on_misbehaviour_validate() {
    let mut ctx = TestContext::default(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 10;
    }

    ctx.init_client_state(deps.as_mut(), &contract);

    let height = mock_height(10, 15).unwrap();
    let mock_header = MockHeader::new(height);

    let misbehaviour: Any = common::ibc::mock::misbehaviour::Misbehaviour {
        client_id: to_mock_client_id(&ctx.client_id),
        header1: mock_header,
        header2: mock_header,
    }
    .into();

    let misbehaviour_message = RawMsgSubmitMisbehaviour {
        client_id: ctx.client_id.to_string(),
        misbehaviour: misbehaviour.into(),
        signer: get_dummy_account_id().to_string(),
    };

    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .misbehaviour(deps.as_mut(), info, misbehaviour_message)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: Other { description: \"Invalid Response Data\" } }"
)]
fn fails_on_empty_response_misbehaviour() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let event = Event::new("empty");

    let reply_message = Reply {
        id: EXECUTE_UPGRADE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: None,
        }),
    };

    contract
        .execute_misbehaviour_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcClientError { error: Other { description: \"UnkownError\" } }")]
fn fails_on_error_response_misbehaviour() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    let reply_message = Reply {
        id: EXECUTE_UPGRADE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Err("UnkownError".to_string()),
    };

    contract
        .execute_misbehaviour_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();
}

#[test]
fn success_on_execute_misbehaviour() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let client_state: ClientState = get_dummy_client_state();

    let client_id = ClientId::from_str("iconlightclient-10").unwrap();

    let response_message_data = MisbehaviourResponse::new(
        client_id.to_string(),
        client_state.get_keccak_hash().to_vec(),
        client_state.encode_to_vec(),
    );

    let event = Event::new("empty");

    let reply_message = Reply {
        id: EXECUTE_UPGRADE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(to_binary(&response_message_data).unwrap()),
        }),
    };

    let result = contract
        .execute_misbehaviour_reply(deps.as_mut(), env, reply_message)
        .unwrap();

    assert_eq!("client_misbehaviour", result.events[0].ty);
    assert_eq!(client_id.as_str(), result.events[0].attributes[0].value);
}

#[test]
#[should_panic(
    expected = "UnknownConsensusStateType { consensus_state_type: \"/ibc.mock.ConsensusState\" }"
)]
fn fails_on_raw_from_consensus_state() {
    let raw = get_dummy_raw_msg_create_client();

    TryInto::<ConsensusState>::try_into(raw.consensus_state.unwrap()).unwrap();
}

#[test]
#[should_panic(
    expected = "DecodeError { description: \"invalid wire type: LengthDelimited (expected Varint)"
)]
fn fails_on_deserialising_invalid_bytes_to_client_state() {
    let data = get_dummy_raw_msg_create_client();

    <ClientState>::decode(data.client_state.unwrap().value.as_slice()).unwrap();
}
