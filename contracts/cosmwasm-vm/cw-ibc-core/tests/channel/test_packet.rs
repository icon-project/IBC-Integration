use std::collections::HashMap;

use cw_ibc_core::conversions::{to_ibc_channel_id, to_ibc_port_id};

use super::*;

#[test]
fn test_packet_send() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 100;

    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());

    packet.data = vec![0];
    let mut test_context = TestContext::for_send_packet(env, &packet);
    test_context.init_send_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();
    let info = create_mock_info("moduleaddress", "test", 100);
    let res = contract.send_packet(deps.as_mut(), &mock_env(), info, packet);
    println!("{res:?}");
    assert!(res.is_ok());
    let res = res.unwrap();
    assert_eq!(res.attributes[0].value, "send_packet");
    assert_eq!(res.events[0].ty, IbcEventType::SendPacket.as_str());

    let packet_heights = contract
        .ibc_store()
        .get_packet_heights(deps.as_ref().storage, &src_port, &src_channel, 0, 10)
        .unwrap();
    println!("{packet_heights:?}");
    let height = packet_heights.get(&1).cloned().unwrap();
    assert_eq!(height, 12345);
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_packet_send_fail_channel_not_found() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());

    packet.sequence = 1;
    packet.data = vec![0];
    let info = create_mock_info("moduleaddress", "test", 100);
    contract
        .bind_port(
            &mut deps.storage,
            &IbcPortId::from_str(&packet.source_port).unwrap(),
            Addr::unchecked("moduleaddress").to_string(),
        )
        .unwrap();
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcPacketError { error: InvalidPacketSequence { given_sequence: Sequence(10), next_sequence: Sequence(1) } }"
)]
fn test_packet_send_fail_invalid_sequence() {
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 100;
    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    packet.data = vec![0];

    let mut test_context = TestContext::for_send_packet(env, &packet);
    test_context.init_send_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);
    packet.sequence = 10;
    let info = create_mock_info("moduleaddress", "test", 100);
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}

#[should_panic(
    expected = "IbcPacketError { error: FrozenClient { client_id: ClientId(\"default-0\") } }"
)]
#[test]
fn test_packet_send_fails_on_frozen_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 100;

    let packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());

    let mut test_context = TestContext::for_send_packet(env, &packet);
    let mut client_state = test_context.client_state.unwrap();
    client_state.frozen_height = 1000;
    test_context.client_state = Some(client_state);
    test_context.init_send_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let info = create_mock_info("moduleaddress", "test", 100);
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}

#[should_panic(
    expected = "IbcPacketError { error: InvalidChannelState { channel_id: ChannelId(\"channel-1\"), state: Init } }"
)]
#[test]
fn test_packet_send_fails_on_invalid_client_state() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 100;

    let packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());

    let mut test_context = TestContext::for_send_packet(env, &packet);
    let mut channel_end = test_context.channel_end();
    channel_end.state = State::Init;
    test_context.channel_end = Some(channel_end);
    test_context.init_send_packet(deps.as_mut().storage, &contract);
    let info = create_mock_info("moduleaddress", "test", 100);
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}

#[should_panic(
    expected = "IbcPacketError { error: InvalidPacketCounterparty { port_id: PortId(\"their-port\"), channel_id: ChannelId(\"invalid_channel\") } }"
)]
#[test]
fn test_packet_send_fails_on_invalid_counterparty() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 100;

    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());

    let mut test_context = TestContext::for_send_packet(env, &packet);

    test_context.init_send_packet(deps.as_mut().storage, &contract);
    packet.destination_channel = "invalid_channel".to_string();
    let info = create_mock_info("moduleaddress", "test", 100);
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}

#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"CapabilityNotFound\", stack: [] } }"
)]
#[test]
fn test_packet_send_fails_for_invalid_port() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;

    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    let mut test_context = TestContext::for_send_packet(env, &packet);

    test_context.init_send_packet(deps.as_mut().storage, &contract);
    let info = create_mock_info("moduleaddress", "test", 100);
    packet.source_port = "invalidPort".to_string();
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}

#[should_panic(
    expected = " IbcPacketError { error: LowPacketHeight { chain_height: Height { revision: 0, height: 100 }, timeout_height: At(Height { revision: 0, height: 90 }) } }"
)]
#[test]
fn test_packet_send_fails_on_timedout_height() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 90;
    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    packet.sequence = 1;
    packet.data = vec![0];
    let mut test_context = TestContext::for_send_packet(env, &packet);

    test_context.init_send_packet(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let info = create_mock_info("moduleaddress", "test", 100);
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}

#[should_panic(expected = "IbcPacketError { error: LowPacketTimestamp }")]
#[test]
fn test_packet_send_fails_on_timedout_timestamp() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let timestamp_future = Timestamp::from_nanoseconds(1692668413 * 1000000000).unwrap();
    let timeout_height_future = 110;

    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    packet.sequence = 1;
    packet.data = vec![0];
    let mut test_context = TestContext::for_send_packet(env, &packet);
    let client_state = test_context.client_state.clone().unwrap();

    test_context.init_send_packet(deps.as_mut().storage, &contract);
    let timestamp_query = LightClient::get_timestamp_at_height_query(
        &IbcClientId::default(),
        client_state.latest_height,
    )
    .unwrap();
    let mut mocks = test_context.mock_queries.clone();
    let expiry_future = Timestamp::from_nanoseconds(1692768413 * 1000000000).unwrap();
    mocks.insert(
        timestamp_query,
        to_binary(&(expiry_future.nanoseconds())).unwrap(),
    );
    mock_lightclient_query(mocks, &mut deps);

    let info = create_mock_info("moduleaddress", "test", 100);
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}
