use super::*;
use crate::channel::test_receive_packet::{get_dummy_raw_msg_recv_packet, make_ack_success};

use common::ibc::core::ics04_channel::packet::Receipt;
use common::ibc::core::ics24_host::identifier::ClientId;

use cw_common::{core_msg::ExecuteMsg as CoreExecuteMsg, hex_string::HexString};
use cw_ibc_core::conversions::{
    to_ibc_channel_id, to_ibc_timeout_block, to_ibc_timeout_height, to_ibc_timestamp,
};
use cw_ibc_core::light_client::light_client::LightClient;
use cw_ibc_core::{
    ics04_channel::close_init::on_chan_close_init_submessage, msg::InstantiateMsg,
    EXECUTE_ON_CHANNEL_CLOSE_INIT,
};
use cw_ibc_core::{
    EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE, EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
    EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE, VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
    VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
};
use prost::Message;

#[test]
fn test_for_channel_open_init_execution_message() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let msg = get_dummy_raw_msg_chan_open_init(None);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();

    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel = to_ibc_channel(msg.channel.clone()).unwrap();

    let module = Addr::unchecked("contractaddress");
    let _cx_module_id = module_id;
    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();

    let commitement = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        commitement.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );

    contract
        .store_connection(
            deps.as_mut().storage,
            &channel.connection_hops[0],
            &conn_end,
        )
        .unwrap();
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ChannelOpenInit {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 41);

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_INIT,
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let msg = get_dummy_raw_msg_chan_open_try(10);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let _channel = to_ibc_channel(msg.channel.clone()).unwrap();
    let light_client = LightClient::new("lightclient".to_string());

    contract
        .bind_port(
            &mut deps.storage,
            &port_id,
            Addr::unchecked("moduleaddress"),
        )
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let commitment = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(0);
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id),
        commitment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(0);

    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
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
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ChannelOpenTry {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, EXECUTE_ON_CHANNEL_OPEN_TRY);

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_TRY,
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let msg = get_dummy_raw_msg_chan_open_ack(10);
    //let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());

    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let counter_channel_id = to_ibc_channel_id(&msg.counterparty_channel_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .bind_port(
            &mut deps.storage,
            &port_id,
            Addr::unchecked("moduleaddress"),
        )
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let commitement = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::default();
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
    let conn_id = ConnectionId::default();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();
    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(counter_channel_id),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
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

    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ChannelOpenAck {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE
    );

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let msg = get_dummy_raw_msg_chan_open_confirm(10);

    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let committment = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(5);
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id),
        committment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );

    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let light_client = LightClient::new("lightclient".to_string());

    contract
        .bind_port(
            &mut deps.storage,
            &port_id,
            Addr::unchecked("moduleaddress"),
        )
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let conn_id = ConnectionId::new(0);
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();

    let channel_end = ChannelEnd {
        state: State::TryOpen,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
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
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ChannelOpenConfirm {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE
    );

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: ChannelId::default().to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE,
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let msg = get_dummy_raw_msg_chan_close_init();

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();

    let module = Addr::unchecked("contractaddress");
    let _cx_module_id = module_id;

    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();
    let commitment = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        commitment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let connection_id = ConnectionId::default();
    contract
        .store_connection(deps.as_mut().storage, &connection_id, &conn_end)
        .unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id.clone()],
        version: Version::new("xcall".to_string()),
    };

    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
        .unwrap();

    let expected =
        on_chan_close_init_submessage(&port_id, &channel_id, &channel_end, &connection_id);
    let data = cw_common::xcall_connection_msg::ExecuteMsg::IbcChannelClose { msg: expected };
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
        info,
        CoreExecuteMsg::ChannelCloseInit {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );

    assert!(res.is_ok());
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let msg = get_dummy_raw_msg_chan_close_confirm(10);
    //let msg = MsgChannelCloseConfirm::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());

    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .bind_port(
            &mut deps.storage,
            &port_id,
            Addr::unchecked("moduleaddress"),
        )
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
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
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
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ChannelCloseConfirm {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{:?}", res);

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE
    );

    let mock_reponse_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE,
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

#[test]
fn test_for_packet_send() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let raw = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let chan_end_on_a = ChannelEnd::new(
        State::TryOpen,
        Order::default(),
        Counterparty::new(
            IbcPortId::from_str(&raw.destination_port).unwrap(),
            Some(IbcChannelId::from_str(&raw.destination_channel).unwrap()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );

    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );

    let conn_end_on_a = ConnectionEnd::new(
        ConnectionState::Open,
        ClientId::default(),
        ConnectionCounterparty::new(
            ClientId::default(),
            Some(ConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );

    let packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    contract
        .store_channel_end(&mut deps.storage, &src_port, &src_channel, &chan_end_on_a)
        .unwrap();
    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, conn_id_on_a, &conn_end_on_a)
        .unwrap();
    contract
        .store_next_sequence_send(
            &mut deps.storage,
            &src_port.clone(),
            &src_channel,
            &1.into(),
        )
        .unwrap();

    let client_state = ClientState {
        latest_height: 10,
        ..get_dummy_client_state()
    };

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
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = RawHeight {
        revision_number: 0,
        revision_height: 10,
    }
    .try_into()
    .unwrap();
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

    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::SendPacket {
            packet: HexString::from_bytes(&raw.encode_to_vec()),
        },
    );
    println!("{:?},", res);
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().attributes[0].value, "send_packet");
    assert_eq!(res.unwrap().events[0].ty, "send_packet")
}

#[test]
fn test_for_recieve_packet() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");
    let msg = get_dummy_raw_msg_recv_packet(12);
    //  let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();
    let light_client = LightClient::new("lightclient".to_string());

    contract
        .bind_port(
            &mut deps.storage,
            &dst_port,
            Addr::unchecked("moduleaddress"),
        )
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);
    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::default(),
        Counterparty::new(src_port, Some(src_channel)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );

    let conn_end_on_b = ConnectionEnd::new(
        ConnectionState::Open,
        IbcClientId::default(),
        ConnectionCounterparty::new(
            IbcClientId::default(),
            Some(IbcConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );

    contract
        .store_channel_end(&mut deps.storage, &dst_port, &dst_channel, &chan_end_on_b)
        .unwrap();
    let conn_id_on_b = &chan_end_on_b.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, &conn_id_on_b.clone(), &conn_end_on_b)
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
    let env = get_mock_env();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    contract
        .store_channel_end(&mut deps.storage, &dst_port, &dst_channel, &chan_end_on_b)
        .unwrap();

    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ReceivePacket {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );

    contract
        .store_packet_receipt(
            &mut deps.storage,
            &dst_port,
            &dst_channel,
            Sequence::from(packet.sequence),
            Receipt::Ok,
        )
        .unwrap();

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 10,
    };
    let timeout = IbcTimeout::with_both(timeout_block, cosmwasm_std::Timestamp::from_nanos(100));
    let (src, dst) = get_dummy_endpoints();

    let packet = IbcPacket::new(vec![0, 1, 2, 3], src, dst, 0, timeout);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
            &packet,
        )
        .unwrap();

    let mock_data_binary = to_binary(&make_ack_success().to_vec()).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message);
    println!("{:?}", response);
    assert!(response.is_ok());
    assert_eq!(response.unwrap().events[0].ty, "recv_packet");
}

#[test]
fn test_for_ack_execute() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let height = 50;
    let raw = get_dummy_raw_msg_acknowledgement(height);
    let msg = get_dummy_raw_msg_acknowledgement(height);

    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();
    let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone()).unwrap();
    let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp).unwrap();
    let _packet_sequence = Sequence::from(packet.sequence);
    let proof_height = to_ibc_height(msg.proof_height.clone()).unwrap();
    let _src = IbcEndpoint {
        port_id: src_port.to_string(),
        channel_id: src_channel.to_string(),
    };
    let _dest = IbcEndpoint {
        port_id: dst_port.to_string(),
        channel_id: dst_channel.to_string(),
    };
    let timeoutblock = to_ibc_timeout_block(&packet_timeout_height);
    let timestamp = packet_timestamp.nanoseconds();
    let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
    let _timeout = IbcTimeout::with_both(timeoutblock, ibctimestamp);
    //Store channel, connection and packet commitment
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(dst_port.clone(), Some(dst_channel.clone())),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port,
            &src_channel,
            &chan_end_on_a_ordered,
        )
        .unwrap();
    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let conn_end_on_a = ConnectionEnd::new(
        ConnectionState::Open,
        ClientId::default(),
        ConnectionCounterparty::new(
            ClientId::default(),
            Some(ConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );
    contract
        .store_connection(
            &mut deps.storage,
            &chan_end_on_a_ordered.connection_hops()[0].clone(),
            &conn_end_on_a,
        )
        .unwrap();
    let packet_commitment =
        compute_packet_commitment(&packet.data, &packet_timeout_height, &packet_timestamp);
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &src_port,
            &src_channel,
            packet.sequence.into(),
            packet_commitment,
        )
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
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .bind_port(
            &mut deps.storage,
            &dst_port,
            Addr::unchecked("moduleaddress"),
        )
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::AcknowledgementPacket {
            msg: HexString::from_bytes(&raw.encode_to_vec()),
        },
    );
    println!("{:?}", res);

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE
    );

    let src = IbcEndpoint {
        port_id: src_port.to_string(),
        channel_id: src_channel.to_string(),
    };
    let dest = IbcEndpoint {
        port_id: dst_port.to_string(),
        channel_id: dst_channel.to_string(),
    };
    let timeoutblock = to_ibc_timeout_block(&packet_timeout_height);
    let timestamp = packet_timestamp.nanoseconds();
    let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
    let timeout = IbcTimeout::with_both(timeoutblock, ibctimestamp);
    let ibc_packet = IbcPacket::new(packet.data, src, dest, packet.sequence, timeout);
    let ack = IbcAcknowledgement::new(msg.acknowledgement);
    let address = Addr::unchecked(msg.signer);
    let mock_reponse_data = cosmwasm_std::IbcPacketAckMsg::new(ack, ibc_packet, address);
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message);

    assert!(response.is_ok());
    assert_eq!(
        "execute_acknowledgement_packet",
        response.unwrap().attributes[1].value
    )
}

#[test]
fn test_for_timeout_execution() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let contract = CwIbcCoreContext::default();
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env, info, InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");
}
