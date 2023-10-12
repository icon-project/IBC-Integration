use cosmwasm_std::IbcChannel;

use cw_ibc_core::{
    conversions::to_ibc_channel_id,
    ics04_channel::{
        open_confirm::{channel_open_confirm_validate, on_chan_open_confirm_submessage},
        EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE,
    },
};

use super::*;

#[test]
#[should_panic(expected = "UndefinedConnectionCounterparty")]
fn test_validate_open_confirm_channel_fail_missing_counterparty() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_open_confirm(10);
    let mut test_context = TestContext::for_channel_open_confirm(env, &msg);
    let mut conn_end = test_context.connection_end();
    let mut counter_party = conn_end.counterparty().clone();
    counter_party.connection_id = None;
    conn_end.set_counterparty(counter_party);
    test_context.connection_end = Some(conn_end);

    test_context.init_channel_open_confirm(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    contract
        .validate_channel_open_confirm(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_validate_open_confirm_channel() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let msg = get_dummy_raw_msg_chan_open_confirm(10);
    let mut test_context = TestContext::for_channel_open_confirm(env, &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.state = State::TryOpen;
    test_context.channel_end = Some(channel_end);
    test_context.init_channel_open_confirm(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_channel_open_confirm(deps.as_mut(), info, &msg);
    println!("{res:?}");
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE
    )
}

#[test]
#[should_panic(
    expected = "IbcChannelError { error: InvalidChannelState { channel_id: ChannelId(\"channel-0\"), state: Open } }"
)]
fn test_execute_open_confirm_channel_fail_invalid_state() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let msg = get_dummy_raw_msg_chan_open_confirm(10);
    let mut test_context = TestContext::for_channel_open_confirm(env, &msg);
    let mut channel_end = test_context.channel_end();
    channel_end.state = State::Open;
    test_context.channel_end = Some(channel_end);
    test_context.init_channel_open_confirm(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let _res = contract
        .validate_channel_open_confirm(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
pub fn test_channel_open_confirm_validate() {
    let msg = get_dummy_raw_msg_chan_open_confirm(10);

    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::TryOpen,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![conn_id],
        version: Version::new("xcall".to_string()),
    };
    let res = channel_open_confirm_validate(&channel_id, &channel_end);

    assert!(res.is_ok())
}
#[test]
pub fn test_on_chan_open_confirm_submessage() {
    let msg = get_dummy_raw_msg_chan_open_confirm(10);

    let conn_id = ConnectionId::new(5);
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::TryOpen,
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
    let res = on_chan_open_confirm_submessage(&channel_end, &port_id, &channel_id);
    let expected = cosmwasm_std::IbcChannelConnectMsg::OpenConfirm {
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
