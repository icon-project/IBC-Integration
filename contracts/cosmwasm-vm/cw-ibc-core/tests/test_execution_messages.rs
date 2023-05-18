pub mod setup;

use std::str::FromStr;
use std::time::Duration;

use common::icon::icon::types::v1::BtpHeader as RawBtpHeader;
use common::icon::icon::types::v1::MerkleNode as RawMerkleNode;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use cosmwasm_std::ContractResult;
use cosmwasm_std::SystemResult;
use cosmwasm_std::WasmQuery;
use cosmwasm_std::{testing::mock_env, to_binary, to_vec, Addr, Event, Reply, SubMsgResponse};
use cw_common::client_response::OpenAckResponse;
use cw_common::client_response::OpenConfirmResponse;
use cw_common::client_response::OpenTryResponse;
use cw_common::client_response::{CreateClientResponse, UpdateClientResponse};

use common::icon::icon::lightclient::v1::ClientState;
use cw_common::consensus_state::ConsensusState;
use cw_common::core_msg::ExecuteMsg;
use cw_common::hex_string::HexString;
use cw_common::ibc_types::IbcClientId;
use cw_common::raw_types::connection::RawMsgConnectionOpenInit;
use cw_common::raw_types::RawVersion;
use cw_common::types::ClientId;
use cw_common::types::ConnectionId;
use cw_common::ProstMessage;

use cw_ibc_core::ConnectionEnd;
use cw_ibc_core::Height;

use cw_ibc_core::{context::CwIbcCoreContext, msg::InstantiateMsg};

use common::icon::icon::lightclient::v1::ClientState as RawClientState;
use common::icon::icon::lightclient::v1::ConsensusState as RawConsensusState;
use cw_common::core_msg::ExecuteMsg as CoreExecuteMsg;

use setup::*;

fn test_for_create_client_execution_message() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();

    let client_state: RawClientState = RawClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: RawConsensusState = RawConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
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

    let create_client_message = CoreExecuteMsg::CreateClient {
        client_state: HexString::from_bytes(&client_state.clone().encode_to_vec()),
        consensus_state: HexString::from_bytes(&consenus_state.clone().encode_to_vec()),
        signer: HexString::from_bytes("raw_message".as_bytes()),
    };

    let response = contract
        .execute(deps.as_mut(), env.clone(), info, create_client_message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "create_client");

    let mock_reponse_data = CreateClientResponse::new(
        "iconclient".to_string(),
        "10-15".to_string(),
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
    let response = contract.reply(deps.as_mut(), env, reply_message).unwrap();

    assert_eq!(response.attributes[0].value, "execute_create_client_reply")
}

#[test]
fn test_for_update_client_execution_messages() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();
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

    contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    let mock_reponse_data = CreateClientResponse::new(
        "iconclient".to_string(),
        "10-15".to_string(),
        client_state.encode_to_vec(),
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
    }
    .try_into()
    .unwrap();

    let message = CoreExecuteMsg::UpdateClient {
        client_id: "iconclient-0".to_string(),
        header: HexString::from_bytes(&signed_header.encode_to_vec()),
        signer: HexString::from_bytes("signeraddress".to_string().as_bytes()),
    };

    let response = contract
        .execute(deps.as_mut(), env.clone(), info, message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "update_client");

    let mock_reponse_data = UpdateClientResponse::new(
        "10-15".to_string(),
        "iconclient-0".to_string(),
        client_state.encode_to_vec(),
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

    let response = contract.reply(deps.as_mut(), env, reply_message).unwrap();

    assert_eq!(response.attributes[0].value, "execute_update_client_reply")
}

#[test]
fn test_for_connection_open_try() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = mock_env();
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

    contract
        .store_client_implementations(
            deps.as_mut().storage,
            ClientId::from_str("iconclient-0").unwrap(),
            "lightclientaddress".to_string(),
        )
        .unwrap();

    let client_state = client_state.encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &IbcClientId::from_str(&message.client_id).unwrap(),
            client_state.clone(),
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
        .execute(deps.as_mut(), env.clone(), info.clone(), exec_message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "connection_open_init");

    let message = get_dummy_raw_msg_conn_open_try(10, 10);

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            ClientId::from_str(&message.client_id.clone()).unwrap(),
            light_client.to_string(),
        )
        .unwrap();

    let cl = to_vec(&client_state.clone());

    contract
        .store_client_state(
            &mut deps.storage,
            &IbcClientId::from_str(&message.client_id.clone()).unwrap(),
            cl.unwrap(),
        )
        .unwrap();

    let consenus_state = to_vec(&consenus_state).unwrap();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::from_str(&message.client_id.clone()).unwrap(),
            Height::new(
                message.proof_height.clone().unwrap().revision_number,
                message.proof_height.clone().unwrap().revision_height,
            )
            .unwrap(),
            consenus_state,
        )
        .unwrap();

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            CoreExecuteMsg::ConnectionOpenTry {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "connection_open_try");

    let conn_id = ConnectionId::new(1);
    let client_id = ClientId::from_str("iconclient-1").unwrap();
    let versions = RawVersion {
        identifier: "identifier".to_string(),
        features: vec!["hello".to_string()],
    };
    let counterparty_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".as_bytes().to_vec(),
    )
    .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let mock_response_data = OpenTryResponse::new(
        conn_id.as_str().to_owned(),
        client_id.ibc_client_id().to_string(),
        counterparty_client_id.ibc_client_id().to_string(),
        "".to_string(),
        counterparty_prefix.as_bytes().to_vec(),
        to_vec(&versions).unwrap(),
        23,
    );
    let mock_data_binary = to_binary(&mock_response_data).unwrap();
    let events = Event::new("open_try");

    let reply_msg = Reply {
        id: 31,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![events],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract
        .reply(deps.as_mut(), env.clone(), reply_msg)
        .unwrap();
    assert_eq!(response.attributes[0].value, "execute_connection_open_try");

    let conn_id = ConnectionId::new(1);
    let client_id = ClientId::from_str("iconclient-1").unwrap();
    let versions = RawVersion {
        identifier: "identifier".to_string(),
        features: vec!["hello".to_string()],
    };
    let mock_response_data = OpenTryResponse::new(
        conn_id.as_str().to_owned(),
        client_id.ibc_client_id().to_string(),
        counterparty_client_id.ibc_client_id().to_string(),
        "".to_string(),
        counterparty_prefix.as_bytes().to_vec(),
        to_vec(&versions).unwrap(),
        23,
    );
    let mock_data_binary = to_binary(&mock_response_data).unwrap();
    let events = Event::new("open_try");

    let reply_msg = Reply {
        id: 31,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![events],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_msg).unwrap();
    assert_eq!(response.attributes[0].value, "execute_connection_open_try");
}

#[test]
#[should_panic(expected = "IbcDecodeError")]
fn fails_on_invalid_raw_bytes_connection_open_init() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = mock_env();
    let mut contract = CwIbcCoreContext::default();
    let exec_message = CoreExecuteMsg::ConnectionOpenInit {
        msg: HexString::from_bytes("invalid_message".as_bytes()),
    };
    contract
        .execute(deps.as_mut(), env.clone(), info.clone(), exec_message)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcDecodeError")]
fn fails_on_invalid_raw_bytes_connection_open_try() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = mock_env();
    let mut contract = CwIbcCoreContext::default();
    let exec_message = CoreExecuteMsg::ChannelOpenTry {
        msg: HexString::from_bytes("invalid_message".as_bytes()),
    };
    contract
        .execute(deps.as_mut(), env.clone(), info.clone(), exec_message)
        .unwrap();
}

#[test]
fn test_for_connection_open_ack() {
    let mut deps = deps();
    let env = mock_env();
    let info = create_mock_info("alice", "umlg", 3000);
    let mut contract = CwIbcCoreContext::new();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();
    assert_eq!(response.attributes[0].value, "instantiate");

    let message = get_dummy_raw_msg_conn_open_ack(10, 10);

    let res_msg = ibc::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck::try_from(
        message.clone(),
    )
    .unwrap();

    let client_id = IbcClientId::default();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
    }
    .try_into()
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

    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            client_id.clone().into(),
            light_client.to_string(),
        )
        .unwrap();

    let counterparty_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".as_bytes().to_vec(),
    )
    .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id.ibc_client_id().clone(),
        None,
        counterparty_prefix.clone(),
    );

    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Init,
        IbcClientId::default().clone(),
        counter_party.clone(),
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    contract
        .store_connection(
            &mut deps.storage,
            res_msg.conn_id_on_a.clone().into(),
            conn_end.clone(),
        )
        .unwrap();

    let client_state_bytes = client_state.encode_to_vec();

    contract
        .store_client_state(
            &mut deps.storage,
            &client_id.clone(),
            client_state_bytes.clone(),
        )
        .unwrap();

    let consenus_state = to_vec(&consenus_state).unwrap();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone().into(),
            res_msg.proofs_height_on_b.clone(),
            consenus_state.clone(),
        )
        .unwrap();
    let conn_id = ConnectionId::new(0);
    let _conn_id_on_b = ConnectionId::new(1);

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenAck {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "connection_open_ack");

    let versions = RawVersion {
        identifier: "identifier".to_string(),
        features: vec!["hello".to_string()],
    };
    let mock_response_data = OpenAckResponse {
        conn_id: conn_id.as_str().to_owned(),
        version: to_vec(&versions).unwrap(),
        counterparty_client_id: conn_id.as_str().to_owned(),
        counterparty_connection_id: counter_party.client_id().to_string(),
        counterparty_prefix: to_vec(&counterparty_prefix).unwrap(),
    };
    let mock_data_binary = to_binary(&mock_response_data).unwrap();
    let events = Event::new("open_ack");

    let reply_msg = Reply {
        id: 32,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![events],
            data: Some(mock_data_binary),
        }),
    };

    let response = contract.reply(deps.as_mut(), env, reply_msg).unwrap();

    assert_eq!(response.attributes[0].value, "execute_connection_open_ack");
}

#[test]
fn test_for_connection_open_confirm() {
    let mut deps = deps();
    let env = mock_env();
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

    let res_msg =
        ibc::core::ics03_connection::msgs::conn_open_confirm::MsgConnectionOpenConfirm::try_from(
            message.clone(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
    }
    .try_into()
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

    let counterparty_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".as_bytes().to_vec(),
    )
    .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id.ibc_client_id().clone(),
        res_msg.conn_id_on_b.clone().into(),
        counterparty_prefix.clone(),
    );

    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::TryOpen,
        IbcClientId::default().clone(),
        counter_party.clone(),
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(1);
    contract
        .store_connection(
            &mut deps.storage,
            res_msg.conn_id_on_b.clone().into(),
            conn_end.clone(),
        )
        .unwrap();

    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            conn_end.client_id().clone().into(),
            light_client.to_string(),
        )
        .unwrap();

    let cl = client_state.encode_to_vec();

    contract
        .store_client_state(&mut deps.storage, &conn_end.client_id().clone().into(), cl)
        .unwrap();

    let consenus_state = to_vec(&consenus_state).unwrap();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            res_msg.proof_height_on_a.clone(),
            consenus_state,
        )
        .unwrap();

    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenConfirm {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "connection_open_confirm");

    let mock_response_data = OpenConfirmResponse {
        conn_id: res_msg.conn_id_on_b.to_string().into(),
        counterparty_client_id: conn_id.as_str().to_string().clone(),
        counterparty_connection_id: counter_party.client_id().to_string(),
        counterparty_prefix: to_vec(&counterparty_prefix).unwrap(),
    };

    let mock_data_binary = to_binary(&mock_response_data).unwrap();
    let events = Event::new("open_confirm");

    let reply_msg = Reply {
        id: 33,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![events],
            data: Some(mock_data_binary),
        }),
    };

    let response = contract
        .reply(deps.as_mut(), env.clone(), reply_msg)
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
    let env = mock_env();
    let message = get_dummy_raw_msg_conn_open_try(10, 10);
    let res_msg = ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry::try_from(
        message.clone(),
    )
    .unwrap();
    let mut contract = CwIbcCoreContext::new();
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
    let counterparty_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".as_bytes().to_vec(),
    )
    .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            res_msg.client_id_on_b.clone().into(),
            light_client.to_string(),
        )
        .unwrap();

    let cl = client_state.encode_to_vec();

    contract
        .store_client_state(&mut deps.storage, &res_msg.client_id_on_b.clone(), cl)
        .unwrap();

    let consenus_state = to_vec(&consenus_state).unwrap();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &res_msg.client_id_on_b.clone(),
            res_msg.proofs_height_on_a.clone(),
            consenus_state,
        )
        .unwrap();

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenTry {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "connection_open_try");

    let conn_id = ConnectionId::new(1);
    let client_id = ClientId::from_str("iconclient-1").unwrap();
    let versions = RawVersion {
        identifier: "identifier".to_string(),
        features: vec!["hello".to_string()],
    };
    let mock_response_data = OpenTryResponse::new(
        conn_id.as_str().to_owned(),
        client_id.ibc_client_id().to_string(),
        counterparty_client_id.ibc_client_id().to_string(),
        "".to_string(),
        counterparty_prefix.as_bytes().to_vec(),
        to_vec(&versions).unwrap(),
        23,
    );
    let mock_data_binary = to_binary(&mock_response_data).unwrap();
    let events = Event::new("open_try");

    let reply_msg = Reply {
        id: 31,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![events],
            data: Some(mock_data_binary),
        }),
    };
    contract.reply(deps.as_mut(), env, reply_msg).unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_connection_open_confirm_fails() {
    let mut deps = deps();
    let env = mock_env();
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

    let res_msg =
        ibc::core::ics03_connection::msgs::conn_open_confirm::MsgConnectionOpenConfirm::try_from(
            message.clone(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
    }
    .try_into()
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

    let counterparty_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".as_bytes().to_vec(),
    )
    .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id.ibc_client_id().clone(),
        res_msg.conn_id_on_b.clone().into(),
        counterparty_prefix.clone(),
    );

    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::TryOpen,
        IbcClientId::default().clone(),
        counter_party.clone(),
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(1);
    contract
        .store_connection(&mut deps.storage, conn_id.clone().into(), conn_end.clone())
        .unwrap();

    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            conn_end.client_id().clone().into(),
            light_client.to_string(),
        )
        .unwrap();

    let cl = client_state.encode_to_vec();

    contract
        .store_client_state(&mut deps.storage, &conn_end.client_id().clone().into(), cl)
        .unwrap();

    let consenus_state = to_vec(&consenus_state).unwrap();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &conn_end.client_id().clone(),
            res_msg.proof_height_on_a.clone(),
            consenus_state,
        )
        .unwrap();

    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

    contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenConfirm {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "unknown field")]
fn test_connection_open_try_fails_invalid_id() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 4000);
    let env = mock_env();
    let mut contract = CwIbcCoreContext::new();
    let message = RawMsgConnectionOpenInit {
        client_id: "client_id_on_a".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };

    let res_msg =
        ibc::core::ics03_connection::msgs::conn_open_init::MsgConnectionOpenInit::try_from(
            message.clone(),
        )
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
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();
    assert_eq!(response.attributes[0].value, "instantiate");

    let counterparty_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".as_bytes().to_vec(),
    )
    .unwrap();
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            res_msg.client_id_on_a.clone().into(),
            light_client.to_string(),
        )
        .unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id.ibc_client_id().clone(),
        None,
        counterparty_prefix.clone(),
    );

    let cl = client_state.encode_to_vec();
    contract
        .store_client_state(&mut deps.storage, &res_msg.client_id_on_a.clone(), cl)
        .unwrap();
    contract
        .client_state(&mut deps.storage, &res_msg.client_id_on_a)
        .unwrap();
    contract
        .connection_next_sequence_init(&mut deps.storage, u64::default())
        .unwrap();

    deps.querier.update_wasm(|r| match r {
        WasmQuery::Smart { contract_addr, msg } => {
            SystemResult::Ok(ContractResult::Ok(to_binary(&vec![0, 2, 3]).unwrap()))
        }
        _ => todo!(),
    });

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenInit {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "connection_open_init");

    let message = get_dummy_raw_msg_conn_open_try(10, 10);
    let res_msg = ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry::try_from(
        message.clone(),
    )
    .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "helloconnectionmessage".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    let cl = client_state.encode_to_vec();

    contract
        .store_client_state(&mut deps.storage, &res_msg.client_id_on_b.clone(), cl)
        .unwrap();

    let consenus_state = to_vec(&consenus_state).unwrap();

    contract
        .store_consensus_state(
            &mut deps.storage,
            &res_msg.client_id_on_b.clone(),
            res_msg.proofs_height_on_a.clone(),
            consenus_state,
        )
        .unwrap();
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            res_msg.client_id_on_b.clone().into(),
            light_client.to_string(),
        )
        .unwrap();

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenTry {
                msg: HexString::from_bytes(&message.encode_to_vec()),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "connection_open_try");

    let conn_id = ConnectionId::new(1);
    let client_id = ClientId::from_str("iconclient-1").unwrap();
    let versions = RawVersion {
        identifier: "identifier".to_string(),
        features: vec!["hello".to_string()],
    };
    let mock_response_data = OpenTryResponse::new(
        conn_id.as_str().to_owned(),
        client_id.ibc_client_id().to_string(),
        counterparty_client_id.ibc_client_id().to_string(),
        "".to_string(),
        counterparty_prefix.as_bytes().to_vec(),
        to_vec(&versions).unwrap(),
        23,
    );
    let mock_data_binary = to_binary(&mock_response_data).unwrap();
    let events = Event::new("open_try");

    let reply_msg = Reply {
        id: 33,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![events],
            data: Some(mock_data_binary),
        }),
    };
    contract.reply(deps.as_mut(), env, reply_msg).unwrap();
}
