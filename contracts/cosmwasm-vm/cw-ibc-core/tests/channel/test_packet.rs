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
    let test_context = TestContext::for_send_packet(env, &packet);
    test_context.init_send_packet(deps.as_mut().storage, &contract);
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();
    let res = contract.send_packet(deps.as_mut(), &mock_env(), packet);

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
    contract
        .send_packet(deps.as_mut(), &mock_env(), packet)
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

    let test_context = TestContext::for_send_packet(env, &packet);
    test_context.init_send_packet(deps.as_mut().storage, &contract);
    packet.sequence = 10;

    contract
        .send_packet(deps.as_mut(), &mock_env(), packet)
        .unwrap();
}
