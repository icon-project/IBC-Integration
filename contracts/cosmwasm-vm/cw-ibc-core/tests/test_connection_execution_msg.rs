pub mod setup;
use std::{str::FromStr, time::Duration};

use cosmwasm_std::{testing::mock_env, to_binary, to_vec, Addr, Event, Reply, SubMsgResponse};
use cw_common::{client_response::OpenConfirmResponse, types::ConnectionId};
use cw_common::{
    client_response::{OpenAckResponse, OpenTryResponse},
    types::ClientId,
    IbcClientId,
};
use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics02_client::types::{ClientState, ConsensusState},
    ics03_connection::Version,
    msg::{ExecuteMsg, InstantiateMsg},
    ConnectionEnd,
};
use ibc::core::ics02_client::height::Height;

use ibc_proto::{
    google::protobuf::Any,
    ibc::core::connection::v1::{
        MsgConnectionOpenAck, MsgConnectionOpenInit as RawMsgConnectionOpenInit,
    },
};
use setup::*;

#[test]
fn test_for_connection_open_try() {
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
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id.ibc_client_id().clone(),
        None,
        counterparty_prefix.clone(),
    );

    let cl = to_vec(&client_state);
    contract
        .store_client_state(
            &mut deps.storage,
            &res_msg.client_id_on_a.clone(),
            cl.unwrap(),
        )
        .unwrap();
    contract
        .client_state(&mut deps.storage, &res_msg.client_id_on_a)
        .unwrap();
    contract
        .connection_next_sequence_init(&mut deps.storage, u64::default())
        .unwrap();

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenInit {
                client_id_on_a: "client_id_on_a".to_string(),
                counterparty: counter_party,
                version: Some(Version::default()),
                delay_period: Duration::new(1, 1),
                signer: "raw".parse().unwrap(),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "connection_open_init");

    let message = get_dummy_raw_msg_conn_open_try(10, 10);
    let mut res_msg =
        ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry::try_from(
            message.clone(),
        )
        .unwrap();
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

    let cl = to_vec(&client_state);

    contract
        .store_client_state(
            &mut deps.storage,
            &res_msg.client_id_on_b.clone(),
            cl.unwrap(),
        )
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
                msg: res_msg.into(),
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

    let client_state_bytes = to_vec(&client_state).unwrap();

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
    let conn_id_on_b = ConnectionId::new(1);

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenAck {
                conn_id_on_a: conn_id.clone(),
                conn_id_on_b: conn_id_on_b.clone(),
                client_state_of_a_on_b: client_state.into(),
                proof_conn_end_on_b: to_vec(&conn_end).unwrap(),
                proof_client_state_of_a_on_b: client_state_bytes.clone(),
                proof_consensus_state_of_a_on_b: to_vec(&consenus_state).unwrap(),
                proofs_height_on_b: res_msg.proofs_height_on_b.clone(),
                consensus_height_of_a_on_b: res_msg.consensus_height_of_a_on_b,
                version: Version::default(),
                signer: "raw".parse().unwrap(),
            },
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "connection_open_ack");

    let versions = ibc_proto::ibc::core::connection::v1::Version {
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

    let cl = to_vec(&client_state);

    contract
        .store_client_state(
            &mut deps.storage,
            &conn_end.client_id().clone().into(),
            cl.unwrap(),
        )
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
                conn_id_on_b: conn_id.clone(),
                proof_conn_end_on_a: to_vec(&conn_end).unwrap(),
                proof_height_on_a: res_msg.proof_height_on_a.clone(),
                signer: "raw".parse().unwrap(),
            },
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "connection_open_confirm");

    let mock_response_data = OpenConfirmResponse {
        conn_id: conn_id.connection_id().as_str().to_owned().clone(),
        counterparty_client_id: conn_id.as_str().to_owned().clone(),
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
        "execute_connection_openconfirm"
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

    let cl = to_vec(&client_state);

    contract
        .store_client_state(
            &mut deps.storage,
            &res_msg.client_id_on_b.clone(),
            cl.unwrap(),
        )
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
                msg: res_msg.into(),
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
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id.ibc_client_id().clone(),
        None,
        counterparty_prefix.clone(),
    );

    let cl = to_vec(&client_state);
    contract
        .store_client_state(
            &mut deps.storage,
            &res_msg.client_id_on_a.clone(),
            cl.unwrap(),
        )
        .unwrap();
    contract
        .client_state(&mut deps.storage, &res_msg.client_id_on_a)
        .unwrap();
    contract
        .connection_next_sequence_init(&mut deps.storage, u64::default())
        .unwrap();

    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenInit {
                client_id_on_a: "client_id_on_a".to_string(),
                counterparty: counter_party,
                version: Some(Version::default()),
                delay_period: Duration::new(1, 1),
                signer: "raw".parse().unwrap(),
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
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            res_msg.client_id_on_b.clone().into(),
            light_client.to_string(),
        )
        .unwrap();

    let cl = to_vec(&client_state);

    contract
        .store_client_state(
            &mut deps.storage,
            &res_msg.client_id_on_b.clone(),
            cl.unwrap(),
        )
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
                msg: res_msg.into(),
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

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_connection_open_ack_fails_of_conn_id() {
    let mut deps = deps();
    let env = mock_env();
    let info = create_mock_info("alice", "umlg", 3000);
    let mut contract = CwIbcCoreContext::new();

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

    let client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            client_id.clone().into(),
            client.to_string(),
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

    let client_state_bytes = to_vec(&client_state).unwrap();

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
    let conn_id = ConnectionId::new(1);

    contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::ConnectionOpenAck {
                conn_id_on_a: conn_id.clone(),
                conn_id_on_b: conn_id.clone(),
                client_state_of_a_on_b: client_state.into(),
                proof_conn_end_on_b: to_vec(&conn_end).unwrap(),
                proof_client_state_of_a_on_b: client_state_bytes.clone(),
                proof_consensus_state_of_a_on_b: to_vec(&consenus_state).unwrap(),
                proofs_height_on_b: res_msg.proofs_height_on_b.clone(),
                consensus_height_of_a_on_b: res_msg.consensus_height_of_a_on_b,
                version: Version::default(),
                signer: "raw".parse().unwrap(),
            },
        )
        .unwrap();
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

    let cl = to_vec(&client_state);

    contract
        .store_client_state(
            &mut deps.storage,
            &conn_end.client_id().clone().into(),
            cl.unwrap(),
        )
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
                conn_id_on_b: conn_id.clone(),
                proof_conn_end_on_a: to_vec(&conn_end).unwrap(),
                proof_height_on_a: Height::new(10, 10).unwrap(),
                signer: "raw".parse().unwrap(),
            },
        )
        .unwrap();
}
