use super::*;
use cosmwasm_std::IbcChannel;
use cw_ibc_core::ics04_channel::open_ack;

use cw_ibc_core::{
    conversions::to_ibc_channel_id,
    ics04_channel::{
        open_ack::{channel_open_ack_validate, on_chan_open_ack_submessage},
        open_try::channel_open_try_msg_validate,
        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
    },
};

#[test]
#[should_panic(expected = "UndefinedConnectionCounterparty")]
fn test_validate_open_ack_channel_fail_missing_counterparty() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_open_ack(10);
    let mut test_context = TestContext::for_channel_open_ack(env, &msg);
    // channel should be init
    let mut channel_end = test_context.channel_end();
    channel_end.set_state(State::Init);
    test_context.channel_end = Some(channel_end);
    // connection setup
    let mut conn_end = test_context.connection_end();
    let mut conn_counter_party = conn_end.counterparty().clone();
    conn_counter_party.connection_id = None;
    conn_end.set_counterparty(conn_counter_party);
    test_context.connection_end = Some(conn_end);
    test_context.init_channel_open_ack(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    contract
        .validate_channel_open_ack(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_validate_open_ack_channel() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let msg = get_dummy_raw_msg_chan_open_ack(10);

    let mut test_context = TestContext::for_channel_open_ack(env, &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.set_state(State::Init);
    test_context.channel_end = Some(channel_end);
    test_context.init_channel_open_ack(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let res = contract.validate_channel_open_ack(deps.as_mut(), info, &msg);
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE
    )
}

#[test]
#[should_panic(expected = "InvalidChannelState")]
fn test_open_ack_channel_fail_invalid_state() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let msg = get_dummy_raw_msg_chan_open_ack(10);

    let mut test_context = TestContext::for_channel_open_ack(env, &msg);
    // channel should be init
    let mut channel_end = test_context.channel_end();
    channel_end.set_state(State::Open);
    test_context.channel_end = Some(channel_end);
    test_context.init_channel_open_ack(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    contract
        .validate_channel_open_ack(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_channel_open_ack_validate() {
    let msg = get_dummy_raw_msg_chan_open_ack(10);

    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(to_ibc_channel_id(&msg.counterparty_channel_id).unwrap()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    let res = channel_open_ack_validate(&channel_id, &channel_end);

    assert!(res.is_ok())
}

#[test]
#[should_panic(expected = "InvalidChannelState")]
fn test_channel_open_ack_validate_fail() {
    let msg = get_dummy_raw_msg_chan_open_ack(10);

    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::TryOpen,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(to_ibc_channel_id(&msg.counterparty_channel_id).unwrap()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    channel_open_ack_validate(&channel_id, &channel_end).unwrap();
}

#[test]
pub fn test_on_chan_open_ack_submessage() {
    let msg = get_dummy_raw_msg_chan_close_confirm(10);

    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![conn_id.clone()],
        version: Version::new("xcall".to_string()),
    };
    let endpoint = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let counter_party = cosmwasm_std::IbcEndpoint {
        port_id: channel_end.remote.port_id.to_string(),
        channel_id: channel_end.clone().remote.channel_id.unwrap().to_string(),
    };
    let res = on_chan_open_ack_submessage(&channel_end, &port_id, &channel_id, &conn_id);
    let expected = cosmwasm_std::IbcChannelConnectMsg::OpenAck {
        channel: IbcChannel::new(
            endpoint,
            counter_party,
            cosmwasm_std::IbcOrder::Unordered,
            "xcall".to_string(),
            conn_id.to_string(),
        ),
        counterparty_version: channel_end.version().to_string(),
    };

    assert_eq!(res.unwrap(), expected);
}

#[test]
#[should_panic(expected = "InvalidVersionLengthConnection")]
fn test_channel_open_try_validate_fail_invalid_connection_lenght() {
    let raw = get_dummy_raw_msg_chan_open_try(10);

    let channel = to_ibc_channel(raw.channel).unwrap();
    let mut connection_end = ConnectionEnd::default();
    connection_end.set_state(common::ibc::core::ics03_connection::connection::State::Open);
    channel_open_try_msg_validate(&channel, &connection_end).unwrap();
}

#[test]
fn test_channel_open_try_validate() {
    let raw = get_dummy_raw_msg_chan_open_try(10);

    let channel = to_ibc_channel(raw.channel).unwrap();
    let mut connection_end = ConnectionEnd::default();
    connection_end.set_state(common::ibc::core::ics03_connection::connection::State::Open);
    connection_end.set_version(common::ibc::core::ics03_connection::version::Version::default());
    let res = channel_open_try_msg_validate(&channel, &connection_end);
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "UnknownOrderType")]
fn fail_channel_open_ack_msg_validate_on_unknown_order() {
    let mut ctx = TestContext::default(get_mock_env());
    if let Some(chann_end) = &mut ctx.channel_end {
        chann_end.remote = common::ibc::core::ics04_channel::channel::Counterparty::default();
        chann_end.ordering = Order::None
    }

    open_ack::on_chan_open_ack_submessage(
        &ctx.channel_end(),
        &ctx.port_id,
        &ctx.channel_id,
        &ctx.connection_id,
    )
    .unwrap();
}

#[test]
fn test_channel_open_ack_msg_validate_on_ordered_type() {
    let mut ctx = TestContext::default(get_mock_env());
    if let Some(chann_end) = &mut ctx.channel_end {
        chann_end.remote = common::ibc::core::ics04_channel::channel::Counterparty::default();
        chann_end.ordering = Order::Ordered
    }

    let res = Some(
        open_ack::on_chan_open_ack_submessage(
            &ctx.channel_end(),
            &ctx.port_id,
            &ctx.channel_id,
            &ctx.connection_id,
        )
        .unwrap(),
    );

    let res_exist = res.is_some();
    assert!(res_exist);
}

#[test]
#[should_panic(expected = "FrozenClient")]
fn fail_test_validate_open_try_channel_for_frozen_client() {
    let msg = get_dummy_raw_msg_chan_open_try(10);
    let mut ctx = TestContext::for_channel_open_try(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 1
    }

    ctx.init_channel_open_try(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .validate_channel_open_try(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "FrozenClient")]
fn fail_test_validate_channel_open_ack_for_frozen_client() {
    let msg = get_dummy_raw_msg_chan_open_ack(10);
    let mut ctx = TestContext::for_channel_open_ack(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 1
    }

    ctx.init_channel_open_ack(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .validate_channel_open_ack(deps.as_mut(), info, &msg)
        .unwrap();
}
