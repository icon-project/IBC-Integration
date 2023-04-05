use std::str::FromStr;
use std::time::Duration;

pub mod setup;
use common::icon::icon::lightclient::v1::ConsensusState;
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::to_binary;
use cosmwasm_std::to_vec;
use cosmwasm_std::Addr;
use cosmwasm_std::Event;
use cosmwasm_std::Reply;
use cosmwasm_std::SubMsgResponse;
use cosmwasm_std::SubMsgResult;
use cw_ibc_core::context::CwIbcCoreContext;
use cw_ibc_core::ics02_client::types::ClientState;
use cw_ibc_core::ics03_connection::conn_types::VerifyConnectionState;
use cw_ibc_core::ics03_connection::event::create_open_ack_event;
use cw_ibc_core::ics03_connection::event::create_open_confirm_event;
use cw_ibc_core::ics03_connection::event::create_open_init_event;
use cw_ibc_core::ics03_connection::event::create_open_try_event;
use cw_ibc_core::ics03_connection::handler::OpenTryResponse;
use cw_ibc_core::ics03_connection::handler::EXECUTE_CONNECTION_OPENTRY;
use cw_ibc_core::traits::IbcClient;
use cw_ibc_core::types::ClientId;
use cw_ibc_core::types::ClientType;
use cw_ibc_core::types::ConnectionId;
use cw_ibc_core::ConnectionEnd;
use cw_ibc_core::IbcClientId;
use ibc::core::ics03_connection::connection::Counterparty;
use ibc::core::ics03_connection::connection::State;
use ibc::core::ics03_connection::events::CLIENT_ID_ATTRIBUTE_KEY;
use ibc::core::ics03_connection::events::CONN_ID_ATTRIBUTE_KEY;
use ibc::core::ics03_connection::events::COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY;
use ibc::core::ics03_connection::events::COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY;
use ibc::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck;
use ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry;
use ibc::core::ics03_connection::version::get_compatible_versions;
use ibc::core::ics03_connection::version::Version;
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc::core::ics24_host::path::ConnectionPath;
use ibc::events::IbcEventType;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height;
use ibc_proto::ibc::core::connection::v1::Counterparty as RawCounterparty;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenAck as RawMsgConnectionOpenAck;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenConfirm as RawMsgConnectionOpenConfirm;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenConfirm;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit as RawMsgConnectionOpenInit;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry as RawMsgConnectionOpenTry;
use setup::*;

#[test]
fn test_set_connection() {
    let mut deps = deps();
    let conn_end = ConnectionEnd::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();
    let result = contract
        .connection_end(deps.as_ref().storage, conn_id)
        .unwrap();

    assert_eq!(conn_end, result)
}

#[test]
fn test_get_connection() {
    let mut deps = deps();
    let ss = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = Counterparty::new(IbcClientId::default(), None, ss.unwrap());
    let conn_end = ConnectionEnd::new(
        State::Open,
        IbcClientId::default(),
        counter_party,
        vec![Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();
    let result = contract
        .connection_end(deps.as_ref().storage, conn_id)
        .unwrap();

    assert_eq!(conn_end, result)
}

#[test]
fn test_connection_sequence() {
    let mut store = deps();
    let contract = CwIbcCoreContext::new();
    contract
        .connection_next_sequence_init(store.as_mut().storage, u64::default())
        .unwrap();
    let result = contract.connection_counter(store.as_ref().storage).unwrap();

    assert_eq!(0, result);

    let increment_sequence = contract
        .increase_connection_counter(store.as_mut().storage)
        .unwrap();
    assert_eq!(1, increment_sequence);
}

#[test]
fn test_client_connection() {
    let mut deps = deps();
    let client_id = ClientId::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();

    contract
        .store_connection_to_client(deps.as_mut().storage, client_id.clone(), conn_id.clone())
        .unwrap();

    let result = contract
        .client_connection(deps.as_ref().storage, client_id)
        .unwrap();

    assert_eq!(conn_id, result)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_get_connection_fail() {
    let deps = deps();

    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();

    contract
        .connection_end(deps.as_ref().storage, conn_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_set_connection_fail() {
    let deps = deps();
    let conn_id = ConnectionId::new(0);
    let contract = CwIbcCoreContext::new();
    contract
        .connection_end(deps.as_ref().storage, conn_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
fn test_connection_sequence_fail() {
    let store = deps();
    let contract = CwIbcCoreContext::new();
    contract.connection_counter(store.as_ref().storage).unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"cw_ibc_core::types::ConnectionId\" })")]
fn test_client_connection_fail() {
    let deps = deps();
    let client_id = ClientId::default();

    let contract = CwIbcCoreContext::new();

    contract
        .client_connection(deps.as_ref().storage, client_id)
        .unwrap();
}

#[test]
pub fn test_to_and_from_connection_open_init() {
    let raw = get_dummy_raw_msg_conn_open_init();
    let msg = MsgConnectionOpenInit::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgConnectionOpenInit::from(msg.clone());
    let msg_back = MsgConnectionOpenInit::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}
#[test]
fn test_to_and_from_connection_open_try() {
    let raw = get_dummy_raw_msg_conn_open_try(10, 34);
    let msg = MsgConnectionOpenTry::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgConnectionOpenTry::from(msg.clone());
    let msg_back = MsgConnectionOpenTry::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}

#[test]
fn test_to_and_from_connection_open_ack() {
    let raw = get_dummy_raw_msg_conn_open_ack(10, 34);
    let msg = MsgConnectionOpenAck::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgConnectionOpenAck::from(msg.clone());
    let msg_back = MsgConnectionOpenAck::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}

#[test]
fn test_to_and_from_connection_open_confirm() {
    let raw = get_dummy_raw_msg_conn_open_confirm();
    let msg = MsgConnectionOpenConfirm::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgConnectionOpenConfirm::from(msg.clone());
    let msg_back = MsgConnectionOpenConfirm::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}

#[test]
fn connection_open_init_from_raw_valid_parameter() {
    let default_raw_init_msg = get_dummy_raw_msg_conn_open_init();
    let res_msg = MsgConnectionOpenInit::try_from(default_raw_init_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}

#[test]
fn connection_invalid_client_id_parameter() {
    let default_raw_init_msg = RawMsgConnectionOpenInit {
        client_id: "client".to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: None,
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    };
    let res_msg = MsgConnectionOpenInit::try_from(default_raw_init_msg.clone());
    assert_eq!(res_msg.is_err(), false)
}

#[test]
fn connection_open_init_invalid_destination_connection_id() {
    let default_raw_init_msg = get_dummy_raw_msg_conn_open_init;
    let default_raw_init_msg = RawMsgConnectionOpenInit {
        counterparty: Some(RawCounterparty {
            connection_id: "abcdefghijksdffjssdkflweldflsfladfsfwjkrekcmmsdfsdfjflddmnopqrstu"
                .to_string(),
            ..get_dummy_raw_counterparty(None)
        }),
        ..default_raw_init_msg()
    };
    let res_msg = MsgConnectionOpenInit::try_from(default_raw_init_msg.clone());
    assert_eq!(res_msg.is_err(), false)
}

#[test]
fn connection_open_try_from_raw_valid_parameter() {
    let default_raw_try_msg = get_dummy_raw_msg_conn_open_try(1, 3);
    let res_msg = MsgConnectionOpenTry::try_from(default_raw_try_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}

#[test]
fn connection_open_try_destination_client_id_with_lower_case_and_special_characters() {
    let default_raw_try_msg = get_dummy_raw_msg_conn_open_try(1, 3);
    let try_msg = RawMsgConnectionOpenTry {
        counterparty: Some(RawCounterparty {
            client_id: "ClientId_".to_string(),
            ..get_dummy_raw_counterparty(Some(0))
        }),
        ..default_raw_try_msg.clone()
    };
    let res_msg = MsgConnectionOpenTry::try_from(try_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}

#[test]
fn connection_open_try_invalid_client_id_name_too_short() {
    let default_raw_try_msg = get_dummy_raw_msg_conn_open_try(1, 3);
    let try_msg = RawMsgConnectionOpenTry {
        client_id: "client".to_string(),
        ..default_raw_try_msg.clone()
    };
    let res_msg = MsgConnectionOpenTry::try_from(try_msg.clone());
    assert_eq!(res_msg.is_ok(), false)
}

#[test]
fn test_commitment_prefix() {
    let contract = CwIbcCoreContext::new();
    let expected = CommitmentPrefix::try_from(b"Ibc".to_vec()).unwrap_or_default();
    let result = contract.commitment_prefix();
    assert_eq!(result, expected);
}
#[test]
fn connection_open_ack_from_raw_valid_parameter() {
    let default_raw_ack_msg = get_dummy_raw_msg_conn_open_ack(5, 5);
    let res_msg = MsgConnectionOpenAck::try_from(default_raw_ack_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}

#[test]
fn connection_open_ack_invalid_connection_id() {
    let default_raw_ack_msg = get_dummy_raw_msg_conn_open_ack(5, 5);
    let ack_msg = RawMsgConnectionOpenAck {
        connection_id: "con007".to_string(),
        ..default_raw_ack_msg.clone()
    };
    let res_msg = MsgConnectionOpenAck::try_from(ack_msg.clone());
    assert_eq!(res_msg.is_ok(), false)
}

#[test]
fn connection_open_ack_invalid_version() {
    let default_raw_ack_msg = get_dummy_raw_msg_conn_open_ack(5, 5);
    let ack_msg = RawMsgConnectionOpenAck {
        version: None,
        ..default_raw_ack_msg.clone()
    };
    let res_msg = MsgConnectionOpenAck::try_from(ack_msg.clone());
    assert_eq!(res_msg.is_ok(), false)
}

#[test]
fn connection_open_ack_invalid_proof_height() {
    let default_raw_ack_msg = get_dummy_raw_msg_conn_open_ack(5, 5);
    let ack_msg = RawMsgConnectionOpenAck {
        proof_height: Some(Height {
            revision_number: 1,
            revision_height: 0,
        }),
        ..default_raw_ack_msg.clone()
    };
    let res_msg = MsgConnectionOpenAck::try_from(ack_msg.clone());
    assert_eq!(res_msg.is_ok(), false)
}

#[test]
fn connection_open_ack_invalid_consensus_height_and_height_is_0() {
    let default_raw_ack_msg = get_dummy_raw_msg_conn_open_ack(5, 5);
    let ack_msg = RawMsgConnectionOpenAck {
        consensus_height: Some(Height {
            revision_number: 1,
            revision_height: 0,
        }),
        ..default_raw_ack_msg
    };
    let res_msg = MsgConnectionOpenAck::try_from(ack_msg.clone());
    assert_eq!(res_msg.is_ok(), false)
}

#[test]
fn connection_open_confirm_with_valid_parameter() {
    let default_raw_confirm_msg = get_dummy_raw_msg_conn_open_confirm();
    let res_msg = MsgConnectionOpenConfirm::try_from(default_raw_confirm_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}

#[test]
fn connection_open_confirm_invalid_connection_id_non_alpha() {
    let default_raw_confirm_msg = get_dummy_raw_msg_conn_open_confirm();
    let confirm_msg = RawMsgConnectionOpenConfirm {
        connection_id: "con007".to_string(),
        ..default_raw_confirm_msg.clone()
    };
    let res_msg = MsgConnectionOpenConfirm::try_from(confirm_msg.clone());
    assert_eq!(res_msg.is_err(), false)
}

#[test]
fn connection_open_confirm_invalid_proof_height() {
    let default_raw_confirm_msg = get_dummy_raw_msg_conn_open_confirm();
    let confirm_msg = RawMsgConnectionOpenConfirm {
        proof_height: Some(Height {
            revision_number: 1,
            revision_height: 0,
        }),
        ..default_raw_confirm_msg
    };
    let res_msg = MsgConnectionOpenConfirm::try_from(confirm_msg.clone());
    assert_eq!(res_msg.is_err(), false)
}

#[test]
fn connection_open_init() {
    let mut deps = deps();

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

    let contract = CwIbcCoreContext::new();
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

    let res = contract.connection_open_init(deps.as_mut(), res_msg);
    assert_eq!(res.is_ok(), true);
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
fn test_validate_open_init_connection_fail() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
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
    contract
        .connection_open_init(deps.as_mut(), res_msg)
        .unwrap();
}

#[test]
fn create_connection_open_init_event() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let event = create_open_init_event(
        connection_id.as_str(),
        client_id.as_str(),
        counterparty_client_id.as_str(),
    );
    assert_eq!(IbcEventType::OpenInitConnection.as_str(), event.ty);
    assert_eq!("connection-10", event.attributes[0].value);
    assert_eq!("07-tendermint-0", event.attributes[1].value);
    assert_eq!("07-tendermint-0", event.attributes[2].value);
}

#[test]
fn create_connection_open_ack_event() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let counterparty_connection_id = ConnectionId::new(20);
    let event = create_open_ack_event(
        connection_id,
        client_id,
        counterparty_connection_id,
        counterparty_client_id,
    );
    assert_eq!(IbcEventType::OpenAckConnection.as_str(), event.ty);
    assert_eq!("connection-10", event.attributes[0].value);
    assert_eq!("07-tendermint-0", event.attributes[1].value);
    assert_eq!("connection-20", event.attributes[2].value);
}

#[test]
fn create_connection_open_try_event() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let counterparty_connection_id = ConnectionId::new(20);
    let event = create_open_try_event(
        connection_id,
        client_id,
        counterparty_connection_id,
        counterparty_client_id,
    );
    assert_eq!(IbcEventType::OpenTryConnection.as_str(), event.ty);
}

#[test]
fn create_conection_open_confirm_event() {
    let connection_id_on_b = ConnectionId::new(10);
    let client_id_on_b = ClientId::default();
    let counterparty_connection_id_on_a = ConnectionId::new(2);
    let counterparty_client_id_on_a = ClientId::default();
    let event = create_open_confirm_event(
        connection_id_on_b,
        client_id_on_b,
        counterparty_connection_id_on_a,
        counterparty_client_id_on_a,
    );
    assert_eq!(IbcEventType::OpenConfirmConnection.as_str(), event.ty);
    assert_eq!("connection-10", event.attributes[0].value);
}

#[test]
fn test_get_compatible_versions() {
    let versions = get_compatible_versions();
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0], Version::default());
}

#[test]
fn connection_to_verify_correct_connection_id() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let event = create_open_init_event(
        connection_id.as_str(),
        client_id.as_str(),
        counterparty_client_id.as_str(),
    );
    let attribute = event
        .attributes
        .iter()
        .find(|attr| attr.key == CONN_ID_ATTRIBUTE_KEY)
        .expect("Missing attribute");
    assert_eq!(attribute.value, "connection-10");
}

#[test]
fn connection_to_verify_correct_client_id() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let event = create_open_init_event(
        connection_id.as_str(),
        client_id.as_str(),
        counterparty_client_id.as_str(),
    );
    let attribute = event
        .attributes
        .iter()
        .find(|attr| attr.key == CLIENT_ID_ATTRIBUTE_KEY)
        .expect("Missing attribute");
    assert_eq!(attribute.value, "07-tendermint-0");
}

#[test]
fn connection_to_verify_correct_counterparty_client_id() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let event = create_open_init_event(
        connection_id.as_str(),
        client_id.as_str(),
        counterparty_client_id.as_str(),
    );
    let attribute = event
        .attributes
        .iter()
        .find(|attr| attr.key == COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY)
        .expect("Missing attribute");
    assert_eq!(attribute.value, "07-tendermint-0");
}

#[test]
fn connection_to_verify_correct_counterparty_conn_id() {
    let connection_id = ConnectionId::new(10);
    let client_id = ClientId::default();
    let counterparty_client_id = ClientId::default();
    let counterparty_conn_id = ConnectionId::new(1);
    let event = create_open_ack_event(
        connection_id,
        client_id,
        counterparty_conn_id,
        counterparty_client_id,
    );
    let attribute = event
        .attributes
        .iter()
        .find(|attr| attr.key == COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY)
        .expect("Missing attribute");
    assert_eq!(attribute.value, "connection-1");
}
#[test]
fn connection_open_try_execute() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let raw = get_dummy_raw_msg_conn_open_try(10, 10);
    let mut msg = MsgConnectionOpenTry::try_from(raw.clone()).unwrap();
    let _store = contract.init_connection_counter(deps.as_mut().storage, u64::default());

    let counterparty_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".as_bytes().to_vec(),
    )
    .unwrap();
    let client_id = ClientId::from_str("iconclient-1").unwrap();
    let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        counterparty_client_id.ibc_client_id().clone(),
        None,
        counterparty_prefix.clone(),
    );
    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default().clone(),
        counter_party.clone(),
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let versions = ibc_proto::ibc::core::connection::v1::Version {
        identifier: "identifier".to_string(),
        features: vec!["hello".to_string()],
    };
    let conn_id = ConnectionId::new(1);

    let contract = CwIbcCoreContext::new();
    contract
        .store_connection_to_client(deps.as_mut().storage, client_id.clone(), conn_id.clone())
        .unwrap();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end)
        .unwrap();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();

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
    let response = SubMsgResponse {
        data: Some(mock_data_binary),
        events: vec![events],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply_msg = Reply {
        id: EXECUTE_CONNECTION_OPENTRY,
        result,
    };

    let res = contract.excute_connection_open_try(deps.as_mut(), reply_msg);

    assert_eq!(res.is_ok(), true)
}

#[test]
fn connection_open_try_validate() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);
    let env = mock_env();
    let contract = CwIbcCoreContext::default();
    contract
        .connection_next_sequence_init(&mut deps.storage, u128::default().try_into().unwrap())
        .unwrap();
    // let versions = vec![ibc_proto::ibc::core::connection::v1::Version {
    //     identifier: "identifier".to_string(),
    //     features: vec!["hello".to_string()],
    // }];
    // let message = RawMsgConnectionOpenTry {
    //     client_id: "iconclient-1".to_string(),
    //     previous_connection_id: 1.to_string(),
    //     client_state: Some(Any {
    //         type_url: "type.googleapis.com/my.package.MyMessage".to_string(),
    //         value: vec![1, 2, 3],
    //     }),
    //     counterparty: Some(get_dummy_raw_counterparty(None)),
    //     delay_period: 0,
    //     proof_height: Some(Height {
    //         revision_number: 1,
    //         revision_height: 1,
    //     }),
    //     proof_init: vec![1],
    //     proof_client: vec![1],
    //     proof_consensus: vec![1],
    //     consensus_height: Some(Height {
    //         revision_number: 1,
    //         revision_height: 1,
    //     }),
    //     signer: "signer".to_string(),
    //     counterparty_versions: versions,
    // };

    let mut message = get_dummy_raw_msg_conn_open_try(10, 10);
    // message.client_id = "iconclient".to_string();

    let mut res_msg = ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry::try_from(
        message.clone(),
    )
    .unwrap();
    res_msg.client_id_on_b = IbcClientId::default();
    // contract
    //     .ibc_store()
    //     .expected_time_per_block()
    //     .save(deps.as_mut().storage, &(env.block.time.seconds() as u128))
    //     .unwrap();

    // let client_type = ClientType::new("iconclient".to_string());
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
            res_msg.client_id_on_b.clone().into(),
            light_client.to_string(),
        )
        .unwrap();

    // let connection_state: VerifyConnectionState = VerifyConnectionState::new(
    //     "hi".to_string(),
    //     vec![1],
    //     vec![2],
    //     vec![3],
    //     vec![4],
    //     vec![5],
    // );
    // let conn_id = ConnectionId::new(1);
    // let client_id = ClientId::from_str("iconclient-1").unwrap();
    // let counterparty_client_id = ClientId::from_str("counterpartyclient-1").unwrap();
    // let counterparty_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
    //     "hello".as_bytes().to_vec(),
    // )
    // .unwrap();
    // let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
    //     counterparty_client_id.ibc_client_id().clone(),
    //     None,
    //     counterparty_prefix.clone(),
    // );

    // let height = ibc::core::ics02_client::height::Height::new(1, 1).unwrap();

    // contract.validate_self_client(client_state.into()).unwrap();
    // contract.consensus_state_path(client_id.into(), &height);

    // let mock_response_data = OpenTryResponse::new(
    //     conn_id.as_str().to_owned(),
    //     client_id.ibc_client_id().to_string(),
    //     counterparty_client_id.ibc_client_id().to_string(),
    //     "".to_string(),
    //     counterparty_prefix.as_bytes().to_vec(),
    //     to_vec(&versions).unwrap(),
    //     23,
    // );
    // let mock_data_binary = to_binary(&mock_response_data).unwrap();
    // let events = Event::new("open_try");
    // let response = SubMsgResponse {
    //     data: Some(mock_data_binary),
    //     events: vec![events],
    // };
    // let result: SubMsgResult = SubMsgResult::Ok(response);
    // let reply_msg = Reply {
    //     id: EXECUTE_CONNECTION_OPENTRY,
    //     result,
    // };
    let cl = to_vec(&client_state);
    
    contract
        .store_client_state(
            &mut deps.storage,
            &res_msg.client_id_on_b.clone(),
            cl.unwrap(),
        )
        .unwrap();

    let consenus_state = to_vec(&consenus_state).unwrap();
    println!("{:?}",consenus_state);
    contract
        .store_consensus_state(
            &mut deps.storage,
            &res_msg.client_id_on_b.clone(),
            res_msg.proofs_height_on_a.clone(),
            consenus_state,
        )
        .unwrap();

    let res = contract.connection_open_try(res_msg, deps.as_mut(), info);
    println!("{:?}", res);
}
