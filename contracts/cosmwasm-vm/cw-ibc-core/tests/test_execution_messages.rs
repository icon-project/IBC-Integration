pub mod setup;

use std::str::FromStr;
use std::time::Duration;

use common::ibc::core::ics24_host::identifier::ClientId;
use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};
use common::icon::icon::types::v1::BtpHeader as RawBtpHeader;
use common::icon::icon::types::v1::MerkleNode as RawMerkleNode;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use common::utils::keccak256;
use cosmwasm_std::ContractResult;
use cosmwasm_std::SystemResult;
use cosmwasm_std::WasmQuery;
use cosmwasm_std::{to_binary, Addr, Event, Reply, SubMsgResponse};
use cw_common::client_response::{CreateClientResponse, UpdateClientResponse};
use cw_common::core_msg::ExecuteMsg;
use cw_common::hex_string::HexString;
use cw_common::ibc_types::IbcClientId;
use cw_common::raw_types::client::{RawMsgCreateClient, RawMsgUpdateClient};
use cw_common::raw_types::connection::RawMsgConnectionOpenInit;

use cw_common::ProstMessage;

use cw_ibc_core::conversions::{to_ibc_client_id, to_ibc_connection_id, to_ibc_height};
use cw_ibc_core::{
    ConnectionEnd, EXECUTE_CONNECTION_OPENTRY, EXECUTE_CREATE_CLIENT, EXECUTE_UPDATE_CLIENT,
};

use cw_ibc_core::light_client::light_client::LightClient;
use cw_ibc_core::{context::CwIbcCoreContext, msg::InstantiateMsg};

use common::ibc::core::ics24_host::identifier::ConnectionId;
use common::icon::icon::lightclient::v1::ClientState as RawClientState;
use common::icon::icon::lightclient::v1::ConsensusState as RawConsensusState;
use common::traits::AnyTypes;
use cw_common::core_msg::ExecuteMsg as CoreExecuteMsg;
use setup::*;

#[test]
fn test_for_create_client_execution_message() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcCoreContext::default();
    let env = get_mock_env();

    let client_state: RawClientState = get_dummy_client_state();

    let consenus_state: RawConsensusState = RawConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");
    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            CoreExecuteMsg::RegisterClient {
                client_type: "iconclient".to_string(),
                client_address: Addr::unchecked("lightclientaddress"),
            },
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "register_client");
    let msg_raw = RawMsgCreateClient {
        client_state: Some(client_state.to_any()),
        consensus_state: Some(consenus_state.to_any()),
        signer: "raw_message".to_owned(),
    };

    let create_client_message = CoreExecuteMsg::CreateClient {
        msg: HexString::from_bytes(&msg_raw.encode_to_vec()),
    };

    let response = contract
        .execute(deps.as_mut(), env.clone(), info, create_client_message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "create_client");

    let mock_reponse_data = CreateClientResponse::new(
        "iconclient".to_string(),
        "10-15".to_string(),
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state.encode_to_vec(),
        consenus_state.encode_to_vec(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: EXECUTE_CREATE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message).unwrap();

    assert_eq!(response.attributes[0].value, "execute_create_client_reply")
}

#[test]
fn test_for_update_client_execution_messages() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcCoreContext::default();
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");
    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            CoreExecuteMsg::RegisterClient {
                client_type: "iconclient".to_string(),
                client_address: Addr::unchecked("lightclientaddress"),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "register_client");

    let client_state: ClientState = get_dummy_client_state();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();
    let client_state_any=client_state.to_any();
    let consensus_state_any=consenus_state.to_any();
    let mock_reponse_data = CreateClientResponse::new(
        "iconclient".to_string(),
        "10-15".to_string(),
        keccak256(&client_state.encode_to_vec()).to_vec(),
        keccak256(&consenus_state.encode_to_vec()).to_vec(),
        client_state_any.encode_to_vec(),
        consensus_state_any.encode_to_vec(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: EXECUTE_CREATE_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    contract
        .reply(deps.as_mut(), env.clone(), reply_message)
        .unwrap();

    let merkle_node = RawMerkleNode {
        dir: 0,
        value: vec![0, 1, 2],
    };

    let btp_header = RawBtpHeader {
        main_height: 27,
        round: 0,
        next_proof_context_hash: hex::decode(
            "d090304264eeee3c3562152f2dc355601b0b423a948824fd0a012c11c3fc2fb4",
        )
        .unwrap(),
        network_section_to_root: vec![merkle_node],
        network_id: 1,
        update_number: 0,
        prev_network_section_hash: hex::decode(
            "b791b4b069c561ca31093f825f083f6cc3c8e5ad5135625becd2ff77a8ccfa1e",
        )
        .unwrap(),
        message_count: 1,
        message_root: hex::decode(
            "7702db70e830e07b4ff46313456fc86d677c7eeca0c011d7e7dcdd48d5aacfe2",
        )
        .unwrap(),
        next_validators: vec![hex::decode("00b040bff300eee91f7665ac8dcf89eb0871015306").unwrap()],
    };

    let signed_header: RawSignedHeader = RawSignedHeader {
        header: Some(btp_header),
        signatures: vec![hex::decode("6c8b2bc2c3d31e34bd4ed9db6eff7d5dc647b13c58ae77d54e0b05141cb7a7995102587f1fa33fd56815463c6b78e100217c29ddca20fcace80510e3dab03a1600").unwrap()],
        current_validators: vec![hex::decode("00b040bff300eee91f7665ac8dcf89eb0871015306").unwrap()],
        trusted_height: 26,
    }
    .try_into()
    .unwrap();

    let msg_hex = RawMsgUpdateClient {
        client_id: "iconclient-0".to_string(),
        header: Some(signed_header.to_any()),
        signer: "signeraddress".to_string(),
    };

    let message = CoreExecuteMsg::UpdateClient {
        msg: HexString::from_bytes(&msg_hex.encode_to_vec()),
    };

    let response = contract
        .execute(deps.as_mut(), env.clone(), info, message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "update_client");

    let mock_reponse_data = UpdateClientResponse::new(
        "10-15".to_string(),
        "iconclient-0".to_string(),
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

    let response = contract.reply(deps.as_mut(), env, reply_message).unwrap();

    assert_eq!(response.attributes[0].value, "execute_update_client_reply")
}

#[test]
fn test_for_connection_open_init() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = get_mock_env();
    let mut contract = CwIbcCoreContext::new();

    let message = RawMsgConnectionOpenInit {
        client_id: "iconclient-0".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(Some(0))),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };

    contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    contract
        .store_client_implementations(
            deps.as_mut().storage,
            &ClientId::from_str("iconclient-0").unwrap(),
            LightClient::new("lightclientaddress".to_string()),
        )
        .unwrap();
    mock_lightclient_reply(&mut deps);
    let client_state_any = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &IbcClientId::from_str(&message.client_id).unwrap(),
            client_state_any,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    contract
        .client_state(
            &mut deps.storage,
            &IbcClientId::from_str(&message.client_id).unwrap(),
        )
        .unwrap();
    contract
        .connection_next_sequence_init(&mut deps.storage, u64::default())
        .unwrap();

    let exec_message = CoreExecuteMsg::ConnectionOpenInit {
        msg: HexString::from_bytes(&message.encode_to_vec()),
    };

    deps.querier.update_wasm(|r| match r {
        WasmQuery::Smart {
            contract_addr: _,
            msg: _,
        } => SystemResult::Ok(ContractResult::Ok(to_binary(&vec![0, 1, 2, 3]).unwrap())),
        _ => todo!(),
    });

    let response = contract
        .execute(deps.as_mut(), env, info, exec_message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "connection_open_init");
}

#[test]
fn test_for_connection_open_try() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = get_mock_env();
    let mut contract = CwIbcCoreContext::new();

    contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let message = get_dummy_raw_msg_conn_open_try(10, 10);

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3],
    }
    .try_into()
    .unwrap();
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(
            &mut deps.storage,
            &ClientId::from_str(&message.client_id).unwrap(),
            light_client,
        )
        .unwrap();

    let cl = client_state.to_any().encode_to_vec();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &IbcClientId::from_str(&message.client_id).unwrap(),
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::from_str(&message.client_id).unwrap(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    mock_lightclient_reply(&mut deps);
    let response = contract
        .execute(
            deps.as_mut(),
            env,
            info,
            CoreExecuteMsg::ConnectionOpenTry {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "execute_connection_open_try");
}

#[test]
#[should_panic(expected = "IbcDecodeError")]
fn fails_on_invalid_raw_bytes_connection_open_init() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = get_mock_env();
    let mut contract = CwIbcCoreContext::default();
    let exec_message = CoreExecuteMsg::ConnectionOpenInit {
        msg: HexString::from_bytes("invalid_message".as_bytes()),
    };
    contract
        .execute(deps.as_mut(), env, info, exec_message)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcDecodeError")]
fn fails_on_invalid_raw_bytes_connection_open_try() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = get_mock_env();
    let mut contract = CwIbcCoreContext::default();
    let exec_message = CoreExecuteMsg::ChannelOpenTry {
        msg: HexString::from_bytes("invalid_message".as_bytes()),
    };
    contract
        .execute(deps.as_mut(), env, info, exec_message)
        .unwrap();
}

#[test]
fn test_for_connection_open_ack() {
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("alice", "umlg", 3000);
    let mut contract = CwIbcCoreContext::new();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();
    assert_eq!(response.attributes[0].value, "instantiate");

    let message = get_dummy_raw_msg_conn_open_ack(10, 10);

    let connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let client_id = IbcClientId::default();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &client_id.clone(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id,
        None,
        counterparty_prefix,
    );

    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Init,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    contract
        .store_connection(&mut deps.storage, &connection_id, &conn_end)
        .unwrap();
    let client_state_any=client_state.to_any();
    let client_state_bytes = client_state_any.encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            client_state_bytes,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let _conn_id = ConnectionId::new(0);
    let _conn_id_on_b = ConnectionId::new(1);

    let response = contract
        .execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::ConnectionOpenAck {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "execute_connection_open_ack");
}

#[test]
fn test_for_connection_open_confirm() {
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("alice", "umlg", 3000);
    let mut contract = CwIbcCoreContext::new();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();
    assert_eq!(response.attributes[0].value, "instantiate");

    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_confirm();
    let connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id,
        connection_id.clone().into(),
        counterparty_prefix,
    );

    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::TryOpen,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let _conn_id = ConnectionId::new(1);
    contract
        .store_connection(&mut deps.storage, &connection_id, &conn_end)
        .unwrap();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            light_client,
        )
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let cl = client_state.to_any().encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &conn_end.client_id().clone(),
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let response = contract
        .execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::ConnectionOpenConfirm {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();

    assert_eq!(
        response.attributes[0].value,
        "execute_connection_open_confirm"
    );
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
fn test_for_connection_open_try_fails() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = get_mock_env();
    let message = get_dummy_raw_msg_conn_open_try(10, 10);
    let client_id = to_ibc_client_id(&message.client_id).unwrap();
    let peoof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let mut contract = CwIbcCoreContext::new();
    let client_state: ClientState = get_dummy_client_state();
    let _counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let _counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3],
    }
    .try_into()
    .unwrap();
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &client_id.clone(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);
    let cl = client_state.to_any().encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &client_id,
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &client_id,
            peoof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info,
            ExecuteMsg::ConnectionOpenTry {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "connection_open_try");

    let events = Event::new("open_try");

    let reply_msg = Reply {
        id: EXECUTE_CONNECTION_OPENTRY,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![events],
            data: None,
        }),
    };
    contract.reply(deps.as_mut(), env, reply_msg).unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_connection_open_confirm_fails() {
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("alice", "umlg", 3000);
    let mut contract = CwIbcCoreContext::new();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();
    assert_eq!(response.attributes[0].value, "instantiate");

    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let message = get_dummy_raw_msg_conn_open_confirm();
    let connection_id = to_ibc_connection_id(&message.connection_id).unwrap();
    let proof_height = to_ibc_height(message.proof_height.clone()).unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3],
    }
    .try_into()
    .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let counterparty_prefix =
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id,
        connection_id.into(),
        counterparty_prefix,
    );

    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::TryOpen,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(1);
    contract
        .store_connection(&mut deps.storage, &conn_id, &conn_end)
        .unwrap();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            light_client,
        )
        .unwrap();

    let cl = client_state.encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &get_mock_env(),
            &conn_end.client_id().clone(),
            cl,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    contract
        .execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::ConnectionOpenConfirm {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();
}
