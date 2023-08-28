use std::collections::HashMap;

use common::icon::tendermint::light::LightHeader;
use cw_ibc_core::{
    conversions::{to_ibc_channel_id, to_ibc_packet, to_ibc_timeout_height, to_ibc_timestamp},
    light_client::light_client::LightClient,
    VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
};
use cw_ibc_core::conversions::to_ibc_channel_id;

use super::*;

#[test]
fn test_acknowledgement_packet_validate_ordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let env = get_mock_env();

    let height = 10;
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let mut test_context = TestContext::for_acknowledge_packet(env.clone(), &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.ordering = Order::Ordered;
    test_context.channel_end = Some(channel_end);

    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();
    test_context.init_acknowledge_packet(deps.as_mut().storage, &contract);
    contract
        .store_next_sequence_ack(&mut deps.storage, &src_port, &src_channel, &1.into())
        .unwrap();

    mock_lightclient_reply(&mut deps);
    let res = contract.acknowledgement_packet_validate(deps.as_mut(), info, env, &msg);
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[test]
fn test_acknowledgement_packet_validate_unordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let mut env = get_mock_env();
    env.block.height = 100;

    let height = 10;
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let mut test_context = TestContext::for_acknowledge_packet(env.clone(), &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.ordering = Order::Unordered;
    test_context.channel_end = Some(channel_end);
    test_context.init_acknowledge_packet(deps.as_mut().storage, &contract);

    mock_lightclient_reply(&mut deps);

    let res = contract.acknowledgement_packet_validate(deps.as_mut(), info, env, &msg);
    assert!(res.is_ok());
}

#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"PacketCommitmentNotFound\", stack: [] } }"
)]
#[test]
fn test_acknowledgement_packet_validate_without_commitment() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let env = get_mock_env();
    let height = 10;
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let mut test_context = TestContext::for_acknowledge_packet(env.clone(), &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.ordering = Order::Unordered;
    test_context.channel_end = Some(channel_end);
    test_context.packet = None;
    test_context.init_acknowledge_packet(deps.as_mut().storage, &contract);

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, env, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_acknowledgement_packet_validate_fail_missing_channel() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let height = 10;
    let env = get_mock_env();
    let msg = get_dummy_raw_msg_acknowledgement(height);

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, env, &msg)
        .unwrap();
}
