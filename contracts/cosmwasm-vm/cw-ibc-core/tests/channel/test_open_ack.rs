use super::*;
use cosmwasm_std::IbcChannel;

use cw_ibc_core::{
    conversions::to_ibc_channel_id,
    ics04_channel::{
        open_ack::{channel_open_ack_validate, on_chan_open_ack_submessage},
        open_try::channel_open_try_msg_validate,
        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
    },
    light_client::light_client::LightClient,
};

#[test]
#[should_panic(expected = "UndefinedConnectionCounterparty")]
fn test_validate_open_ack_channel_fail_missing_counterparty() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_open_ack(10);
    // let msg = MsgChannelOpenAck::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let src_port = to_ibc_port_id(&msg.port_id).unwrap();
    let src_channel = to_ibc_channel_id(&msg.channel_id).unwrap();

    let dst_channel = to_ibc_channel_id(&msg.counterparty_channel_id).unwrap();

    let committment = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        committment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();

    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: src_port.clone(),
            channel_id: Some(dst_channel),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(&mut deps.storage, &src_port, &src_channel, &channel_end)
        .unwrap();
    let client_state: ClientState = get_dummy_client_state();

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &env,
            &IbcClientId::default(),
            client,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let client_type = IbcClientType::new("iconclient".to_string());

    contract
        .store_client_into_registry(
            &mut deps.storage,
            client_type,
            "contractaddress".to_string(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = to_ibc_height(msg.proof_height.clone()).unwrap();
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    contract
        .validate_channel_open_ack(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_validate_open_ack_channel() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let msg = get_dummy_raw_msg_chan_open_ack(10);
    //  let msg = MsgChannelOpenAck::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let src_port = to_ibc_port_id(&msg.port_id).unwrap();
    let src_channel = to_ibc_channel_id(&msg.channel_id).unwrap();

    let dst_channel = to_ibc_channel_id(&msg.counterparty_channel_id).unwrap();
    let light_client = LightClient::new("lightclient".to_string());

    contract
        .bind_port(&mut deps.storage, &src_port, "moduleaddress".to_string())
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let commitement = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(5);
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id),
        commitement.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();

    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: src_port.clone(),
            channel_id: Some(dst_channel),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(&mut deps.storage, &src_port, &src_channel, &channel_end)
        .unwrap();

    let client_state: ClientState = get_dummy_client_state();

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &env,
            &IbcClientId::default(),
            client,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let client_type = IbcClientType::new("iconclient".to_string());

    contract
        .store_client_into_registry(
            &mut deps.storage,
            client_type,
            "contractaddress".to_string(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = to_ibc_height(msg.proof_height.clone()).unwrap();
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let res = contract.validate_channel_open_ack(deps.as_mut(), info, &msg);

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE
    )
}

#[test]
fn test_execute_open_ack_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let connection_id = ConnectionId::new(5);
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a;
    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
        .unwrap();
    contract
        .store_channel_commitment(deps.as_mut().storage, &port_id, &channel_id, &channel_end)
        .unwrap();
    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    contract
        .store_callback_data(
            deps.as_mut().storage,
            EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
            &expected_data,
        )
        .unwrap();
    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("Action").add_attribute("method", "channel_open_ack")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
        result,
    };

    let result = contract.execute_channel_open_ack(deps.as_mut(), reply);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "InvalidChannelState")]
fn test_execute_open_ack_channel_fail_invalid_state() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let connection_id = ConnectionId::new(5);
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a;
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
        .unwrap();
    contract
        .store_channel_commitment(deps.as_mut().storage, &port_id, &channel_id, &channel_end)
        .unwrap();
    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    contract
        .store_callback_data(
            deps.as_mut().storage,
            EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
            &expected_data,
        )
        .unwrap();

    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("Action").add_attribute("method", "channel_open_ack")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
        result,
    };

    contract
        .execute_channel_open_ack(deps.as_mut(), reply)
        .unwrap();
}

#[test]
fn test_channel_open_ack_validate() {
    let msg = get_dummy_raw_msg_chan_open_ack(10);
    // let msg = MsgChannelOpenAck::try_from(raw).unwrap();
    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(to_ibc_channel_id(&msg.counterparty_channel_id).unwrap()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    let res = channel_open_ack_validate(&channel_id, &channel_end);

    assert!(res.is_ok())
}

#[test]
#[should_panic(expected = "InvalidChannelState")]
fn test_channel_open_ack_validate_fail() {
    let msg = get_dummy_raw_msg_chan_open_ack(10);
    // let msg = MsgChannelOpenAck::try_from(raw).unwrap();
    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::TryOpen,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(to_ibc_channel_id(&msg.counterparty_channel_id).unwrap()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    channel_open_ack_validate(&channel_id, &channel_end).unwrap();
}

#[test]
pub fn test_on_chan_open_ack_submessage() {
    let raw = get_dummy_raw_msg_chan_close_confirm(10);
    let msg = MsgChannelCloseConfirm::try_from(raw).unwrap();
    let conn_id = ConnectionId::new(5);
    let port_id = msg.port_id_on_b.clone();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id.clone()],
        version: Version::new("xcall".to_string()),
    };
    let endpoint = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: msg.chan_id_on_b.to_string(),
    };
    let counter_party = cosmwasm_std::IbcEndpoint {
        port_id: channel_end.remote.port_id.to_string(),
        channel_id: channel_end.clone().remote.channel_id.unwrap().to_string(),
    };
    let res = on_chan_open_ack_submessage(&channel_end, &port_id, &msg.chan_id_on_b, &conn_id);
    let expected = cosmwasm_std::IbcChannelConnectMsg::OpenAck {
        channel: IbcChannel::new(
            endpoint,
            counter_party,
            cosmwasm_std::IbcOrder::Unordered,
            "xcall".to_string(),
            conn_id.to_string(),
        ),
        counterparty_version: channel_end.version().to_string(),
    };

    assert_eq!(res.unwrap(), expected);
}

#[test]
#[should_panic(expected = "InvalidVersionLengthConnection")]
fn test_channel_open_try_validate_fail_invalid_connection_lenght() {
    let raw = get_dummy_raw_msg_chan_open_try(10);
    // let msg = MsgChannelOpenTry::try_from(raw).unwrap();
    let channel = to_ibc_channel(raw.channel).unwrap();
    let mut connection_end = ConnectionEnd::default();
    connection_end.set_state(common::ibc::core::ics03_connection::connection::State::Open);
    channel_open_try_msg_validate(&channel, &connection_end).unwrap();
}

#[test]
fn test_channel_open_try_validate() {
    let raw = get_dummy_raw_msg_chan_open_try(10);
    // let msg = MsgChannelOpenTry::try_from(raw).unwrap();
    let channel = to_ibc_channel(raw.channel).unwrap();
    let mut connection_end = ConnectionEnd::default();
    connection_end.set_state(common::ibc::core::ics03_connection::connection::State::Open);
    connection_end.set_version(common::ibc::core::ics03_connection::version::Version::default());
    let res = channel_open_try_msg_validate(&channel, &connection_end);
    assert!(res.is_ok());
}
