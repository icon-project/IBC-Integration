use common::ibc::core::ics04_channel::commitment::PacketCommitment;
use cw_ibc_core::{
    conversions::{to_ibc_channel_id, to_ibc_timeout_height, to_ibc_timestamp},
    light_client::light_client::LightClient,
    VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
};

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

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.timeout_packet_validate_to_light_client(deps.as_mut(), info, env, msg);
    println!("{res:?}");
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE
    )
}
