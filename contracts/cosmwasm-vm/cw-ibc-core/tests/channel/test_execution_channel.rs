use super::*;
use crate::channel::test_receive_packet::{get_dummy_raw_msg_recv_packet, make_ack_success};

use common::ibc::core::ics02_client::height::Height;
use cw_common::core_msg::InstantiateMsg;
use cw_common::{core_msg::ExecuteMsg as CoreExecuteMsg, hex_string::HexString};

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
    let mut test_context = TestContext::for_channel_open_init(env.clone(), &msg);
    test_context.init_channel_open_init(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();

    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::ChannelOpenInit {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{res:?}");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 41);
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
    let mut test_context = TestContext::for_channel_open_try(env.clone(), &msg);
    test_context.init_channel_open_try(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::ChannelOpenTry {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{res:?}");

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, EXECUTE_ON_CHANNEL_OPEN_TRY);
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
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let mut test_context = TestContext::for_channel_open_ack(env.clone(), &msg);
    test_context.init_channel_open_ack(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::ChannelOpenAck {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{res:?}");

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE
    );
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
    let mut test_context = TestContext::for_channel_open_confirm(env.clone(), &msg);

    let mut channel_end = test_context.channel_end();
    channel_end.state = State::TryOpen;
    test_context.channel_end = Some(channel_end);
    test_context.init_channel_open_confirm(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::ChannelOpenConfirm {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{res:?}");

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE
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
    let mut test_context = TestContext::for_channel_close_init(env.clone(), &msg);
    test_context.init_channel_close_init(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();

    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::ChannelCloseInit {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{res:?}");

    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages[0].id, 45);
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
    let mut test_context = TestContext::for_channel_close_confirm(env.clone(), &msg);
    test_context.init_channel_close_confirm(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::ChannelCloseConfirm {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{res:?}");

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE
    );
}

#[test]
fn test_for_packet_send() {
    let mut deps = deps();
    let info = create_mock_info("moduleaddress", "umlg", 20000000);
    let mut contract = CwIbcCoreContext::default();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 110;
    let raw = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");

    let height: Height = RawHeight {
        revision_number: 0,
        revision_height: 100,
    }
    .try_into()
    .unwrap();

    let mut test_context = TestContext::for_send_packet(env.clone(), &raw);

    test_context.init_send_packet(deps.as_mut().storage, &contract);
    let timestamp_query = LightClient::get_timestamp_at_height_query(
        &IbcClientId::default(),
        height.revision_height(),
    )
    .unwrap();
    let mut mocks = test_context.mock_queries.clone();
    mocks.insert(timestamp_query, to_binary(&0_u64).unwrap());
    mock_lightclient_query(mocks, &mut deps);
    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::SendPacket {
            packet: HexString::from_bytes(&raw.encode_to_vec()),
        },
    );
    println!("{res:?},");
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
    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);
    test_context.init_receive_packet(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.execute(
        deps.as_mut(),
        env.clone(),
        info,
        CoreExecuteMsg::ReceivePacket {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{res:?}");
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 10,
    };
    let timeout = IbcTimeout::with_both(timeout_block, cosmwasm_std::Timestamp::from_nanos(100));
    let (src, dst) = get_dummy_endpoints();

    let _packet = IbcPacket::new(vec![0, 1, 2, 3], src, dst, 0, timeout);

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
    println!("{response:?}");
    assert!(response.is_ok());
    assert_eq!(response.unwrap().events[0].ty, "write_acknowledgement");
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
    let msg = get_dummy_raw_msg_acknowledgement(height);

    let mut test_context = TestContext::for_acknowledge_packet(env.clone(), &msg);
    test_context.init_acknowledge_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.execute(
        deps.as_mut(),
        env,
        info,
        CoreExecuteMsg::AcknowledgementPacket {
            msg: HexString::from_bytes(&msg.encode_to_vec()),
        },
    );
    println!("{res:?}");

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE
    );
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
