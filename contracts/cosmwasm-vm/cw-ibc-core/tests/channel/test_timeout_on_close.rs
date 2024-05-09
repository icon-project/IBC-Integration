use cw_common::types::TimeoutMsgType;
use cw_ibc_core::VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE;

use super::*;

#[test]
fn test_timeout_on_close_packet_validate_to_light_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let height = 2;
    let timeout_timestamp = 5;
    let msg = get_dummy_raw_msg_timeout_on_close(height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout_on_close(env.clone(), &msg);
    test_context.init_timeout_packet_on_close(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.timeout_packet_validate(
        deps.as_mut(),
        env,
        info,
        TimeoutMsgType::TimeoutOnClose(msg),
    );
    print!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE
    )
}

#[test]
#[should_panic(expected = "InvalidPacketCounterparty")]
fn test_timeout_on_close_packet_validate_to_light_client_fails_on_invalid_channel_counterparty() {
    let height = 2;
    let timeout_timestamp = 5;
    let mut msg = get_dummy_raw_msg_timeout_on_close(height, timeout_timestamp);

    let mut ctx = TestContext::for_packet_timeout_on_close(get_mock_env(), &msg);
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    ctx.init_timeout_packet_on_close(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    if let Some(packet) = &mut msg.packet {
        packet.destination_port = "different_port".to_string();
    }

    contract
        .timeout_on_close_packet_validate_to_light_client(deps.as_mut(), info, ctx.env, msg)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcPacketError { error: IncorrectPacketCommitment { sequence: Sequence(1) } }"
)]
fn test_timeout_on_close_packet_validate_to_light_client_fails_on_incorrect_packet_commitment() {
    let height = 2;
    let timeout_timestamp = 5;
    let mut msg = get_dummy_raw_msg_timeout_on_close(height, timeout_timestamp);

    let mut ctx = TestContext::for_packet_timeout_on_close(get_mock_env(), &msg);
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    ctx.init_timeout_packet_on_close(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    if let Some(packet) = &mut msg.packet {
        packet.timeout_timestamp = 100;
    }

    contract
        .timeout_on_close_packet_validate_to_light_client(deps.as_mut(), info, ctx.env, msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "FrozenClient")]
fn test_timeout_on_close_packet_validate_to_light_client_fails_for_frozen_client() {
    let height = 2;
    let timeout_timestamp = 5;
    let msg = get_dummy_raw_msg_timeout_on_close(height, timeout_timestamp);

    let mut ctx = TestContext::for_packet_timeout_on_close(get_mock_env(), &msg);
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 10;
    }

    ctx.init_timeout_packet_on_close(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .timeout_on_close_packet_validate_to_light_client(deps.as_mut(), info, ctx.env, msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidPacketSequence")]
fn test_timeout_on_close_packet_validate_to_light_client_fails_on_invalid_packet_sequence() {
    let height = 2;
    let timeout_timestamp = 5;
    let mut msg = get_dummy_raw_msg_timeout_on_close(height, timeout_timestamp);
    msg.next_sequence_recv = 2;

    let mut ctx = TestContext::for_packet_timeout_on_close(get_mock_env(), &msg);
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    if let Some(channel_end) = &mut ctx.channel_end {
        channel_end.ordering = Order::Ordered;
    }

    ctx.init_timeout_packet_on_close(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .timeout_on_close_packet_validate_to_light_client(deps.as_mut(), info, ctx.env, msg)
        .unwrap();
}

#[test]
fn test_timeout_on_close_packet_validate_to_light_client_for_orderd_channel() {
    let height = 2;
    let timeout_timestamp = 5;
    let msg = get_dummy_raw_msg_timeout_on_close(height, timeout_timestamp);

    let mut ctx = TestContext::for_packet_timeout_on_close(get_mock_env(), &msg);
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    if let Some(channel_end) = &mut ctx.channel_end {
        channel_end.ordering = Order::Ordered;
    }

    ctx.init_timeout_packet_on_close(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let res = contract.timeout_on_close_packet_validate_to_light_client(
        deps.as_mut(),
        info,
        ctx.env,
        msg,
    );
    assert!(res.is_ok())
}
