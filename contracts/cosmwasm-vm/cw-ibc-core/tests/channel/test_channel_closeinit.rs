use super::*;
use cw_ibc_core::{
    conversions::to_ibc_channel_id,
    ics04_channel::close_init::{channel_close_init_validate, on_chan_close_init_submessage},
};

use std::str::FromStr;

use common::ibc::core::ics04_channel::{
    channel::{Counterparty, Order, State},
    Version,
};
use cosmwasm_std::{to_binary, IbcOrder};

use cw_ibc_core::ics04_channel::open_init::create_channel_submesssage;
use cw_ibc_core::ics04_channel::EXECUTE_ON_CHANNEL_CLOSE_INIT;
use cw_ibc_core::ChannelEnd;

#[test]
fn test_validate_close_init_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();

    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let connection_id = ConnectionId::new(0);

    let mut test_context = TestContext::for_channel_close_init(mock_env(), &msg);
    let channel_end = test_context.channel_end();

    test_context.init_channel_close_init(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_channel_close_init(deps.as_mut(), info.clone(), &msg);
    let expected =
        on_chan_close_init_submessage(&port_id, &channel_id, &channel_end, &connection_id);

    let data = cw_common::ibc_dapp_msg::ExecuteMsg::IbcChannelClose { msg: expected };
    let data = to_binary(&data).unwrap();
    let on_chan_open_init = create_channel_submesssage(
        "moduleaddress".to_string(),
        data,
        info.funds,
        EXECUTE_ON_CHANNEL_CLOSE_INIT,
    );
    println!("{res:?}");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0], on_chan_open_init)
}

#[should_panic(
    expected = "IbcConnectionError { error: ConnectionMismatch { connection_id: ConnectionId(\"connection-0\") } }"
)]
#[test]
fn test_validate_close_init_channel_fails_invalid_connection_state() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();

    let mut test_context = TestContext::for_channel_close_init(mock_env(), &msg);
    let mut connection_end = test_context.connection_end();
    connection_end.state = common::ibc::core::ics03_connection::connection::State::Uninitialized;
    test_context.connection_end = Some(connection_end);

    test_context.init_channel_close_init(deps.as_mut().storage, &contract);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    let mut query_map = HashMap::<Binary, Binary>::new();
    query_map = mock_client_state_query(
        query_map,
        &IbcClientId::default(),
        &get_dummy_client_state(),
    );
    mock_lightclient_query(query_map, &mut deps);
    contract
        .validate_channel_close_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[should_panic(
    expected = "IbcChannelError { error: ChannelClosed { channel_id: ChannelId(\"channel-0\") } }"
)]
#[test]
fn test_validate_close_init_channel_fails_on_closed_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();

    let mut test_context = TestContext::for_channel_close_init(mock_env(), &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.state = State::Closed;
    test_context.channel_end = Some(channel_end);

    test_context.init_channel_close_init(deps.as_mut().storage, &contract);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();

    contract
        .validate_channel_close_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[should_panic(
    expected = "IbcChannelError { error: InvalidConnectionHopsLength { expected: 1, actual: 0 } }"
)]
#[test]
fn test_validate_close_init_channel_fails_on_invalid_connection_hops() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();

    let mut test_context = TestContext::for_channel_close_init(mock_env(), &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.connection_hops = vec![];
    test_context.channel_end = Some(channel_end);

    test_context.init_channel_close_init(deps.as_mut().storage, &contract);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    contract
        .validate_channel_close_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_close_init_channel_fail_missing_connection_end() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();
    let mut test_context = TestContext::for_channel_close_init(mock_env(), &msg);
    test_context.connection_end = None;

    test_context.init_channel_close_init(deps.as_mut().storage, &contract);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();

    contract
        .validate_channel_close_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_channel_close_init_validate() {
    let msg = get_dummy_raw_msg_chan_close_init();

    let connection_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id],
        version: Version::new("xcall".to_string()),
    };
    let channel_close_init_validate = channel_close_init_validate(&channel_id, &channel_end);

    assert!(channel_close_init_validate.is_ok())
}

#[test]
#[should_panic(
    expected = "IbcChannelError { error: ChannelClosed { channel_id: ChannelId(\"channel-0\") } }"
)]
fn test_channel_close_init_validate_fail() {
    let msg = get_dummy_raw_msg_chan_close_init();

    let connection_id = ConnectionId::new(5);
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Closed,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id],
        version: Version::new("xcall".to_string()),
    };
    channel_close_init_validate(&channel_id, &channel_end).unwrap();
}

#[test]
fn test_on_chan_close_init_submessage() {
    let msg = get_dummy_raw_msg_chan_close_init();

    let connection_id = ConnectionId::new(5);
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id.clone()],
        version: Version::new("xcall".to_string()),
    };
    let channel_close_init_validate =
        on_chan_close_init_submessage(&port_id, &channel_id, &channel_end, &connection_id);

    assert_eq!("xcall", channel_close_init_validate.channel().version);
    assert_eq!(
        IbcOrder::Unordered,
        channel_close_init_validate.channel().order
    );
}

#[test]
#[should_panic(expected = "ClientFrozen")]
fn fail_test_validate_chanel_close_init_for_frozen_client() {
    let msg = get_dummy_raw_msg_chan_close_init();
    let mut ctx = TestContext::for_channel_close_init(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 1
    }

    ctx.init_channel_close_init(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .validate_channel_close_init(deps.as_mut(), info, &msg)
        .unwrap();
}
