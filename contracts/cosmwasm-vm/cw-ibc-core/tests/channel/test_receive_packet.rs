use common::ibc::core::ics03_connection::connection::State as ConnectionState;

use common::ibc::core::ics04_channel::commitment::AcknowledgementCommitment;
use common::ibc::core::ics04_channel::packet::Receipt;
use common::ibc::core::ics04_channel::timeout::TimeoutHeight;
use common::ibc::timestamp::Timestamp;

use cosmwasm_std::to_vec;
use cw_common::raw_types::channel::RawMsgRecvPacket;
use cw_common::types::Ack;
use cw_ibc_core::conversions::{to_ibc_channel_id, to_ibc_port_id, to_ibc_timeout_block};

use cw_ibc_core::VALIDATE_ON_PACKET_RECEIVE_ON_MODULE;

use super::*;

pub fn make_ack_success() -> Binary {
    let res = Ack::Result(b"1".into());

    to_binary(&res).unwrap()
}

pub fn get_dummy_raw_packet_recv(timeout_height: u64, timeout_timestamp: u64) -> RawPacket {
    let (src, dest) = get_dummy_endpoints();
    RawPacket {
        sequence: 1,
        source_port: src.port_id,
        source_channel: src.channel_id,
        destination_port: dest.port_id,
        destination_channel: dest.channel_id,
        data: vec![0],
        timeout_height: Some(RawHeight {
            revision_number: 12,
            revision_height: timeout_height,
        }),
        timeout_timestamp,
    }
}

pub fn get_dummy_raw_msg_recv_packet(height: u64) -> RawMsgRecvPacket {
    let timestamp = Timestamp::default();
    RawMsgRecvPacket {
        packet: Some(get_dummy_raw_packet_recv(height, timestamp.nanoseconds())),
        proof_commitment: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

#[test]
fn test_receive_packet() {
    let mut contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();

    let info = create_mock_info("channel-creater", "umlg", 2000000000);

    let msg = get_dummy_raw_msg_recv_packet(12);
    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);
    let packet = msg.packet.clone().unwrap();

    test_context.init_receive_packet(deps.as_mut().storage, &mut contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);
    let missing_receipts = contract
        .ibc_store()
        .get_missing_packet_receipts(
            deps.as_ref().storage,
            &IbcPortId::from_str(&packet.destination_port).unwrap(),
            &IbcChannelId::from_str(&packet.destination_channel).unwrap(),
            0,
            10,
        )
        .unwrap();

    assert!(!missing_receipts.contains(&packet.sequence));

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
}

#[should_panic(
    expected = "IbcChannelError { error: InvalidChannelState { channel_id: ChannelId(\"channel-3\"), state: Closed } }"
)]
#[test]
fn test_receive_packet_fails_on_channel_closed() {
    let mut contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);

    let msg = get_dummy_raw_msg_recv_packet(12);

    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);

    let mut chan_end_on_b = test_context.channel_end();
    chan_end_on_b.set_state(State::Closed);
    test_context.channel_end = Some(chan_end_on_b);

    test_context.init_receive_packet(deps.as_mut().storage, &mut contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);

    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
}

#[should_panic(
    expected = "IbcPacketError { error: ConnectionNotOpen { connection_id: ConnectionId(\"connection-0\") } }"
)]
#[test]
fn test_receive_packet_fails_on_invalid_connection_state() {
    let mut contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);

    let msg = get_dummy_raw_msg_recv_packet(12);
    let _packet = msg.packet.clone().unwrap();

    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);

    let mut conn_end_on_b = get_dummy_connection();
    conn_end_on_b.set_state(ConnectionState::Init);
    test_context.connection_end = Some(conn_end_on_b.clone());

    test_context.init_receive_packet(deps.as_mut().storage, &mut contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);

    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
}

#[should_panic(expected = "CallAlreadyInProgress")]
#[test]
fn test_receive_packet_fails_on_packet_already_being_received() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);

    let msg = get_dummy_raw_msg_recv_packet(12);
    let packet = msg.packet.clone().unwrap();

    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);

    test_context.init_receive_packet(deps.as_mut().storage, &contract);

    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
            &packet,
        )
        .unwrap();

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);

    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
}

#[should_panic(
    expected = "IbcPacketError { error: FrozenClient { client_id: ClientId(\"default-0\") } }"
)]
#[test]
fn test_receive_packet_fails_on_frozen_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);

    let msg = get_dummy_raw_msg_recv_packet(12);

    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);

    let mut client_state: ClientState = get_dummy_client_state();
    client_state.frozen_height = 1000;
    test_context.client_state = Some(client_state);

    test_context.init_receive_packet(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);

    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
}

#[should_panic(
    expected = "IbcPacketError { error: InvalidPacketCounterparty { port_id: PortId(\"our-port\"), channel_id: ChannelId(\"channel-1\") } }"
)]
#[test]
fn test_receive_packet_fails_on_invalid_counterparty() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);
    let msg = get_dummy_raw_msg_recv_packet(12);

    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);
    let mut chan_end_on_b = test_context.channel_end();
    chan_end_on_b.set_counterparty_channel_id(to_ibc_channel_id("invalidchannel").unwrap());
    test_context.channel_end = Some(chan_end_on_b);
    test_context.init_receive_packet(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);

    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
}

#[should_panic(expected = "IbcPacketError { error: Other(\"Already Received\") }")]
#[test]
fn test_receive_packet_no_op_on_packet_already_received() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);

    let msg = get_dummy_raw_msg_recv_packet(12);
    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);
    let packet = msg.packet.clone().unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();
    test_context.init_receive_packet(deps.as_mut().storage, &contract);

    contract
        .store_packet_receipt(
            deps.as_mut().storage,
            &dst_port,
            &dst_channel,
            packet.sequence.into(),
            Receipt::Ok,
        )
        .unwrap();

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let _res = contract
        .validate_receive_packet(deps.as_mut(), info, env, &msg)
        .unwrap();
}

#[test]
fn test_is_packet_already_received_for_none_ordered_channel() {
    let msg = get_dummy_raw_msg_recv_packet(10);
    let mut ctx = TestContext::for_receive_packet(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();

    ctx.init_receive_packet(deps.as_mut().storage, &contract);
    if let Some(channel_end) = &mut ctx.channel_end {
        channel_end.ordering = Order::None;
    }

    let packet = msg.packet.clone().unwrap();
    let des_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let des_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let res = contract
        .is_packet_already_received(
            deps.as_ref(),
            &ctx.channel_end(),
            &des_port,
            &des_channel,
            msg.packet.unwrap().sequence.into(),
        )
        .unwrap();

    assert!(!res)
}

#[test]
#[should_panic(
    expected = "InvalidPacketSequence { given_sequence: Sequence(1), next_sequence: Sequence(0) }"
)]
fn test_is_packet_already_received_fail_for_invalid_packet_sequence() {
    let msg = get_dummy_raw_msg_recv_packet(10);
    let mut ctx = TestContext::for_receive_packet(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();

    if let Some(channel_end) = &mut ctx.channel_end {
        channel_end.ordering = Order::Ordered;
    }

    let packet = msg.packet.clone().unwrap();
    let des_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let des_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    contract
        .store_next_sequence_recv(
            deps.as_mut().storage,
            &des_port,
            &des_channel,
            &Sequence::from(0),
        )
        .unwrap();

    contract
        .is_packet_already_received(
            deps.as_ref(),
            &ctx.channel_end(),
            &des_port,
            &des_channel,
            msg.packet.unwrap().sequence.into(),
        )
        .unwrap();
}

#[test]
fn test_is_packet_already_received() {
    let msg = get_dummy_raw_msg_recv_packet(10);
    let mut ctx = TestContext::for_receive_packet(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();

    if let Some(channel_end) = &mut ctx.channel_end {
        channel_end.ordering = Order::Ordered;
    }

    let packet = msg.packet.clone().unwrap();
    let des_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let des_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    contract
        .store_next_sequence_recv(
            deps.as_mut().storage,
            &des_port,
            &des_channel,
            &Sequence::from(1),
        )
        .unwrap();

    let res = contract
        .is_packet_already_received(
            deps.as_ref(),
            &ctx.channel_end(),
            &des_port,
            &des_channel,
            msg.packet.unwrap().sequence.into(),
        )
        .unwrap();

    assert!(!res)
}

#[test]
#[should_panic(expected = "{ error: AcknowledgementExists { sequence: Sequence(1) } }")]
fn fail_test_validate_write_acknowledgement() {
    let msg = get_dummy_raw_msg_recv_packet(10);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();

    let packet = msg.packet.unwrap();
    let sequence = packet.sequence.into();
    let dest_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dest_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();
    let ack_commitment = AcknowledgementCommitment::from(to_vec("0").unwrap());

    contract
        .store_packet_acknowledgement(
            deps.as_mut().storage,
            &dest_port,
            &dest_channel,
            sequence,
            ack_commitment,
        )
        .unwrap();

    contract
        .validate_write_acknowledgement(deps.as_mut().storage, &packet)
        .unwrap();
}

#[test]
fn execute_receive_packet() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();

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

    let result = SubMsgResponse {
        data: None,
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply = Reply { id: 0, result };

    let mut chan_end_on_b =
        get_dummy_channel_end(&IbcPortId::from_str(&packet.src.port_id).unwrap());
    chan_end_on_b
        .set_counterparty_channel_id(IbcChannelId::from_str(&packet.src.channel_id).unwrap());

    contract
        .store_channel_end(
            &mut deps.storage,
            &IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            &IbcChannelId::from_str(&packet.dest.channel_id).unwrap(),
            &chan_end_on_b,
        )
        .unwrap();

    let res = contract.execute_receive_packet(deps.as_mut(), reply);
    assert!(res.is_ok());
    let store = contract.get_callback_data::<IbcPacket>(
        deps.as_ref().storage,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
    );
    assert!(store.is_err())
}

#[test]
fn execute_receive_packet_ordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 10,
    };
    let timeout = IbcTimeout::with_both(timeout_block, cosmwasm_std::Timestamp::from_nanos(100));
    let (src, dst) = get_dummy_endpoints();

    let packet = IbcPacket::new(vec![0, 1, 2, 3], src, dst, 1, timeout);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
            &packet,
        )
        .unwrap();

    let result = SubMsgResponse {
        data: None,
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply = Reply { id: 0, result };

    let mut channel_end =
        get_dummy_channel_end(&IbcPortId::from_str(&packet.dest.port_id).unwrap());
    channel_end
        .set_counterparty_channel_id(IbcChannelId::from_str(&packet.dest.channel_id).unwrap());

    contract
        .store_channel_end(
            &mut deps.storage,
            &IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            &IbcChannelId::from_str(&packet.dest.channel_id).unwrap(),
            &channel_end,
        )
        .unwrap();
    contract
        .store_next_sequence_recv(
            &mut deps.storage,
            &IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            &IbcChannelId::from_str(&packet.dest.channel_id).unwrap(),
            &Sequence::from(1),
        )
        .unwrap();

    let res = contract.execute_receive_packet(deps.as_mut(), reply);

    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_receive_packet_fail_missing_channel() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_recv_packet(12);
    let env = get_mock_env();

    contract
        .validate_receive_packet(deps.as_mut(), info, env, &msg)
        .unwrap();
}

#[test]
fn test_lookup_module_packet() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
    let msg = get_dummy_raw_msg_recv_packet(12);
    let port_id = to_ibc_port_id(&msg.packet.unwrap().source_port).unwrap();
    ctx.claim_capability(
        &mut deps.storage,
        port_id.as_bytes().to_vec(),
        module_id.to_string(),
    )
    .unwrap();
    let res = ctx.lookup_modules(&mut deps.storage, port_id.to_string().as_bytes().to_vec());

    assert!(res.is_ok());
    assert_eq!("contractaddress", res.unwrap())
}

#[test]
fn test_timeout_height_to_str() {
    let contract = CwIbcCoreContext::new();

    let timeout_height = TimeoutHeight::from(mock_height(1, 1).unwrap());
    let timeout = to_ibc_timeout_block(&timeout_height);

    let res = contract.timeout_height_to_str(timeout);
    assert_eq!(res, "1-1".to_string())
}
