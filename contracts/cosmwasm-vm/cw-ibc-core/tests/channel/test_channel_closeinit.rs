use super::*;
use cw_ibc_core::{
    conversions::to_ibc_channel_id,
    ics04_channel::close_init::{channel_close_init_validate, on_chan_close_init_submessage},
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
use cw_ibc_core::{ChannelEnd, ConnectionEnd};

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
    let module = Addr::unchecked("contractaddress");

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
        .store_connection(deps.as_mut().storage, &connection_id, &conn_end)
        .unwrap();

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
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
        .unwrap();

    let res = contract.validate_channel_close_init(deps.as_mut(), info.clone(), &msg);
    let expected =
        on_chan_close_init_submessage(&port_id, &channel_id, &channel_end, &connection_id);

    let data = cw_common::xcall_connection_msg::ExecuteMsg::IbcChannelClose { msg: expected };
    let data = to_binary(&data).unwrap();
    let on_chan_open_init = create_channel_submesssage(
        "contractaddress".to_string(),
        data,
        info.funds,
        EXECUTE_ON_CHANNEL_CLOSE_INIT,
    );
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0], on_chan_open_init)
}

#[should_panic(expected = "IbcConnectionError { error: ConnectionMismatch { connection_id: ConnectionId(\"connection-5\") } }")]
#[test]
fn test_validate_close_init_channel_fails_invalid_connection_state() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();

    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let module = Addr::unchecked("contractaddress");

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
        common::ibc::core::ics03_connection::connection::State::Uninitialized,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let connection_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &connection_id, &conn_end)
        .unwrap();

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
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
        .unwrap();

     contract.validate_channel_close_init(deps.as_mut(), info.clone(), &msg).unwrap();
   
}

#[should_panic(expected = "IbcChannelError { error: ChannelClosed { channel_id: ChannelId(\"channel-0\") } }")]
#[test]
fn test_validate_close_init_channel_fails_on_closed_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();

    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let module = Addr::unchecked("contractaddress");

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
        .store_connection(deps.as_mut().storage, &connection_id, &conn_end)
        .unwrap();

    let channel_end = ChannelEnd {
        state: State::Closed,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![connection_id.clone()],
        version: Version::new("xcall".to_string()),
    };

    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
        .unwrap();

     contract.validate_channel_close_init(deps.as_mut(), info.clone(), &msg).unwrap();
   
}

#[should_panic(expected = "IbcChannelError { error: InvalidConnectionHopsLength { expected: 1, actual: 0 } }")]
#[test]
fn test_validate_close_init_channel_fails_on_invalid_connection_hops() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();

    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
    let module = Addr::unchecked("contractaddress");

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
        .store_connection(deps.as_mut().storage, &connection_id, &conn_end)
        .unwrap();

    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: port_id.clone(),
            channel_id: Some(channel_id.clone()),
        },
        connection_hops: vec![],
        version: Version::new("xcall".to_string()),
    };

    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
        .unwrap();

     contract.validate_channel_close_init(deps.as_mut(), info.clone(), &msg).unwrap();
   
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_close_init_channel_fail_missing_connection_end() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_chan_close_init();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let _channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();

    let module = Addr::unchecked("contractaddress");
    let _cx_module_id = module_id;

    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();
    let connection_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&msg.channel_id).unwrap();
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
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
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
