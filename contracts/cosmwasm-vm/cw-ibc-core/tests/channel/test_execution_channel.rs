use super::*;
use cw_common::{client_response::LightClientResponse, core_msg::ExecuteMsg as CoreExecuteMsg};
use cw_ibc_core::{
    ics04_channel::close_init::on_chan_close_init_submessage, msg::InstantiateMsg,
    EXECUTE_ON_CHANNEL_CLOSE_INIT,
};
use prost::Message;

#[test]
fn test_for_channel_open_init_execution_message() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let message_raw = get_dummy_raw_msg_chan_open_init(None);
    let mut msg = MsgChannelOpenInit::try_from(message_raw.clone()).unwrap();
    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();
    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::types::ModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();
    let commitement = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        commitement.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::from(msg.connection_hops_on_a[0].clone());
    msg.connection_hops_on_a = vec![conn_id.connection_id().clone()];
    msg.version_proposal = Version::from_str("xcall-1").unwrap();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ChannelOpenInit {
            msg: message_raw.encode_to_vec(),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 41);

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_a.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 41,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message).unwrap();

    assert_eq!(response.events[0].ty, "channel_id_created")
}

#[test]
fn test_for_channel_open_try_execution_message() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut msg = MsgChannelOpenTry::try_from(raw.clone()).unwrap();
    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();
    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::types::ModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();
    let commitment = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(0);
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id),
        commitment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(0);
    msg.connection_hops_on_b = vec![conn_id.connection_id().clone()];
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
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
    let height = msg.proof_height_on_a;
    let consenus_state = to_vec(&consenus_state).unwrap();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        CoreExecuteMsg::ChannelOpenTry {
            msg: raw.encode_to_vec(),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 421);

    let reply_message = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_a.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_reponse_data = LightClientResponse {
        message_info: cw_common::types::MessageInfo {
            sender: info.sender.clone(),
            funds: info.funds.clone(),
        },
        ibc_endpoint: reply_message,
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 421,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env.clone(), reply_message);

    assert!(response.is_ok());
    assert_eq!(response.unwrap().messages[0].id, 422);

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_a.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 422,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message);

    assert!(response.is_ok());
    assert_eq!(
        response.as_ref().unwrap().events[0].ty,
        "channel_id_created"
    );
    assert_eq!(response.unwrap().events[1].ty, "channel_open_try")
}

#[test]
fn test_for_channel_open_ack_execution() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_open_ack(10);
    let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id.clone(), module_id.clone())
        .unwrap();
    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::types::ModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();

    let commitement = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::default();
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
    let conn_id = ConnectionId::default();
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

    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        CoreExecuteMsg::ChannelOpenAck {
            msg: raw.encode_to_vec(),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 431);

    let reply_message = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_a.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_reponse_data = LightClientResponse {
        message_info: cw_common::types::MessageInfo {
            sender: info.sender.clone(),
            funds: info.funds.clone(),
        },
        ibc_endpoint: reply_message,
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 431,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    let response = contract.reply(deps.as_mut(), env.clone(), reply_message);

    assert!(response.is_ok());
    assert_eq!(response.unwrap().messages[0].id, 432);

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_a.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 432,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message);

    assert!(response.is_ok());
    assert_eq!(response.as_ref().unwrap().events[0].ty, "channel_open_ack");
    assert_eq!(
        response.unwrap().attributes[0].value,
        "execute_channel_open_ack"
    )
}

#[test]
fn test_for_channel_open_confirm() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_open_confirm(10);
    let msg = MsgChannelOpenConfirm::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let committment = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(5);
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id),
        committment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_b.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id.clone(), module_id.clone())
        .unwrap();

    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::types::ModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();
    let conn_id = ConnectionId::new(0);
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();

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
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id.clone(),
            msg.chan_id_on_b.clone().into(),
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
    let height = msg.proof_height_on_a;
    let consenus_state = to_vec(&consenus_state).unwrap();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        CoreExecuteMsg::ChannelOpenConfirm {
            msg: raw.encode_to_vec(),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 441);

    let reply_message = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_b.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_reponse_data = LightClientResponse {
        message_info: cw_common::types::MessageInfo {
            sender: info.sender.clone(),
            funds: info.funds.clone(),
        },
        ibc_endpoint: reply_message,
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 441,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env.clone(), reply_message);

    assert!(response.is_ok());
    assert_eq!(response.unwrap().messages[0].id, 442);

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_b.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 442,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message);

    assert!(response.is_ok());
    assert_eq!(
        response.as_ref().unwrap().events[0].ty,
        "channel_open_confirm"
    );
}

#[test]
fn test_for_channel_close_init() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw.clone()).unwrap();
    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_a.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();
    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::types::ModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();
    let commitment = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        commitment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let connection_id = ConnectionId::default();
    contract
        .store_connection(
            deps.as_mut().storage,
            connection_id.clone(),
            conn_end.clone(),
        )
        .unwrap();
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

    let expected = on_chan_close_init_submessage(&msg, &channel_end, &connection_id);
    let data = cw_common::xcall_msg::ExecuteMsg::IbcChannelClose { msg: expected };
    let data = to_binary(&data).unwrap();
    let on_chan_open_init = create_channel_submesssage(
        "contractaddress".to_string(),
        data,
        info.funds.clone(),
        EXECUTE_ON_CHANNEL_CLOSE_INIT,
    );

    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        CoreExecuteMsg::ChannelCloseInit {
            port_id_on_a: msg.port_id_on_a.to_string(),
            chan_id_on_a: msg.chan_id_on_a.to_string(),
            signer: "alice".as_bytes().to_vec(),
        },
    );

    assert_eq!(res.is_ok(), true);
    assert_eq!(res.as_ref().unwrap().messages[0].id, 45);
    assert_eq!(res.unwrap().messages[0], on_chan_open_init);

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 45,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message);
    assert!(response.is_ok());
    assert_eq!(
        response.as_ref().unwrap().events[0].ty,
        "channel_close_init"
    );
}

#[test]
fn test_for_channel_close_confirm() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_close_confirm(10);
    let msg = MsgChannelCloseConfirm::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.port_id_on_b.clone());
    let module = Addr::unchecked("contractaddress");
    let cx_module_id = cw_common::types::ModuleId::from(module_id.clone());
    contract
        .add_route(&mut deps.storage, cx_module_id.clone(), &module)
        .unwrap();
    contract
        .store_module_by_port(&mut deps.storage, port_id.clone(), module_id.clone())
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
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();
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
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id.clone(),
            msg.chan_id_on_b.clone().into(),
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
    let height = msg.proof_height_on_a;
    let consenus_state = to_vec(&consenus_state).unwrap();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        CoreExecuteMsg::ChannelCloseConfirm {
            msg: raw.encode_to_vec(),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 461);

    let reply_message = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_b.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_reponse_data = LightClientResponse {
        message_info: cw_common::types::MessageInfo {
            sender: info.sender.clone(),
            funds: info.funds.clone(),
        },
        ibc_endpoint: reply_message,
    };

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 461,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env.clone(), reply_message);

    assert!(response.is_ok());
    assert_eq!(response.unwrap().messages[0].id, 462);

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_b.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 462,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message);

    assert!(response.is_ok());
    assert_eq!(
        response.as_ref().unwrap().events[0].ty,
        "channel_close_confirm"
    );
}
