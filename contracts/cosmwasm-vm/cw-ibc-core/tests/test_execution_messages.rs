pub mod setup;

use std::str::FromStr;

use common::icon::icon::types::v1::BtpHeader as RawBtpHeader;
use common::icon::icon::types::v1::MerkleNode as RawMerkleNode;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use cosmwasm_std::ContractResult;
use cosmwasm_std::SystemResult;
use cosmwasm_std::WasmQuery;
use cosmwasm_std::{testing::mock_env, to_binary, to_vec, Addr, Event, Reply, SubMsgResponse};
use cw_common::client_response::OpenTryResponse;
use cw_common::client_response::{CreateClientResponse, UpdateClientResponse};
use cw_common::IbcClientId;
use cw_common::IbcConnectionId;
use cw_common::ProstMessage;

use cw_common::types::ClientId;
use cw_common::types::ConnectionId;
use cw_ibc_core::ics02_client::types::SignedHeader;
use cw_ibc_core::Height;

use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics02_client::types::{ClientState, ConsensusState},
    msg::InstantiateMsg,
};

use cw_common::core_msg::ExecuteMsg as CoreExecuteMsg;

use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit as RawMsgConnectionOpenInit;
use setup::*;

#[test]
fn test_for_create_client_execution_message() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();

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
        client_state: client_state.clone().try_into().unwrap(),
        consensus_state: consenus_state.clone().try_into().unwrap(),
        signer: "raw_message".as_bytes().into(),
    };

    let response = contract
        .execute(deps.as_mut(), env.clone(), info, create_client_message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "create_client");

    let mock_reponse_data = CreateClientResponse::new(
        "iconclient".to_string(),
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

    let signed_header: SignedHeader = RawSignedHeader {
        header: Some(btp_header),
        signatures: vec![hex::decode("6c8b2bc2c3d31e34bd4ed9db6eff7d5dc647b13c58ae77d54e0b05141cb7a7995102587f1fa33fd56815463c6b78e100217c29ddca20fcace80510e3dab03a1600").unwrap()],
    }
    .try_into()
    .unwrap();

    let message = CoreExecuteMsg::UpdateClient {
        client_id: "iconclient-0".to_string(),
        header: signed_header.try_into().unwrap(),
        signer: "signeraddress".to_string().as_bytes().into(),
    };

    let response = contract
        .execute(deps.as_mut(), env.clone(), info, message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "update_client");

    let mock_reponse_data = UpdateClientResponse::new(
        "10-15".to_string(),
        "iconclient-0".to_string(),
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

    let client_state = to_vec(&client_state).unwrap();
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
        msg: message.encode_to_vec(),
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
                msg: message.encode_to_vec(),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "connection_open_try");

    let conn_id = ConnectionId::new(1);
    let client_id = ClientId::from_str("iconclient-1").unwrap();
    let versions = ibc_proto::ibc::core::connection::v1::Version {
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
    let versions = ibc_proto::ibc::core::connection::v1::Version {
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
        msg: "invalid_message".as_bytes().to_vec(),
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
        msg: "invalid_message".as_bytes().to_vec(),
    };
    contract
        .execute(deps.as_mut(), env.clone(), info.clone(), exec_message)
        .unwrap();
}
