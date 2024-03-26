use super::*;

use cosmwasm_std::to_vec;
use cw_ibc_core::conversions::to_ibc_channel_id;

use common::ibc::core::ics04_channel::commitment::AcknowledgementCommitment;

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

    mock_lightclient_query(test_context.mock_queries, &mut deps);
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

    mock_lightclient_query(test_context.mock_queries, &mut deps);

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

#[test]
#[should_panic(expected = "ChannelClosed")]
fn acknowledgement_packet_validate_fail_on_channel_close() {
    let msg = get_dummy_raw_msg_acknowledgement(10);
    let mut ctx = TestContext::for_acknowledge_packet(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    if let Some(chann_end) = &mut ctx.channel_end {
        chann_end.state = State::Closed
    }

    ctx.init_acknowledge_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, ctx.env, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidPacketCounterparty")]
fn acknowledgement_packet_validate_fail_on_invalid_packet_counterparty() {
    let mut msg = get_dummy_raw_msg_acknowledgement(10);
    let mut ctx = TestContext::for_acknowledge_packet(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    ctx.init_acknowledge_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    if let Some(packet) = &mut msg.packet {
        packet.destination_port = "different_port".to_string();
    }

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, ctx.env, &msg)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcPacketError { error: IncorrectPacketCommitment { sequence: Sequence(1) } }"
)]
fn acknowledgement_packet_validate_fail_for_incorrect_packet_commitment() {
    let mut msg = get_dummy_raw_msg_acknowledgement(10);
    let mut ctx = TestContext::for_acknowledge_packet(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    ctx.init_acknowledge_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    if let Some(packet) = &mut msg.packet {
        packet.timeout_height = Some(RawHeight {
            revision_height: 100,
            revision_number: 100,
        });
    }

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, ctx.env, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidPacketSequence")]
fn acknowledgement_packet_validate_fail_for_invalid_packet_sequence() {
    let msg = get_dummy_raw_msg_acknowledgement(10);
    let mut ctx = TestContext::for_acknowledge_packet(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    if let Some(chann_end) = &mut ctx.channel_end {
        chann_end.ordering = Order::Ordered
    }

    ctx.init_acknowledge_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let packet = msg.clone().packet.unwrap();
    let dest_port = to_ibc_port_id(&packet.source_port).unwrap();
    let dest_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    contract
        .store_next_sequence_ack(
            deps.as_mut().storage,
            &dest_port,
            &dest_channel,
            &Sequence::from_str("2").unwrap(),
        )
        .unwrap();

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, ctx.env, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "FrozenClient")]
fn acknowledgement_packet_validate_fail_for_frozen_client() {
    let msg = get_dummy_raw_msg_acknowledgement(10);
    let mut ctx = TestContext::for_acknowledge_packet(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 10
    }

    ctx.init_acknowledge_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, ctx.env, &msg)
        .unwrap();
}

#[test]
fn test_get_packet_acknowledgement() {
    let ctx = TestContext::default(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::new();

    let sequence = Sequence::from_str("0").unwrap();
    let ack_commitment = AcknowledgementCommitment::from(to_vec("ack").unwrap());

    contract
        .store_packet_acknowledgement(
            deps.as_mut().storage,
            &ctx.port_id,
            &ctx.channel_id,
            sequence,
            ack_commitment.clone(),
        )
        .unwrap();

    let res = contract
        .get_packet_acknowledgement(
            deps.as_ref().storage,
            &ctx.port_id,
            &ctx.channel_id,
            sequence,
        )
        .unwrap();

    assert_eq!(ack_commitment, res);
}
