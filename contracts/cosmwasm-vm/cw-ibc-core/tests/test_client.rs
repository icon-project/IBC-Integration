pub mod setup;

use std::str::FromStr;

use common::client_state::{get_default_icon_client_state, IClientState};
use common::ibc::{
    core::ics02_client::msgs::misbehaviour::MsgSubmitMisbehaviour, signer::Signer, Height,
};
use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};
use common::traits::AnyTypes;
use common::utils::keccak256;
use cosmwasm_std::{to_binary, Addr, Event, Reply, SubMsgResponse};
use cw_common::client_response::{
    CreateClientResponse, MisbehaviourResponse, UpdateClientResponse, UpgradeClientResponse,
};
use cw_common::ibc_types::{IbcMsgCreateClient, IbcMsgUpdateClient};
use cw_common::raw_types::client::{RawMsgCreateClient, RawMsgUpgradeClient};
use cw_common::raw_types::Any;

use common::ibc::core::ics02_client::client_type::ClientType;
use common::ibc::core::ics24_host::identifier::ClientId;
use common::ibc::mock::header::MockHeader;
use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics02_client::events::{
        client_misbehaviour_event, create_client_event, generated_client_id_event,
        update_client_event, upgrade_client_event,
    },
    traits::IbcClient,
    MsgUpgradeClient,
};
use debug_print::debug_println;
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
#[should_panic(expected = "InvalidClientId { client_id: \"new_client_type-1\" }")]
fn store_client_implement_failure() {
    let mock = deps();
    let contract = CwIbcCoreContext::default();

    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 1).unwrap();

    contract
        .get_client_implementations(mock.as_ref().storage, client_id)
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
fn check_for_update_client_event() {
    let raw_message = get_dummy_raw_msg_update_client_message();
    let message: IbcMsgUpdateClient = IbcMsgUpdateClient::try_from(raw_message).unwrap();
    let height = Height::new(15, 10).unwrap();
    let client_type = ClientType::new("new_client_type".to_string());
    let result = update_client_event(client_type, height, vec![height], &message.client_id);

    assert_eq!("update_client", result.ty);
}

#[test]
fn check_for_raw_message_to_update_client_message() {
    let raw_message = get_dummy_raw_msg_update_client_message();
    let message: IbcMsgUpdateClient = IbcMsgUpdateClient::try_from(raw_message.clone()).unwrap();
    assert_eq!(raw_message, message.into())
}

#[test]
fn check_for_raw_message_to_updgrade_client() {
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 10).unwrap();
    let signer = get_dummy_account_id();

    let height = mock_height(1, 1).unwrap();

    let client_state = MockClientState::new(MockHeader::new(height));
    let consensus_state = MockConsensusState::new(MockHeader::new(height));

    let proof = get_dummy_merkle_proof();

    let msg = MsgUpgradeClient {
        client_id,
        client_state: client_state.into(),
        consensus_state: consensus_state.into(),
        proof_upgrade_client: proof.clone(),
        proof_upgrade_consensus_state: proof,
        signer,
    };

    let raw_message: RawMsgUpgradeClient = RawMsgUpgradeClient::try_from(msg.clone()).unwrap();

    let upgrade_message_from_raw_message = MsgUpgradeClient::try_from(raw_message).unwrap();

    assert_eq!(upgrade_message_from_raw_message, msg);
}
#[test]
fn test_upgrade_client_event() {
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();
    let signer = get_dummy_account_id();

    let height = Height::new(1, 1).unwrap();
    let mock_height = to_mock_height(height);

    let client_state = MockClientState::new(MockHeader::new(mock_height));
    let consensus_state = MockConsensusState::new(MockHeader::new(mock_height));

    let proof = get_dummy_merkle_proof();

    let msg = MsgUpgradeClient {
        client_id,
        client_state: client_state.into(),
        consensus_state: consensus_state.into(),
        proof_upgrade_client: proof.clone(),
        proof_upgrade_consensus_state: proof,
        signer,
    };

    let event = upgrade_client_event(client_type, height, msg.client_id);

    assert_eq!("upgrade_client", event.ty);

    assert_eq!(event.attributes[0].value, "new_client_type-10")
}

#[test]
fn create_misbehaviour_event_test() {
    use cw_common::raw_types::client::RawMsgSubmitMisbehaviour;
    let raw_message = get_dummy_raw_msg_client_mishbehaviour();
    let misbehaviour: MsgSubmitMisbehaviour =
        MsgSubmitMisbehaviour::try_from(raw_message.clone()).unwrap();

    let raw_message_from_mb: RawMsgSubmitMisbehaviour =
        RawMsgSubmitMisbehaviour::try_from(misbehaviour).unwrap();

    assert_eq!(raw_message, raw_message_from_mb);

    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();

    let event = client_misbehaviour_event(client_id.as_str(), client_type.as_str());

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
        .get_client_type(deps.as_ref().storage, client_id)
        .unwrap();
}

#[test]
fn check_for_raw_message_create_client_deserialize() {
    let raw_message = get_dummy_raw_msg_create_client();
    let height = mock_height(10, 15).unwrap();
    let mock_header = MockHeader::new(height);
    let mock_client_state = MockClientState::new(mock_header);
    let mock_consenus_state = MockConsensusState::new(mock_header);
    let actual_message = IbcMsgCreateClient {
        client_state: mock_client_state.into(),
        consensus_state: mock_consenus_state.into(),
        signer: get_dummy_account_id(),
    };

    let create_client_message: IbcMsgCreateClient =
        IbcMsgCreateClient::try_from(raw_message).unwrap();

    assert_eq!(create_client_message, actual_message)
}

#[test]
fn check_for_create_client_message_into_raw_message() {
    let height = mock_height(10, 15).unwrap();
    let mock_header = MockHeader::new(height);
    let mock_client_state = MockClientState::new(mock_header);
    let mock_consenus_state = MockConsensusState::new(mock_header);
    let actual_message = IbcMsgCreateClient {
        client_state: mock_client_state.into(),
        consensus_state: mock_consenus_state.into(),
        signer: get_dummy_account_id(),
    };

    let raw_message: RawMsgCreateClient = RawMsgCreateClient::try_from(actual_message).unwrap();
    debug_println!("{raw_message:?}");
    debug_println!("{:?}", get_dummy_raw_msg_create_client());

    assert_eq!(raw_message, get_dummy_raw_msg_create_client())
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
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract
        .register_client(deps.as_mut(), client_type, light_client)
        .unwrap();

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message =
        IbcMsgCreateClient::new(client_state.into(), consenus_state.into(), signer);

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

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = IbcMsgCreateClient::new(
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
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.encode_to_vec(),
        consenus_state.encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
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
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = IbcMsgCreateClient::new(
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
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.to_any().encode_to_vec(),
        consenus_state.to_any().encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    let client_id =
        common::ibc::core::ics24_host::identifier::ClientId::from_str("iconclient-0").unwrap();

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
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = IbcMsgCreateClient::new(
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
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.to_any().encode_to_vec(),
        consenus_state.to_any().encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    let client_id =
        common::ibc::core::ics24_host::identifier::ClientId::from_str("iconclient-0").unwrap();

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
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract
        .register_client(deps.as_mut(), client_type, light_client)
        .unwrap();

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message =
        IbcMsgCreateClient::new(client_state.into(), consenus_state.into(), signer);

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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
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
      

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract
        .register_client(deps.as_mut(), client_type, light_client)
        .unwrap();

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message =
        IbcMsgCreateClient::new(client_state.into(), consenus_state.into(), signer);

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
        

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = IbcMsgCreateClient::new(
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
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.encode_to_vec(),
        consenus_state.encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 3,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let update_client_message = IbcMsgUpdateClient {
        client_id: client_id.clone(),
        header: client_state.clone().into(),
        signer,
    };

    let result = contract
        .update_client(deps.as_mut(), info, update_client_message)
        .unwrap();

    assert_eq!(client_id.as_str(), result.attributes[1].value);

    let mock_reponse_data = UpdateClientResponse::new(
        "10-15".to_string(),
        client_id.as_str().to_string(),
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.encode_to_vec(),
        consenus_state.encode_to_vec(),
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
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let client_id = ClientId::from_str("iconclient-0").unwrap();
    let signer = Signer::from_str("new_signer").unwrap();
    let update_client_message = IbcMsgUpdateClient {
        client_id,
        header: client_state.into(),
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
        .execute_update_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();
}

#[test]
fn check_for_upgrade_client() {
    let mut deps = deps();

    let info = create_mock_info("alice", "umlg", 2000);
    let env = get_mock_env();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");

    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2000000000,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let mock_reponse_data = CreateClientResponse::new(
        client_type.as_str().to_string(),
        "0-100".to_string(),
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.to_any().encode_to_vec(),
        consenus_state.to_any().encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    let upgrade_client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 200000000,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let upgrade_consenus_state: ConsensusState =
        common::icon::icon::lightclient::v1::ConsensusState {
            message_root: "message_root_new".as_bytes().to_vec(),
            next_proof_context_hash: vec![1, 2, 3, 4],
        }
        .try_into()
        .unwrap();
    let signer = Signer::from_str("new_signer").unwrap();

    let upgrdade_client_message = MsgUpgradeClient {
        client_id,
        client_state: upgrade_client_state.into(),
        consensus_state: upgrade_consenus_state.into(),
        proof_upgrade_client: get_dummy_merkle_proof(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof(),
        signer,
    };

    let result = contract
        .upgrade_client(deps.as_mut(), info, env, upgrdade_client_message)
        .unwrap();

    assert_eq!("upgrade_client", result.attributes[0].value)
}

#[test]
#[should_panic(expected = "IbcClientError { error: HeaderNotWithinTrustPeriod")]
fn fails_on_upgrade_client_invalid_trusting_period() {
    let mut deps = deps();

    let info = create_mock_info("alice", "umlg", 2000);
    let env = get_mock_env();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");

    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let mock_reponse_data = CreateClientResponse::new(
        client_type.as_str().to_string(),
        "0-100".to_string(),
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.to_any().encode_to_vec(),
        consenus_state.to_any().encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    let upgrade_client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 200000000,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let upgrade_consenus_state: ConsensusState =
        common::icon::icon::lightclient::v1::ConsensusState {
            message_root: "message_root_new".as_bytes().to_vec(),
            next_proof_context_hash: vec![1, 2, 3, 4],
        }
        .try_into()
        .unwrap();
    let signer = Signer::from_str("new_signer").unwrap();

    let upgrdade_client_message = MsgUpgradeClient {
        client_id,
        client_state: upgrade_client_state.into(),
        consensus_state: upgrade_consenus_state.into(),
        proof_upgrade_client: get_dummy_merkle_proof(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof(),
        signer,
    };

    contract
        .upgrade_client(deps.as_mut(), info, env, upgrdade_client_message)
        .unwrap();
}

#[test]
#[should_panic(
    expected = " IbcClientError { error: ClientFrozen { client_id: ClientId(\"iconclient-0\") } }"
)]
fn fails_on_upgrade_client_frozen_client() {
    let mut deps = deps();

    let info = create_mock_info("alice", "umlg", 2000);
    let env = get_mock_env();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");

    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 3,
        max_clock_drift: 5,
        latest_height: 100,
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let mock_reponse_data = CreateClientResponse::new(
        client_type.as_str().to_string(),
        "0-100".to_string(),
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.to_any().encode_to_vec(),
        consenus_state.to_any().encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    let upgrade_client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 200000000,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let upgrade_consenus_state: ConsensusState =
        common::icon::icon::lightclient::v1::ConsensusState {
            message_root: "message_root_new".as_bytes().to_vec(),
            next_proof_context_hash: vec![1, 2, 3, 4],
        }
        .try_into()
        .unwrap();
    let signer = Signer::from_str("new_signer").unwrap();

    let upgrdade_client_message = MsgUpgradeClient {
        client_id,
        client_state: upgrade_client_state.into(),
        consensus_state: upgrade_consenus_state.into(),
        proof_upgrade_client: get_dummy_merkle_proof(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof(),
        signer,
    };

    contract
        .upgrade_client(deps.as_mut(), info, env, upgrdade_client_message)
        .unwrap();
}

#[test]
fn check_for_execute_upgrade_client() {
    let mut deps = deps();

    let info = create_mock_info("alice", "umlg", 2000);
    let env = get_mock_env();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");

    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2000000000,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
      

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let mock_reponse_data = CreateClientResponse::new(
        client_type.as_str().to_string(),
        "0-100".to_string(),
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.to_any().encode_to_vec(),
        consenus_state.to_any().encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    let upgrade_client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 200000000,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
      

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let upgrade_consenus_state: ConsensusState =
        common::icon::icon::lightclient::v1::ConsensusState {
            message_root: "message_root_new".as_bytes().to_vec(),
            next_proof_context_hash: vec![1, 2, 3, 4],
        }
        .try_into()
        .unwrap();
    let signer = Signer::from_str("new_signer").unwrap();

    let upgrdade_client_message = MsgUpgradeClient {
        client_id: client_id.clone(),
        client_state: upgrade_client_state.clone().into(),
        consensus_state: upgrade_consenus_state.clone().into(),
        proof_upgrade_client: get_dummy_merkle_proof(),
        proof_upgrade_consensus_state: get_dummy_merkle_proof(),
        signer,
    };

    contract
        .upgrade_client(deps.as_mut(), info, env, upgrdade_client_message)
        .unwrap();

    let upgrade_client_response = UpgradeClientResponse::new(
        upgrade_client_state.get_keccak_hash().to_vec(),
        upgrade_client_state.encode_to_vec(),
        upgrade_consenus_state.get_keccak_hash().to_vec(),
        upgrade_consenus_state.encode_to_vec(),
        client_id.to_string(),
        "0-100".to_string(),
    );

    let mock_data_binary = to_binary(&upgrade_client_response).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 23,
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
    let mut deps = deps();

    let env = get_mock_env();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let upgrade_client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 200000000,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
      

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let upgrade_consenus_state: ConsensusState =
        common::icon::icon::lightclient::v1::ConsensusState {
            message_root: "message_root_new".as_bytes().to_vec(),
            next_proof_context_hash: vec![1, 2, 3, 4],
        }
        .try_into()
        .unwrap();

    let upgrade_client_response = UpgradeClientResponse::new(
        upgrade_client_state.get_keccak_hash().to_vec(),
        upgrade_client_state.encode_to_vec(),
        upgrade_consenus_state.get_keccak_hash().to_vec(),
        upgrade_consenus_state.encode_to_vec(),
        "hello".to_string(),
        "0-100".to_string(),
    );

    let mock_data_binary = to_binary(&upgrade_client_response).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 23,
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
    let mut deps = deps();

    let env = get_mock_env();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let reply_message = Reply {
        id: 23,
        result: cosmwasm_std::SubMsgResult::Err("UnknownResponse".to_string()),
    };

    contract
        .execute_upgrade_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: Other { description: \"Invalid Response Data\" } }"
)]
fn fails_on_null_response_data_on_execute_upgrade_client() {
    let mut deps = deps();

    let env = get_mock_env();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 0)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 23,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: None,
        }),
    };

    contract
        .execute_upgrade_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: Other { description: \"Client Implementation Already Exist\" } }"
)]
fn fails_on_storing_already_registered_client_into_registry() {
    let mut mock_deps = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_client_type".to_string());
    let light_client_address = "light-client".to_string();
    contract
        .store_client_into_registry(
            mock_deps.as_mut().storage,
            client_type.clone(),
            light_client_address.clone(),
        )
        .unwrap();

    let result = contract
        .get_client_from_registry(mock_deps.as_ref().storage, client_type.clone())
        .unwrap();

    assert_eq!(light_client_address, result);

    contract
        .register_client(
            mock_deps.as_mut(),
            client_type,
            Addr::unchecked(light_client_address),
        )
        .unwrap();
}

#[test]
fn sucess_on_getting_client() {
    let mut mock_deps = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 0).unwrap();

    let client_address = "newclientaddress".to_string();

    contract
        .store_client_implementations(
            mock_deps.as_mut().storage,
            client_id.clone(),
            client_address.clone(),
        )
        .unwrap();

    let result = contract
        .get_client(mock_deps.as_ref().storage, client_id)
        .unwrap();

    assert_eq!(result, client_address)
}

#[test]
#[should_panic(expected = "InvalidClientId { client_id: \"new_client_type-0\" }")]
fn fails_on_getting_client_invalid_client() {
    let mock_deps = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 0).unwrap();

    contract
        .get_client(mock_deps.as_ref().storage, client_id)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: ClientNotFound { client_id: ClientId(\"new_client_type-0\") } }"
)]
fn fails_on_getting_client_empty_client() {
    let mut mock_deps = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 0).unwrap();

    let client_address = "".to_string();

    contract
        .store_client_implementations(
            mock_deps.as_mut().storage,
            client_id.clone(),
            client_address,
        )
        .unwrap();

    contract
        .get_client(mock_deps.as_ref().storage, client_id)
        .unwrap();
}

#[test]
fn success_on_getting_client_state() {
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
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let client_type = ClientType::new("iconclient".to_string());
    let light_client = Addr::unchecked("lightclient");
    contract
        .register_client(deps.as_mut(), client_type.clone(), light_client)
        .unwrap();

    let signer = Signer::from_str("new_signer").unwrap();

    let create_client_message = IbcMsgCreateClient::new(
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
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.to_any().encode_to_vec(),
        consenus_state.to_any().encode_to_vec(),
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
        .execute_create_client_reply(deps.as_mut(), get_mock_env(), reply_message)
        .unwrap();

    let client_id = ClientId::from_str("iconclient-0").unwrap();

    let state = contract
        .get_client_state(deps.as_mut().storage, client_id)
        .unwrap();
    let client_state_any = Any::decode(state.as_slice()).unwrap();
    let client_state: ClientState = ClientState::from_any(client_state_any).unwrap();
    let client_state: Box<dyn IClientState> = Box::new(client_state);

    assert_eq!(None, client_state.frozen_height())
}

#[test]
#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"NotFound ClientId(iconclient-0)\", stack: [] } }"
)]
fn fails_on_getting_client_state() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    let client_id = ClientId::from_str("iconclient-0").unwrap();

    contract
        .get_client_state(deps.as_mut().storage, client_id)
        .unwrap();
}

#[test]
fn sucess_on_misbehaviour_validate() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);
    contract
        .init_client_counter(deps.as_mut().storage, 10)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
       

        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let client_id = ClientId::from_str("iconlightclient-10").unwrap();

    contract
        .store_client_implementations(
            deps.as_mut().storage,
            client_id.clone(),
            "clientaddress".to_string(),
        )
        .unwrap();

    contract
        .store_client_state(
            deps.as_mut().storage,
            &get_mock_env(),
            &client_id,
            client_state.to_any().encode_to_vec(),
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let height = mock_height(10, 15).unwrap();
    let mock_header = MockHeader::new(height);

    let misbehaviour = common::ibc::mock::misbehaviour::Misbehaviour {
        client_id: to_mock_client_id(&client_id),
        header1: mock_header,
        header2: mock_header,
    };

    let misbehaviour_message = MsgSubmitMisbehaviour {
        client_id,
        misbehaviour: misbehaviour.into(),
        signer: get_dummy_account_id(),
    };

    let result = contract.misbehaviour(deps.as_mut(), info, misbehaviour_message);

    assert!(result.is_ok())
}

#[test]
#[should_panic(
    expected = "IbcClientError { error: ClientFrozen { client_id: ClientId(\"iconlightclient-10\") } }"
)]
fn fails_on_frozen_client_on_misbehaviour_validate() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("alice", "umlg", 2000);
    contract
        .init_client_counter(deps.as_mut().storage, 10)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 10,
        max_clock_drift: 5,
        latest_height: 100,
       
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let client_id = ClientId::from_str("iconlightclient-10").unwrap();

    contract
        .store_client_implementations(
            deps.as_mut().storage,
            client_id.clone(),
            "clientaddress".to_string(),
        )
        .unwrap();

    contract
        .store_client_state(
            deps.as_mut().storage,
            &get_mock_env(),
            &client_id,
            client_state.to_any().encode_to_vec(),
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let height = mock_height(10, 15).unwrap();
    let mock_header = MockHeader::new(height);

    let misbehaviour = common::ibc::mock::misbehaviour::Misbehaviour {
        client_id: to_mock_client_id(&client_id),
        header1: mock_header,
        header2: mock_header,
    };

    let misbehaviour_message = MsgSubmitMisbehaviour {
        client_id,
        misbehaviour: misbehaviour.into(),
        signer: get_dummy_account_id(),
    };

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
        id: 23,
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
        id: 23,
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
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 10,
        max_clock_drift: 5,
        latest_height: 100,
       
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

    let client_id = ClientId::from_str("iconlightclient-10").unwrap();

    let response_message_data = MisbehaviourResponse::new(
        client_id.to_string(),
        client_state.get_keccak_hash().to_vec(),
        client_state.encode_to_vec(),
    );

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 23,
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
fn success_on_raw_from_consensus_state() {
    let raw = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    };

    let consenus_state: ConsensusState = raw.clone().try_into().unwrap();

    let raw_message: common::icon::icon::lightclient::v1::ConsensusState =
        consenus_state.try_into().unwrap();

    assert_eq!(raw, raw_message)
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
