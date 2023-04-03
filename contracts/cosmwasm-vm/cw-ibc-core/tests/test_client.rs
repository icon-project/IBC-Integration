pub mod setup;

use std::str::FromStr;

use cosmwasm_std::{to_binary, to_vec, Addr, Event, Reply, SubMsgResponse};
use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics02_client::{
        events::{
            client_misbehaviour_event, create_client_event, generated_client_id_event,
            update_client_event, upgrade_client_event,
        },
        handler::{CreateClientResponse, UpdateClientResponse},
        types::{ClientState, ConsensusState},
    },
    traits::IbcClient,
    types::{ClientId, ClientType},
    MsgCreateClient, MsgUpdateClient, MsgUpgradeClient,
};
use ibc::{
    core::ics02_client::msgs::misbehaviour::MsgSubmitMisbehaviour,
    mock::{
        client_state::MockClientState, consensus_state::MockConsensusState, header::MockHeader,
    },
    signer::Signer,
    Height,
};

use setup::*;

#[test]
fn get_client_next_sequence() {
    let mut mock = deps();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(mock.as_mut().storage, 0)
        .unwrap();

    let result = contract.client_counter(mock.as_ref().storage).unwrap();

    assert_eq!(result, 0)
}

#[test]
fn increment_next_client_sequence() {
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

    let client_type = ClientType::new("new_cleint_type".to_string());

    let client_id = ClientId::new(client_type, 1).unwrap();

    let light_client_address = "light-client".to_string();

    contract
        .store_client_implementations(
            mock.as_mut().storage,
            client_id.clone(),
            light_client_address.clone(),
        )
        .unwrap();

    let result = contract
        .get_client_implementations(mock.as_ref().storage, client_id)
        .unwrap();

    assert_eq!(light_client_address, result)
}

#[test]
#[should_panic(expected = "InvalidClientId { client_id: \"new_cleint_type-1\" }")]
fn store_client_implement_failure() {
    let mock = deps();
    let contract = CwIbcCoreContext::default();

    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type, 1).unwrap();

    contract
        .get_client_implementations(mock.as_ref().storage, client_id)
        .unwrap();
}

#[test]
fn store_client_into_registry() {
    let mut mock = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_cleint_type".to_string());
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
#[should_panic(expected = "InvalidClientType { client_type: \"new_cleint_type\" }")]
fn fails_on_querying_client_from_registry() {
    let mock = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_cleint_type".to_string());
    contract
        .get_client_from_registry(mock.as_ref().storage, client_type)
        .unwrap();
}

#[test]
fn test_create_client_event() {
    let height = Height::new(15, 10).unwrap();

    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 1).unwrap();
    let result = create_client_event(
        client_id.ibc_client_id().as_str(),
        client_type.client_type().as_str(),
        &height.to_string(),
    );

    assert_eq!("create_client", result.ty)
}

#[test]
fn check_for_update_client_event() {
    let raw_message = get_dummy_raw_msg_update_client_message();
    let message: MsgUpdateClient = MsgUpdateClient::try_from(raw_message.clone()).unwrap();
    let height = Height::new(15, 10).unwrap();
    let client_type = ClientType::new("new_cleint_type".to_string());
    let result = update_client_event(
        client_type.client_type(),
        height,
        vec![height],
        &message.client_id,
    );

    assert_eq!("update_client", result.ty);
}

#[test]
fn check_for_raw_message_to_update_client_message() {
    let raw_message = get_dummy_raw_msg_update_client_message();
    let message: MsgUpdateClient = MsgUpdateClient::try_from(raw_message.clone()).unwrap();
    assert_eq!(raw_message, message.into())
}

#[test]
fn check_for_raw_message_to_updgrade_client() {
    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();
    let signer = get_dummy_account_id();

    let height = Height::new(1, 1).unwrap();

    let client_state = MockClientState::new(MockHeader::new(height));
    let consensus_state = MockConsensusState::new(MockHeader::new(height));

    let proof = get_dummy_merkle_proof();

    let msg = MsgUpgradeClient {
        client_id: client_id.ibc_client_id().clone(),
        client_state: client_state.into(),
        consensus_state: consensus_state.into(),
        proof_upgrade_client: proof.clone(),
        proof_upgrade_consensus_state: proof,
        signer,
    };

    let raw_message: ibc_proto::ibc::core::client::v1::MsgUpgradeClient =
        ibc_proto::ibc::core::client::v1::MsgUpgradeClient::try_from(msg.clone()).unwrap();

    let upgrade_message_from_raw_message = MsgUpgradeClient::try_from(raw_message).unwrap();

    assert_eq!(upgrade_message_from_raw_message, msg);
}

#[test]
fn test_upgrade_client_event() {
    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();
    let signer = get_dummy_account_id();

    let height = Height::new(1, 1).unwrap();

    let client_state = MockClientState::new(MockHeader::new(height));
    let consensus_state = MockConsensusState::new(MockHeader::new(height));

    let proof = get_dummy_merkle_proof();

    let msg = MsgUpgradeClient {
        client_id: client_id.ibc_client_id().clone(),
        client_state: client_state.into(),
        consensus_state: consensus_state.into(),
        proof_upgrade_client: proof.clone(),
        proof_upgrade_consensus_state: proof,
        signer,
    };

    let event = upgrade_client_event(client_type.client_type(), height, msg.client_id);

    assert_eq!("upgrade_client", event.ty);

    assert_eq!(event.attributes[0].value, "new_cleint_type-10")
}

#[test]
fn create_misbehaviour_event_test() {
    let raw_message = get_dummy_raw_msg_client_mishbehaviour();
    let misbehaviour: MsgSubmitMisbehaviour =
        MsgSubmitMisbehaviour::try_from(raw_message.clone()).unwrap();

    let raw_message_from_mb: ibc_proto::ibc::core::client::v1::MsgSubmitMisbehaviour =
        ibc_proto::ibc::core::client::v1::MsgSubmitMisbehaviour::try_from(misbehaviour).unwrap();

    assert_eq!(raw_message, raw_message_from_mb);

    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();

    let event =
        client_misbehaviour_event(client_id.ibc_client_id().clone(), client_type.client_type());

    assert_eq!("client_misbehaviour", event.ty)
}

#[test]
fn store_client_type_sucess() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("icon_client".to_string());

    let client_id = ClientId::new(client_type.clone(), 10).unwrap();

    contract
        .store_client_type(
            deps.as_mut().storage,
            client_id.clone(),
            client_type.clone(),
        )
        .unwrap();
    let result = contract
        .get_client_type(deps.as_ref().storage, client_id)
        .unwrap();

    assert_eq!(client_type.client_type(), result)
}

#[test]
#[should_panic(expected = "InvalidClientId { client_id: \"icon_client-10\" }")]
fn fail_to_query_client_type() {
    let deps = deps();

    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("icon_client".to_string());

    let client_id = ClientId::new(client_type.clone(), 10).unwrap();

    contract
        .get_client_type(deps.as_ref().storage, client_id)
        .unwrap();
}

#[test]
fn check_for_raw_message_create_client_deserialize() {
    let raw_message = get_dummy_raw_msg_create_client();
    let height = Height::new(10, 15).unwrap();
    let mock_header = MockHeader::new(height);
    let mock_client_state = MockClientState::new(mock_header);
    let mock_consenus_state = MockConsensusState::new(mock_header);
    let actual_message = MsgCreateClient {
        client_state: mock_client_state.into(),
        consensus_state: mock_consenus_state.into(),
        signer: get_dummy_account_id(),
    };

    let create_client_message: MsgCreateClient = MsgCreateClient::try_from(raw_message).unwrap();

    assert_eq!(create_client_message, actual_message)
}

#[test]
fn check_for_create_client_message_into_raw_message() {
    let height = Height::new(10, 15).unwrap();
    let mock_header = MockHeader::new(height);
    let mock_client_state = MockClientState::new(mock_header);
    let mock_consenus_state = MockConsensusState::new(mock_header);
    let actual_message = MsgCreateClient {
        client_state: mock_client_state.into(),
        consensus_state: mock_consenus_state.into(),
        signer: get_dummy_account_id(),
    };

    let raw_message: ibc_proto::ibc::core::client::v1::MsgCreateClient =
        ibc_proto::ibc::core::client::v1::MsgCreateClient::try_from(actual_message).unwrap();

    assert_eq!(raw_message, get_dummy_raw_msg_create_client())
}

#[test]
fn check_for_genereted_client_id_event() {
    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();
    let event = generated_client_id_event(client_id.ibc_client_id().clone());

    assert_eq!("client_id_created", event.ty);

    assert_eq!(
        event.attributes[0].value,
        client_id.ibc_client_id().as_str()
    )
}

#[test]
fn check_for_create_client_message() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract.register_client(deps.as_mut(), client_type, light_client);

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message =
        MsgCreateClient::new(client_state.into(), consenus_state.into(), signer);

    let response = contract
        .create_client(deps.as_mut(), info, create_client_message)
        .unwrap();

    assert_eq!(response.messages[0].id, 21);

    assert_eq!(response.attributes[0].value, "create_client");
}

#[test]
fn check_for_create_client_message_response() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract.register_client(deps.as_mut(), client_type.clone(), light_client);

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = MsgCreateClient::new(
        client_state.clone().into(),
        consenus_state.clone().into(),
        signer,
    );

    let response = contract
        .create_client(deps.as_mut(), info, create_client_message)
        .unwrap();
    assert_eq!(response.messages[0].id, 21);

    assert_eq!(response.attributes[0].value, "create_client");

    let mock_reponse_data = CreateClientResponse::new(
        client_type.as_str().to_string(),
        "10-15".to_string(),
        to_vec(&client_state).unwrap(),
        to_vec(&consenus_state).unwrap(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 21,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    let result = contract
        .execute_create_client_reply(deps.as_mut(), reply_message)
        .unwrap();

    assert_eq!(result.attributes[0].value, "execute_create_client_reply");
    assert_eq!(result.attributes[1].value, "iconclient-0");

    assert_eq!(result.events[0].ty, "create_client");
    assert_eq!(result.events[0].attributes[0].value, "iconclient-0");
    assert_eq!(result.events[0].attributes[1].value, "iconclient");
    assert_eq!(result.events[0].attributes[2].value, "10-15");
}

#[test]
fn check_for_client_state_from_storage() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    println!("1 {:?}", consenus_state);

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract.register_client(deps.as_mut(), client_type.clone(), light_client);

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = MsgCreateClient::new(
        client_state.clone().into(),
        consenus_state.clone().into(),
        signer,
    );

    contract
        .create_client(deps.as_mut(), info, create_client_message)
        .unwrap();

    let mock_reponse_data = CreateClientResponse::new(
        client_type.as_str().to_string(),
        "10-15".to_string(),
        to_vec(&client_state).unwrap(),
        consenus_state.try_into().unwrap(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 21,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    contract
        .execute_create_client_reply(deps.as_mut(), reply_message)
        .unwrap();

    let client_id = ibc::core::ics24_host::identifier::ClientId::from_str("iconclient-0").unwrap();

    let client_state = contract
        .client_state(deps.as_ref().storage, &client_id)
        .unwrap();

    assert_eq!(client_state.client_type().as_str(), "iconclient");
}

#[test]
fn check_for_consensus_state_from_storage() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract.register_client(deps.as_mut(), client_type.clone(), light_client);

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = MsgCreateClient::new(
        client_state.clone().into(),
        consenus_state.clone().into(),
        signer,
    );

    contract
        .create_client(deps.as_mut(), info, create_client_message)
        .unwrap();

    let mock_reponse_data = CreateClientResponse::new(
        client_type.as_str().to_string(),
        "10-15".to_string(),
        to_vec(&client_state).unwrap(),
        consenus_state.clone().try_into().unwrap(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 21,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    contract
        .execute_create_client_reply(deps.as_mut(), reply_message)
        .unwrap();

    let client_id = ibc::core::ics24_host::identifier::ClientId::from_str("iconclient-0").unwrap();

    let height = Height::new(10, 15).unwrap();

    let consensus_state_result =
        contract.consensus_state(deps.as_ref().storage, &client_id, &height);

    assert!(consensus_state_result.is_ok());
    assert_eq!(
        [1, 2, 3, 4],
        consensus_state_result.unwrap().root().as_bytes()
    )
}

#[test]
#[should_panic(expected = "IbcClientError { error: Other { description: \"invalid_response\" } }")]
fn fail_on_create_client_message_error_response() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract.register_client(deps.as_mut(), client_type.clone(), light_client);

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = MsgCreateClient::new(
        client_state.clone().into(),
        consenus_state.clone().into(),
        signer,
    );

    let response = contract
        .create_client(deps.as_mut(), info, create_client_message)
        .unwrap();
    assert_eq!(response.messages[0].id, 21);

    assert_eq!(response.attributes[0].value, "create_client");

    let reply_message = Reply {
        id: 21,
        result: cosmwasm_std::SubMsgResult::Err("invalid_response".to_string()),
    };

    contract
        .execute_create_client_reply(deps.as_mut(), reply_message)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidNextClientSequence")]
fn fails_on_create_client_message_without_proper_initialisation() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract.register_client(deps.as_mut(), client_type, light_client);

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message =
        MsgCreateClient::new(client_state.into(), consenus_state.into(), signer);

    contract
        .create_client(deps.as_mut(), info, create_client_message)
        .unwrap();
}

#[test]
fn check_for_update_client_message() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract.register_client(deps.as_mut(), client_type.clone(), light_client);

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = MsgCreateClient::new(
        client_state.clone().into(),
        consenus_state.clone().into(),
        signer.clone(),
    );

    let response = contract
        .create_client(deps.as_mut(), info.clone(), create_client_message)
        .unwrap();
    assert_eq!(response.messages[0].id, 21);

    assert_eq!(response.attributes[0].value, "create_client");

    let mock_reponse_data = CreateClientResponse::new(
        client_type.as_str().to_string(),
        "0-25".to_string(),
        to_vec(&client_state).unwrap(),
        to_vec(&consenus_state).unwrap(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 21,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    let client_id = ClientId::from_str("iconclient-0").unwrap();

    contract
        .execute_create_client_reply(deps.as_mut(), reply_message)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 3,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let update_client_message = MsgUpdateClient {
        client_id: client_id.ibc_client_id().clone(),
        header: client_state.clone().into(),
        signer,
    };

    let result = contract
        .update_client(deps.as_mut(), info, update_client_message)
        .unwrap();

    assert_eq!(
        client_id.ibc_client_id().as_str(),
        result.attributes[1].value
    );

    let mock_reponse_data = UpdateClientResponse::new(
        "10-15".to_string(),
        client_id.ibc_client_id().as_str().to_string(),
        to_vec(&client_state).unwrap(),
        to_vec(&consenus_state).unwrap(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 22,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    let update_response = contract.execute_update_client_reply(deps.as_mut(), reply_message);

    assert!(update_response.is_ok());

    let result = update_response.unwrap();

    assert_eq!("execute_update_client_reply", result.attributes[0].value);

    assert_eq!("10-15", result.attributes[1].value);

    assert_eq!("update_client", result.events[0].ty);

    assert_eq!("iconclient-0", result.events[0].attributes[0].value)
}

#[test]
#[should_panic(expected = "InvalidClientId { client_id: \"iconclient-0\" }")]
fn fails_on_updating_non_existing_client() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let client_id = ClientId::from_str("iconclient-0").unwrap();
    let signer = Signer::from_str("new_signer").unwrap();
    let update_client_message = MsgUpdateClient {
        client_id: client_id.ibc_client_id().clone(),
        header: client_state.clone().into(),
        signer,
    };

    contract
        .update_client(deps.as_mut(), info, update_client_message)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcClientError { error: Other { description: \"response_error\" } }")]
fn fails_on_error_ressponse() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    let reply_message = Reply {
        id: 22,
        result: cosmwasm_std::SubMsgResult::Err("response_error".to_string()),
    };
    contract
        .execute_update_client_reply(deps.as_mut(), reply_message)
        .unwrap();
}
