use cw_common::types::TimeoutMsgType;
use cw_ibc_core::{light_client::light_client::LightClient, VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE};

use super::*;

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_timeout_packet_fails_invalid_channel() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;
    let msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout(env.clone(), &msg);
    test_context.channel_end = None;
    test_context.init_timeout_packet(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    contract
        .timeout_packet_validate_to_light_client(deps.as_mut(), info, env, msg)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcPacketError { error: InvalidPacketCounterparty { port_id: PortId(\"invalidport\"), channel_id: ChannelId(\"invalid_channel\") } }"
)]
fn test_timeout_packet_fails_invalid_counterparty() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;
    let mut msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout(env.clone(), &msg);

    test_context.init_timeout_packet(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let mut packet = msg.packet.clone().unwrap();
    packet.destination_channel = "invalid_channel".to_string();
    packet.destination_port = "invalidport".to_string();
    msg.packet = Some(packet);
    contract
        .timeout_packet_validate_to_light_client(deps.as_mut(), info, env, msg)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcPacketError { error: IncorrectPacketCommitment { sequence: Sequence(1) } }"
)]
fn test_timeout_packet_fails_invalid_packet_commitment() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;

    let mut msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout(env.clone(), &msg);
    test_context.save_timestamp_at_height(proof_height, 0);
    test_context.init_timeout_packet(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let mut packet = msg.packet.clone().unwrap();
    packet.data = vec![1, 2, 3, 4, 5, 6];

    msg.packet = Some(packet);

    contract
        .timeout_packet_validate_to_light_client(deps.as_mut(), info, env, msg)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"PacketCommitmentNotFound\", stack: [] } }"
)]
fn test_timeout_packet_fails_for_invalid_packet() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;
    let mut msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout(env.clone(), &msg);
    test_context.save_timestamp_at_height(proof_height, 0);
    test_context.init_timeout_packet(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let mut packet = msg.packet.clone().unwrap();
    packet.sequence = 100;

    msg.packet = Some(packet);

    contract
        .timeout_packet_validate_to_light_client(deps.as_mut(), info, env, msg)
        .unwrap();
}

#[test]
fn test_timeout_packet_validate_to_light_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;
    let msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout(env.clone(), &msg);
    test_context.init_timeout_packet(deps.as_mut().storage, &contract);
    test_context.save_timestamp_at_height(proof_height, 0);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res =
        contract.timeout_packet_validate(deps.as_mut(), env, info, TimeoutMsgType::Timeout(msg));

    println!("{res:?}");
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE
    )
}

#[should_panic(expected = "PacketNotExpired")]
#[test]
fn test_timeout_packet_fails_if_height_not_expired() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let proof_height = 50;
    let timeout_height = proof_height + 10;
    let timeout_timestamp = 0;
    let msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout(env.clone(), &msg);
    test_context.init_timeout_packet(deps.as_mut().storage, &contract);

    let timestamp_query =
        LightClient::get_timestamp_at_height_query(&IbcClientId::default(), proof_height).unwrap();
    let mut mocks = test_context.mock_queries.clone();
    mocks.insert(timestamp_query, to_binary(&0_u64).unwrap());
    mock_lightclient_query(mocks, &mut deps);

    contract
        .timeout_packet_validate_to_light_client(deps.as_mut(), info, env, msg)
        .unwrap();
}

#[should_panic(expected = "PacketNotExpired")]
#[test]
fn test_timeout_packet_fails_if_timestamp_not_expired() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let proof_height = 50;
    let timeout_height = proof_height + 10;
    let timeout_timestamp = 1692668413 * 1000000000;
    let msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout(env.clone(), &msg);
    test_context.init_timeout_packet(deps.as_mut().storage, &contract);

    let timestamp_query =
        LightClient::get_timestamp_at_height_query(&IbcClientId::default(), proof_height).unwrap();
    let mut mocks = test_context.mock_queries.clone();
    let expiry_future = Timestamp::from_nanoseconds(1691668413 * 1000000000).unwrap();
    mocks.insert(
        timestamp_query,
        to_binary(&expiry_future.nanoseconds()).unwrap(),
    );
    mock_lightclient_query(mocks, &mut deps);

    contract
        .timeout_packet_validate_to_light_client(deps.as_mut(), info, env, msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "ChannelClosed")]
fn test_timeout_packet_fails_for_closed_connection() {
    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;
    let msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);

    let mut ctx = TestContext::for_packet_timeout(get_mock_env(), &msg);
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    if let Some(channel_end) = &mut ctx.channel_end {
        channel_end.state = cw_ibc_core::ics04_channel::State::Closed;
    }

    ctx.init_timeout_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .timeout_packet_validate_to_light_client(deps.as_mut(), info, ctx.env, msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidPacketSequence")]
fn test_timeout_packet_fails_for_invalid_packet_sequence() {
    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;
    let mut msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    msg.next_sequence_recv = 20;

    let mut ctx = TestContext::for_packet_timeout(get_mock_env(), &msg);
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    if let Some(channel_end) = &mut ctx.channel_end {
        channel_end.ordering = channel::Order::Ordered;
    }

    ctx.init_timeout_packet(deps.as_mut().storage, &contract);
    ctx.save_timestamp_at_height(proof_height, 0);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .timeout_packet_validate_to_light_client(deps.as_mut(), info, ctx.env, msg)
        .unwrap();
}

#[test]
fn test_timeout_packet_for_ordered_channel() {
    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;
    let msg = get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);

    let mut ctx = TestContext::for_packet_timeout(get_mock_env(), &msg);
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    if let Some(channel_end) = &mut ctx.channel_end {
        channel_end.ordering = channel::Order::Ordered;
    }

    ctx.init_timeout_packet(deps.as_mut().storage, &contract);
    ctx.save_timestamp_at_height(proof_height, 0);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let res = contract.timeout_packet_validate_to_light_client(deps.as_mut(), info, ctx.env, msg);
    assert!(res.is_ok())
}
