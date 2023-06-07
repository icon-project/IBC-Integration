use super::*;
use cw_ibc_core::ics04_channel::close_init::{
    channel_close_init_validate, on_chan_close_init_submessage,
};

use std::{str::FromStr, time::Duration};

use common::ibc::core::ics04_channel::{
    channel::{Counterparty, Order, State},
    Version,
};
use cosmwasm_std::{to_binary, Addr, Event, IbcOrder, Reply, SubMsgResponse, SubMsgResult};
use cw_common::ibc_types::IbcClientId;

use cw_ibc_core::ics04_channel::open_init::create_channel_submesssage;
use cw_ibc_core::ics04_channel::EXECUTE_ON_CHANNEL_CLOSE_INIT;
use cw_ibc_core::{
    context::CwIbcCoreContext, ics04_channel::MsgChannelCloseInit, ChannelEnd, ConnectionEnd,
};

#[test]
fn test_validate_close_init_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();
    let module = Addr::unchecked("contractaddress");
    // contract
    //     .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
    //     .unwrap();

    // let cx_module_id = module_id;
    // contract
    //     .add_route(&mut deps.storage, cx_module_id, &module)
    //     .unwrap();

    contract
        .store_capability(
            &mut deps.storage,
            port_id.to_string().as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();

    let commitment = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        commitment.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let connection_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, connection_id.clone(), conn_end)
        .unwrap();

    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a.clone();

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

    contract
        .store_channel_end(&mut deps.storage, port_id, channel_id, channel_end.clone())
        .unwrap();

    let res = contract.validate_channel_close_init(deps.as_mut(), info.clone(), &msg);
    let expected = on_chan_close_init_submessage(&msg, &channel_end, &connection_id);
    let data = cw_common::xcall_msg::ExecuteMsg::IbcChannelClose { msg: expected };
    let data = to_binary(&data).unwrap();
    let on_chan_open_init = create_channel_submesssage(
        "contractaddress".to_string(),
        data,
        info.funds,
        EXECUTE_ON_CHANNEL_CLOSE_INIT,
    );

    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap().messages[0], on_chan_open_init)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_close_init_channel_fail_missing_connection_end() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();
    // contract
    //     .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
    //     .unwrap();
    let module = Addr::unchecked("contractaddress");
    let _cx_module_id = module_id;
    // contract
    //     .add_route(&mut deps.storage, cx_module_id, &module)
    //     .unwrap();
    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();
    let connection_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a.clone();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(&mut deps.storage, port_id, channel_id, channel_end)
        .unwrap();

    contract
        .validate_channel_close_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_execute_close_init_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let connection_id = ConnectionId::new(5);
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a;
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id],
        version: Version::new("xcall".to_string()),
    };
    contract
        .store_channel_end(
            &mut deps.storage,
            port_id.clone(),
            channel_id.clone(),
            channel_end.clone(),
        )
        .unwrap();
    contract
        .store_channel(deps.as_mut().storage, &port_id, &channel_id, channel_end)
        .unwrap();
    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("Action").add_attribute("method", "channel_close_init")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_CLOSE_INIT,
        result,
    };

    let result = contract.execute_channel_close_init(deps.as_mut(), reply);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_execute_close_init_channel_fail() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();
    // contract
    //     .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
    //     .unwrap();
    let module = Addr::unchecked("contractaddress");
    let _cx_module_id = module_id;
    // contract
    //     .add_route(&mut deps.storage, cx_module_id, &module)
    //     .unwrap();
    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a;
    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("Action").add_attribute("method", "channel_close_init")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_CLOSE_INIT,
        result,
    };

    contract
        .execute_channel_close_init(deps.as_mut(), reply)
        .unwrap();
}

#[test]
fn test_channel_close_init_validate() {
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let connection_id = ConnectionId::new(5);
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a.clone();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id),
        },
        connection_hops: vec![connection_id],
        version: Version::new("xcall".to_string()),
    };
    let channel_close_init_validate = channel_close_init_validate(&channel_end, &msg);

    assert!(channel_close_init_validate.is_ok())
}

#[test]
#[should_panic(expected = "InvalidChannelState")]
fn test_channel_close_init_validate_fail() {
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let connection_id = ConnectionId::new(5);
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a.clone();
    let channel_end = ChannelEnd {
        state: State::Closed,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id),
        },
        connection_hops: vec![connection_id],
        version: Version::new("xcall".to_string()),
    };
    channel_close_init_validate(&channel_end, &msg).unwrap();
}

#[test]
fn test_on_chan_close_init_submessage() {
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let connection_id = ConnectionId::new(5);
    let channel_id = msg.chan_id_on_a.clone();
    let port_id = msg.port_id_on_a.clone();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id,
            channel_id: Some(channel_id),
        },
        connection_hops: vec![connection_id.clone()],
        version: Version::new("xcall".to_string()),
    };
    let channel_close_init_validate =
        on_chan_close_init_submessage(&msg, &channel_end, &connection_id);

    assert_eq!("xcall", channel_close_init_validate.channel().version);
    assert_eq!(
        IbcOrder::Unordered,
        channel_close_init_validate.channel().order
    );
}
