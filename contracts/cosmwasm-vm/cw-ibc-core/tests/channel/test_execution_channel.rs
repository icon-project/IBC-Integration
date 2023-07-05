use super::*;
use crate::channel::test_receive_packet::{get_dummy_raw_msg_recv_packet, make_ack_success};

use common::ibc::core::ics04_channel::{msgs::recv_packet::MsgRecvPacket, packet::Receipt};
use common::ibc::core::ics24_host::identifier::ClientId;

use cw_common::raw_types::Protobuf;
use cw_common::{
    client_response::{LightClientResponse, PacketResponse},
    core_msg::ExecuteMsg as CoreExecuteMsg,
    hex_string::HexString,
};
use cw_ibc_core::{
    ics04_channel::close_init::on_chan_close_init_submessage, msg::InstantiateMsg,
    EXECUTE_ON_CHANNEL_CLOSE_INIT,
};
use cw_ibc_core::{
    VALIDATE_ON_PACKET_RECEIVE_ON_LIGHT_CLIENT, VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
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

    let message_raw = get_dummy_raw_msg_chan_open_init(None);
    let mut msg = MsgChannelOpenInit::try_from(message_raw.clone()).unwrap();
    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();

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
    let conn_id = msg.connection_hops_on_a[0].clone();
    msg.connection_hops_on_a = vec![conn_id.clone()];
    msg.version_proposal = Version::from_str("xcall-1").unwrap();
    contract
        .store_connection(deps.as_mut().storage, conn_id, conn_end)
        .unwrap();
    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ChannelOpenInit {
            msg: HexString::from_bytes(&message_raw.encode_to_vec()),
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut msg = MsgChannelOpenTry::try_from(raw.clone()).unwrap();
    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();

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
    msg.connection_hops_on_b = vec![conn_id.clone()];
    contract
        .store_connection(deps.as_mut().storage, conn_id, conn_end)
        .unwrap();
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();
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
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_a;
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
        info.clone(),
        CoreExecuteMsg::ChannelOpenTry {
            msg: HexString::from_bytes(&raw.encode_to_vec()),
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
            funds: info.funds,
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_open_ack(10);
    let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();

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
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end)
        .unwrap();
    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id,
            msg.chan_id_on_a.clone(),
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
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();
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
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_b;
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
        info.clone(),
        CoreExecuteMsg::ChannelOpenAck {
            msg: HexString::from_bytes(&raw.encode_to_vec()),
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
            funds: info.funds,
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_open_confirm(10);
    let msg = MsgChannelOpenConfirm::try_from(raw.clone()).unwrap();
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
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_b.clone();

    let module = Addr::unchecked("contractaddress");
    let _cx_module_id = module_id;

    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();
    let conn_id = ConnectionId::new(0);
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end)
        .unwrap();

    let channel_end = ChannelEnd {
        state: State::TryOpen,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id,
            msg.chan_id_on_b.clone(),
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
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();
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
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_a;
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
        info.clone(),
        CoreExecuteMsg::ChannelOpenConfirm {
            msg: HexString::from_bytes(&raw.encode_to_vec()),
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
            funds: info.funds,
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
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();

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
        .store_connection(deps.as_mut().storage, connection_id.clone(), conn_end)
        .unwrap();
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a.clone();
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
        info,
        CoreExecuteMsg::ChannelCloseInit {
            msg: HexString::from_bytes(&msg.encode_vec().unwrap()),
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

    let raw = get_dummy_raw_msg_chan_close_confirm(10);
    let msg = MsgChannelCloseConfirm::try_from(raw.clone()).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_b.clone();
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
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end)
        .unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(msg.chan_id_on_b.clone()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id,
            msg.chan_id_on_b.clone(),
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
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();
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
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_a;
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
        info.clone(),
        CoreExecuteMsg::ChannelCloseConfirm {
            msg: HexString::from_bytes(&raw.encode_to_vec()),
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
            funds: info.funds,
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

#[test]
fn test_for_packet_send() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = get_mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let chan_end_on_a = ChannelEnd::new(
        State::TryOpen,
        Order::default(),
        Counterparty::new(IbcPortId::default(), Some(IbcChannelId::default())),
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
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let raw = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    let packet: Packet =
        get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds())
            .try_into()
            .unwrap();

    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a.clone(),
            chan_end_on_a.clone(),
        )
        .unwrap();
    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, conn_id_on_a.clone(), conn_end_on_a)
        .unwrap();
    contract
        .store_next_sequence_send(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a,
            1.into(),
        )
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 10,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

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
    let raw = get_dummy_raw_msg_recv_packet(12);
    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();
    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::default(),
        Counterparty::new(packet.port_id_on_a, Some(packet.chan_id_on_a.clone())),
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
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone(),
            packet.chan_id_on_b.clone(),
            chan_end_on_b.clone(),
        )
        .unwrap();
    let conn_id_on_b = &chan_end_on_b.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, conn_id_on_b.clone(), conn_end_on_b)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 12,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();

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
    }
    .try_into()
    .unwrap();

    let height = msg.proof_height_on_a;
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
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            IbcClientId::default(),
            light_client.to_string(),
        )
        .unwrap();
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone(),
            packet.chan_id_on_b.clone(),
            chan_end_on_b.clone(),
        )
        .unwrap();

    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        CoreExecuteMsg::ReceivePacket {
            msg: HexString::from_bytes(&raw.encode_to_vec()),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 521);

    let message_info = cw_common::types::MessageInfo {
        sender: info.sender,
        funds: info.funds,
    };
    let packet_repsone = PacketResponse {
        seq_on_a: msg.packet.sequence,
        port_id_on_a: msg.packet.port_id_on_a.clone(),
        chan_id_on_a: msg.packet.chan_id_on_a.clone(),
        port_id_on_b: msg.packet.port_id_on_b.clone(),
        chan_id_on_b: msg.packet.chan_id_on_b,
        data: msg.packet.data,
        timeout_height_on_b: msg.packet.timeout_height_on_b,
        timeout_timestamp_on_b: msg.packet.timeout_timestamp_on_b,
    };

    let mock_reponse_data = PacketDataResponse {
        packet: packet_repsone,
        signer: msg.signer,
        acknowledgement: None,
        message_info,
    };

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: VALIDATE_ON_PACKET_RECEIVE_ON_LIGHT_CLIENT,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    contract
        .store_packet_receipt(
            &mut deps.storage,
            &msg.packet.port_id_on_a,
            &msg.packet.chan_id_on_a,
            msg.packet.sequence,
            Receipt::Ok,
        )
        .unwrap();
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.packet.port_id_on_b;

    let module = Addr::unchecked("contractaddress");

    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();
    let response = contract.reply(deps.as_mut(), env.clone(), reply_message);

    assert!(response.is_ok());

    assert_eq!(
        response.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 10,
    };
    let timeout = IbcTimeout::with_both(timeout_block, cosmwasm_std::Timestamp::from_nanos(100));
    let src = IbcEndpoint {
        port_id: packet.port_id_on_b.to_string(),
        channel_id: packet.chan_id_on_a.to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

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
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();

    let packet = msg.packet.clone();
    //Store channel, connection and packet commitment
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(
            packet.port_id_on_b.clone(),
            Some(packet.chan_id_on_b.clone()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a.clone(),
            chan_end_on_a_ordered.clone(),
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
            chan_end_on_a_ordered.connection_hops()[0].clone(),
            conn_end_on_a,
        )
        .unwrap();
    let packet_commitment = compute_packet_commitment(
        &msg.packet.data,
        &msg.packet.timeout_height_on_b,
        &msg.packet.timeout_timestamp_on_b,
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &packet.port_id_on_a,
            &packet.chan_id_on_a,
            packet.sequence,
            packet_commitment,
        )
        .unwrap();
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();
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
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_b;
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
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            IbcClientId::default(),
            light_client.to_string(),
        )
        .unwrap();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        CoreExecuteMsg::AcknowledgementPacket {
            msg: HexString::from_bytes(&raw.encode_to_vec()),
        },
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 531);

    let packet_repsone = PacketResponse {
        seq_on_a: msg.packet.sequence,
        port_id_on_a: msg.packet.port_id_on_a.clone(),
        chan_id_on_a: msg.packet.chan_id_on_a.clone(),
        port_id_on_b: msg.packet.port_id_on_b.clone(),
        chan_id_on_b: msg.packet.chan_id_on_b.clone(),
        data: msg.packet.data.clone(),
        timeout_height_on_b: msg.packet.timeout_height_on_b,
        timeout_timestamp_on_b: msg.packet.timeout_timestamp_on_b,
    };
    let message_info = cw_common::types::MessageInfo {
        sender: info.sender,
        funds: info.funds,
    };
    let mock_reponse_data = PacketDataResponse {
        message_info,
        packet: packet_repsone,
        signer: msg.signer.clone(),
        acknowledgement: Some(msg.acknowledgement.clone()),
    };
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 531,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.packet.port_id_on_a.clone();

    let module = Addr::unchecked("contractaddress");

    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();
    let response = contract.reply(deps.as_mut(), env.clone(), reply_message);

    assert!(response.is_ok());
    assert_eq!(response.as_ref().unwrap().clone().messages[0].id, 532);

    let src = IbcEndpoint {
        port_id: msg.packet.port_id_on_a.to_string(),
        channel_id: msg.packet.chan_id_on_a.to_string(),
    };
    let dest = IbcEndpoint {
        port_id: msg.packet.port_id_on_b.to_string(),
        channel_id: msg.packet.chan_id_on_b.to_string(),
    };
    let timeoutblock = match msg.packet.timeout_height_on_b {
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::Never => IbcTimeoutBlock {
            revision: 1,
            height: 1,
        },
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::At(x) => IbcTimeoutBlock {
            revision: x.revision_number(),
            height: x.revision_height(),
        },
    };
    let timestamp = msg.packet.timeout_timestamp_on_b.nanoseconds();
    let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
    let timeout = IbcTimeout::with_both(timeoutblock, ibctimestamp);
    let ibc_packet = IbcPacket::new(
        msg.packet.data,
        src,
        dest,
        msg.packet.sequence.into(),
        timeout,
    );
    let ack = IbcAcknowledgement::new(msg.acknowledgement.as_bytes());
    let address = Addr::unchecked(msg.signer.to_string());
    let mock_reponse_data = cosmwasm_std::IbcPacketAckMsg::new(ack, ibc_packet, address);
    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();
    let event = Event::new("empty");
    let reply_message = Reply {
        id: 532,
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
