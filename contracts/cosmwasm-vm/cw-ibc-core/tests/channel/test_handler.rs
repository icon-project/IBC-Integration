use super::*;

use common::ibc::core::ics04_channel::channel::Order;
use common::ibc::core::ics24_host::identifier::PortId;

use cw_ibc_core::ics03_connection::State as ConnectionState;
use cw_ibc_core::ics04_channel::{open_init, open_try};
use cw_ibc_core::{context::CwIbcCoreContext, ConnectionEnd};

#[test]
#[should_panic(expected = "UndefinedConnectionCounterparty")]
fn test_validate_open_try_channel_fail_missing_counterparty() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut test_context = TestContext::for_channel_open_try(env, &raw);
    let mut connection_end = test_context.connection_end();
    let mut counter_party = connection_end.counterparty().clone();
    counter_party.connection_id = None;
    connection_end.set_counterparty(counter_party);
    test_context.connection_end = Some(connection_end);
    test_context.init_channel_open_try(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);
    contract
        .validate_channel_open_try(deps.as_mut(), info, &raw)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidConnectionHopsLength")]
fn fail_test_validate_open_init_channel_on_invalid_connection_hops_length() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    let mut raw_channel = get_dummy_raw_channel_end(Some(0));
    raw_channel.connection_hops = Vec::new();

    let msg = RawMsgChannelOpenInit {
        port_id: PortId::default().to_string(),
        channel: Some(raw_channel),
        signer: get_dummy_bech32_account(),
    };
    contract
        .validate_channel_open_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidVersionLengthConnection")]
fn fail_channel_open_init_msg_validate_on_invalid_version_length_of_connection() {
    let ctx = TestContext::default(get_mock_env());

    let conn_end = ConnectionEnd::new(
        ctx.connection_end().state,
        ctx.client_id.clone(),
        ctx.connection_end().counterparty().clone(),
        Vec::new(),
        ctx.connection_end().delay_period(),
    );
    open_init::channel_open_init_msg_validate(&ctx.channel_end(), conn_end).unwrap();
}

#[test]
#[should_panic(expected = "ChannelFeatureNotSupportedByConnection")]
fn fail_channel_open_init_msg_validate_on_channel_feature_not_supported() {
    let mut ctx = TestContext::default(get_mock_env());
    if let Some(chann_end) = &mut ctx.channel_end {
        chann_end.ordering = Order::None
    }

    open_init::channel_open_init_msg_validate(&ctx.channel_end(), ctx.connection_end()).unwrap();
}

#[test]
#[should_panic(expected = "ConnectionNotOpen")]
fn fail_channel_open_try_msg_validate_on_invalid_connection_state() {
    let mut ctx = TestContext::default(get_mock_env());
    if let Some(conn_end) = &mut ctx.connection_end {
        conn_end.state = ConnectionState::Init;
    }

    open_try::channel_open_try_msg_validate(&ctx.channel_end(), &ctx.connection_end()).unwrap();
}

#[test]
#[should_panic(expected = "ChannelFeatureNotSupportedByConnection")]
fn fail_channel_open_try_msg_validate_on_feature_not_supported() {
    let mut ctx = TestContext::default(get_mock_env());
    if let Some(chann_end) = &mut ctx.channel_end {
        chann_end.ordering = Order::None
    }

    open_try::channel_open_try_msg_validate(&ctx.channel_end(), &ctx.connection_end()).unwrap();
}

#[test]
#[should_panic(expected = "ClientFrozen")]
fn fail_test_validate_open_init_channel_for_frozen_client() {
    let mut ctx = TestContext::default(get_mock_env());
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 1
    }

    ctx.init_channel_open_init(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    let msg = get_dummy_raw_msg_chan_open_init(Some(0));
    contract
        .validate_channel_open_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidConnectionHopsLength")]
fn fail_test_validate_open_try_channel_on_invalid_connection_hops_length() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    let mut raw_channel = get_dummy_raw_channel_end(Some(0));
    raw_channel.connection_hops = Vec::new();

    let mut msg = get_dummy_raw_msg_chan_open_try(0);
    msg.channel = Some(raw_channel);

    contract
        .validate_channel_open_try(deps.as_mut(), info, &msg)
        .unwrap();
}
