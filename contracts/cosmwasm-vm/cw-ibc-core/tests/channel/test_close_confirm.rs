use std::collections::HashMap;

use common::ibc::core::{
    ics03_connection::connection::Counterparty as ConnectionCounterparty,
    ics23_commitment::commitment::CommitmentPrefix,
};
use cosmwasm_std::IbcChannel;
use cw_ibc_core::{
    conversions::to_ibc_channel_id,
    ics04_channel::{
        channel_close_confirm_validate, on_chan_close_confirm_submessage,
        EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE,
    },
};

use super::*;

#[test]
#[should_panic(expected = "UndefinedConnectionCounterparty")]
fn test_validate_close_confirm_channel_fail_missing_counterparty() {
    let mut deps = deps();
    let env = mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_confirm(10);

    let mut test_context = TestContext::for_channel_close_confirm(env, &msg);
    let mut connection_end = test_context.connection_end();
    connection_end.set_counterparty(ConnectionCounterparty::new(
        IbcClientId::default(),
        None,
        CommitmentPrefix::try_from("hello".as_bytes().to_vec()).unwrap(),
    ));
    test_context.connection_end = Some(connection_end);
    test_context.init_channel_close_confirm(deps.as_mut().storage, &contract);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    contract
        .validate_channel_close_confirm(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcChannelError { error: ChannelClosed { channel_id: ChannelId(\"channel-0\") } }"
)]
fn test_validate_close_confirm_channel_fail_channel_closed() {
    let mut deps = deps();
    let env = mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_confirm(10);
    let mut test_context = TestContext::for_channel_close_confirm(env, &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.state = State::Closed;
    test_context.channel_end = Some(channel_end);

    test_context.init_channel_close_confirm(deps.as_mut().storage, &contract);

    contract
        .validate_channel_close_confirm(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "IbcConnectionError { error: ConnectionMismatch { connection_id: ConnectionId(\"connection-0\") } }"
)]
fn test_validate_close_confirm_channel_fail_connection_state_invalid() {
    let mut deps = deps();
    let env = mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_confirm(10);
    let mut test_context = TestContext::for_channel_close_confirm(env, &msg);
    let mut connection_end = test_context.connection_end();
    connection_end.state = ConnectionState::Init;
    test_context.connection_end = Some(connection_end);
    test_context.init_channel_close_confirm(deps.as_mut().storage, &contract);

    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    contract
        .store_client_implementations(
            &mut deps.storage,
            &IbcClientId::default(),
            LightClient::new("lightclient".to_string()),
        )
        .unwrap();
    let mut query_map = HashMap::<Binary, Binary>::new();
    query_map = mock_consensus_state_query(
        query_map,
        &IbcClientId::default(),
        &test_context.consensus_state.unwrap(),
        test_context.height.revision_height(),
    );
    query_map = mock_client_state_query(
        query_map,
        &IbcClientId::default(),
        &test_context.client_state.unwrap(),
    );
    mock_lightclient_query(query_map, &mut deps);

    contract
        .validate_channel_close_confirm(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_validate_close_confirm_channel() {
    let mut deps = deps();
    let env = mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 200000000);
    let msg = get_dummy_raw_msg_chan_close_confirm(10);
    let mut test_context = TestContext::for_channel_close_confirm(env, &msg);
    contract
        .init_channel_counter(deps.as_mut().storage, u64::default())
        .unwrap();
    test_context.init_channel_close_confirm(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let res = contract.validate_channel_close_confirm(deps.as_mut(), info, &msg);
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE
    )
}

#[test]
pub fn test_channel_close_confirm_validate() {
    let msg = get_dummy_raw_msg_chan_close_confirm(10);

    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    let res = channel_close_confirm_validate(&channel_id, &channel_end);

    assert!(res.is_ok())
}
#[test]
pub fn test_on_chan_close_confirm_submessage() {
    let msg = get_dummy_raw_msg_chan_close_confirm(10);

    let conn_id = ConnectionId::new(5);
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
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
    let res = on_chan_close_confirm_submessage(&channel_end, &port_id, &channel_id);
    let expected = cosmwasm_std::IbcChannelCloseMsg::CloseConfirm {
        channel: IbcChannel::new(
            endpoint,
            counter_party,
            cosmwasm_std::IbcOrder::Unordered,
            "xcall".to_string(),
            conn_id.to_string(),
        ),
    };

    assert_eq!(res.unwrap(), expected);
}

#[test]
#[should_panic(expected = "ChannelClosed")]
pub fn test_channel_close_confirm_validate_fail() {
    let msg = get_dummy_raw_msg_chan_close_confirm(10);

    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Closed,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    channel_close_confirm_validate(&channel_id, &channel_end).unwrap();
}

#[test]
#[should_panic(expected = "InvalidConnectionHopsLength")]
pub fn test_channel_close_confirm_validate_fail_connection_hops() {
    let msg = get_dummy_raw_msg_chan_close_confirm(10);
    //  let msg = MsgChannelCloseConfirm::try_from(raw).unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![],
        version: Version::new("xcall".to_string()),
    };
    channel_close_confirm_validate(&channel_id, &channel_end).unwrap();
}

#[test]
#[should_panic(expected = "FrozenClient")]
fn fail_test_validate_channel_close_confirm_for_frozen_client() {
    let msg = get_dummy_raw_msg_chan_close_confirm(10);
    let mut ctx = TestContext::for_channel_close_confirm(get_mock_env(), &msg);
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    if let Some(client_state) = &mut ctx.client_state {
        client_state.frozen_height = 1
    }

    ctx.init_channel_close_confirm(deps.as_mut().storage, &contract);
    mock_lightclient_query(ctx.mock_queries, &mut deps);

    contract
        .validate_channel_close_confirm(deps.as_mut(), info, &msg)
        .unwrap();
}
