use super::*;
use cosmwasm_std::IbcChannel;
use cw_ibc_core::ics04_channel::{
    open_ack::{channel_open_ack_validate, on_chan_open_ack_submessage},
    open_try::channel_open_try_msg_validate,
    EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
};

#[test]
#[should_panic(expected = "UndefinedConnectionCounterparty")]
fn test_validate_open_ack_channel_fail_missing_counterparty() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_ack(10);
    let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let port_id = PortId::from(msg.port_id_on_a.clone());

    let committment = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        committment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();

    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.ibc_port_id().clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id.connection_id().clone()],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id.clone(),
            msg.chan_id_on_a.clone().into(),
            channel_end,
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

    let client = to_vec(&client_state);
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client.unwrap())
        .unwrap();
    let client_type = ClientType::from(IbcClientType::new("iconclient".to_string()));

    contract
        .store_client_into_registry(
            &mut deps.storage,
            client_type,
            "contractaddress".to_string(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_b;
    let consenus_state = to_vec(&consenus_state).unwrap();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();

    contract
        .validate_channel_open_ack(deps.as_mut(), info.clone(), &msg)
        .unwrap();
}

#[test]
fn test_validate_open_ack_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_ack(10);
    let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_ibc_core::types::ModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();

    let commitement = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(5);
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id),
        commitement.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();

    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.ibc_port_id().clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id.connection_id().clone()],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id.clone(),
            msg.chan_id_on_a.clone().into(),
            channel_end,
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

    let client = to_vec(&client_state);
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client.unwrap())
        .unwrap();
    let client_type = ClientType::from(IbcClientType::new("iconclient".to_string()));

    contract
        .store_client_into_registry(
            &mut deps.storage,
            client_type,
            "contractaddress".to_string(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_b;
    let consenus_state = to_vec(&consenus_state).unwrap();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();

    let res = contract.validate_channel_open_ack(deps.as_mut(), info.clone(), &msg);

    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap().messages[0].id, 431)
}

#[test]
fn test_execute_open_ack_from_light_client() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_ack(10);
    let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let channel_id_on_b = ChannelId::new(0);
    let ss = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(5);
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id.clone()),
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    contract
        .store_connection(
            deps.as_mut().storage,
            connection_id.clone().into(),
            conn_end.clone(),
        )
        .unwrap();
    let counter_party = Counterparty::new(msg.port_id_on_a.clone(), Some(msg.chan_id_on_b.clone()));
    let channel_end = ChannelEnd::new(
        State::Init,
        Order::Unordered,
        counter_party,
        vec![connection_id.clone()],
        Version::from_str("xcall").unwrap(),
    );
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();

    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_ibc_core::types::ModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();

    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: PortId::from(msg.port_id_on_a.clone()).to_string(),
        channel_id: channel_id_on_b.clone().to_string(),
    };
    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("action").add_attribute("action", "channel open try execution")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_TRY,
        result,
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            PortId::from(msg.port_id_on_a.clone()),
            channel_id_on_b.clone(),
            channel_end.clone(),
        )
        .unwrap();

    let expected = on_chan_open_ack_submessage(
        &channel_end,
        &PortId::from(msg.port_id_on_a.clone()),
        &channel_id_on_b.clone(),
        &connection_id.into(),
    );
    let data = cw_xcall::msg::ExecuteMsg::IbcChannelConnect {
        msg: expected.unwrap(),
    };
    let data = to_binary(&data).unwrap();
    let on_chan_open_ack = create_channel_submesssage(
        "contractaddress".to_string(),
        data,
        &info,
        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
    );
    let res = contract.execute_open_ack_from_light_client_reply(deps.as_mut(), info.clone(), reply);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap().messages[0], on_chan_open_ack)
}

#[test]
fn test_execute_open_ack_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let connection_id = ConnectionId::new(5);
    let channel_id = ChannelId::from(msg.chan_id_on_a.clone());
    let port_id = PortId::from(msg.port_id_on_a.clone());
    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.ibc_port_id().clone(),
            channel_id: Some(channel_id.ibc_channel_id().clone()),
        },
        connection_hops: vec![connection_id.connection_id().clone()],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id.clone(),
            channel_id.clone(),
            channel_end.clone(),
        )
        .unwrap();
    contract
        .store_channel(
            deps.as_mut().storage,
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            channel_end,
        )
        .unwrap();
    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.ibc_port_id().clone().to_string(),
        channel_id: channel_id.ibc_channel_id().clone().to_string(),
    };
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
    let msg = MsgChannelCloseInit::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let connection_id = ConnectionId::new(5);
    let channel_id = ChannelId::from(msg.chan_id_on_a.clone());
    let port_id = PortId::from(msg.port_id_on_a.clone());
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.ibc_port_id().clone(),
            channel_id: Some(channel_id.ibc_channel_id().clone()),
        },
        connection_hops: vec![connection_id.connection_id().clone()],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id.clone(),
            channel_id.clone(),
            channel_end.clone(),
        )
        .unwrap();
    contract
        .store_channel(
            deps.as_mut().storage,
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            channel_end,
        )
        .unwrap();
    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.ibc_port_id().clone().to_string(),
        channel_id: channel_id.ibc_channel_id().clone().to_string(),
    };
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
    let raw = get_dummy_raw_msg_chan_open_ack(10);
    let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let conn_id = ConnectionId::new(5);
    let port_id = PortId::from(msg.port_id_on_a.clone());
    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.ibc_port_id().clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id.connection_id().clone()],
        version: Version::new("xcall".to_string()),
    };
    let res = channel_open_ack_validate(&msg, &channel_end);

    assert!(res.is_ok())
}

#[test]
#[should_panic(expected = "InvalidChannelState")]
fn test_channel_open_ack_validate_fail() {
    let raw = get_dummy_raw_msg_chan_open_ack(10);
    let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let conn_id = ConnectionId::new(5);
    let port_id = PortId::from(msg.port_id_on_a.clone());
    let channel_end = ChannelEnd {
        state: State::TryOpen,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.ibc_port_id().clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id.connection_id().clone()],
        version: Version::new("xcall".to_string()),
    };
    channel_open_ack_validate(&msg, &channel_end).unwrap();
}

#[test]
pub fn test_on_chan_open_ack_submessage() {
    let raw = get_dummy_raw_msg_chan_close_confirm(10);
    let msg = MsgChannelCloseConfirm::try_from(raw.clone()).unwrap();
    let conn_id = ConnectionId::new(5);
    let port_id = PortId::from(msg.port_id_on_b.clone());
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.ibc_port_id().clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id.connection_id().clone()],
        version: Version::new("xcall".to_string()),
    };
    let endpoint = cosmwasm_std::IbcEndpoint {
        port_id: port_id.ibc_port_id().to_string(),
        channel_id: msg.chan_id_on_b.to_string(),
    };
    let counter_party = cosmwasm_std::IbcEndpoint {
        port_id: channel_end.remote.port_id.to_string(),
        channel_id: channel_end.clone().remote.channel_id.unwrap().to_string(),
    };
    let res = on_chan_open_ack_submessage(
        &channel_end,
        &port_id,
        &msg.chan_id_on_b.clone().into(),
        &conn_id,
    );
    let expected = cosmwasm_std::IbcChannelConnectMsg::OpenAck {
        channel: IbcChannel::new(
            endpoint,
            counter_party,
            cosmwasm_std::IbcOrder::Unordered,
            "xcall".to_string(),
            conn_id.connection_id().to_string(),
        ),
        counterparty_version: channel_end.version().to_string(),
    };

    assert_eq!(res.unwrap(), expected);
}

#[test]
#[should_panic(expected = "InvalidVersionLengthConnection")]
fn test_channel_open_try_validate_fail_invalid_connection_lenght() {
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let msg = MsgChannelOpenTry::try_from(raw.clone()).unwrap();
    let mut connection_end = ConnectionEnd::default();
    connection_end.set_state(ibc::core::ics03_connection::connection::State::Open);
    channel_open_try_msg_validate(&msg, &connection_end).unwrap();
}

#[test]
fn test_channel_open_try_validate() {
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let msg = MsgChannelOpenTry::try_from(raw.clone()).unwrap();
    let mut connection_end = ConnectionEnd::default();
    connection_end.set_state(ibc::core::ics03_connection::connection::State::Open);
    connection_end.set_version(ibc::core::ics03_connection::version::Version::default());
    let res = channel_open_try_msg_validate(&msg, &connection_end);
    assert!(res.is_ok());
}
