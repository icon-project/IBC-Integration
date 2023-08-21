use std::{str::FromStr, time::Duration};

use common::traits::AnyTypes;
use cosmwasm_std::{
    to_binary, Addr, Event, IbcEndpoint, IbcPacket, IbcPacketReceiveMsg, IbcTimeout,
    IbcTimeoutBlock, Reply, SubMsgResponse, SubMsgResult,
};

use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};

use common::ibc::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};
use common::ibc::{
    core::ics04_channel::{
        channel::{Counterparty, Order, State},
        Version,
    },
    events::IbcEventType,
};
use cw_common::ibc_types::{IbcClientId, IbcConnectionId, IbcPortId};
use cw_common::raw_types::channel::{RawMsgChannelCloseInit, RawPacket};
use cw_common::raw_types::to_raw_packet;

use cw_ibc_core::conversions::{to_ibc_channel, to_ibc_channel_id, to_ibc_height, to_ibc_port_id};

use cw_ibc_core::ics04_channel::open_init::{
    create_channel_submesssage, on_chan_open_init_submessage,
};

use cw_ibc_core::ics04_channel::{
    create_channel_event, create_packet_event, EXECUTE_ON_CHANNEL_OPEN_INIT,
    EXECUTE_ON_CHANNEL_OPEN_TRY,
};
use cw_ibc_core::light_client::light_client::LightClient;
use cw_ibc_core::{
    context::CwIbcCoreContext, ics04_channel::create_channel_id_generated_event, ChannelEnd,
    ConnectionEnd, Sequence,
};
use cw_ibc_core::{traits::*, IbcClientType};

pub mod channel;
pub mod setup;
use prost::Message;
use setup::*;

#[test]
fn test_add_channel() {
    let ctx = CwIbcCoreContext::new();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let channel_end = ChannelEnd::new(
        State::Init,
        Order::None,
        Counterparty::default(),
        Vec::default(),
        Version::from("ics-20".to_string()),
    );
    let mut mock_deps = deps();

    let _storing = ctx.store_channel_end(
        mock_deps.as_mut().storage,
        &port_id,
        &channel_id,
        &channel_end,
    );

    let retrived_channel_end =
        ctx.get_channel_end(mock_deps.as_ref().storage, &port_id, &channel_id);

    assert_eq!(channel_end, retrived_channel_end.unwrap())
}

#[test]
fn test_channel_sequence_initialisation() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let _store = ctx.init_channel_counter(mock_deps.as_mut().storage, u64::default());
    let result = ctx.channel_counter(mock_deps.as_ref().storage);

    assert_eq!(0, result.unwrap());

    let incremented_result = ctx.increase_channel_sequence(mock_deps.as_mut().storage);
    assert_eq!(1, incremented_result.unwrap());
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
fn test_channel_sequence_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    ctx.increase_channel_sequence(mock_deps.as_mut().storage)
        .unwrap();
}

#[test]
fn test_channel_sequence_send() {
    let ctx = CwIbcCoreContext::new();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let sequence = Sequence::from(6);
    let mut mock_deps = deps();

    let _store =
        ctx.store_next_sequence_send(mock_deps.as_mut().storage, &port_id, &channel_id, &sequence);
    let result = ctx.get_next_sequence_send(mock_deps.as_ref().storage, &port_id, &channel_id);

    assert_eq!(sequence, result.unwrap())
}

#[test]
fn test_channel_sequence_send_increment() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let sequence = Sequence::default();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let _store =
        ctx.store_next_sequence_send(mock_deps.as_mut().storage, &port_id, &channel_id, &sequence);
    let result = ctx.get_next_sequence_send(mock_deps.as_ref().storage, &port_id, &channel_id);

    assert_eq!(sequence, result.unwrap());

    let incremented_result =
        ctx.increase_next_sequence_send(mock_deps.as_mut().storage, &port_id, &channel_id);
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
fn test_channel_sequence_recv_increment() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let sequence = Sequence::default();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let _store =
        ctx.store_next_sequence_recv(mock_deps.as_mut().storage, &port_id, &channel_id, &sequence);
    let result = ctx.get_next_sequence_recv(mock_deps.as_ref().storage, &port_id, &channel_id);

    assert_eq!(sequence, result.unwrap());

    let incremented_result =
        ctx.increase_next_sequence_recv(mock_deps.as_mut().storage, &port_id, &channel_id);
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
fn test_channel_sequence_ack_increment() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let sequence = Sequence::default();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let _store =
        ctx.store_next_sequence_ack(mock_deps.as_mut().storage, &port_id, &channel_id, &sequence);
    let result = ctx.get_next_sequence_ack(mock_deps.as_ref().storage, &port_id, &channel_id);

    assert_eq!(sequence, result.unwrap());

    let incremented_result =
        ctx.increase_next_sequence_ack(mock_deps.as_mut().storage, &port_id, &channel_id);
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
#[should_panic(expected = "MissingNextAckSeq")]
fn test_channel_sequence_ack_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_ack(mock_deps.as_mut().storage, &port_id, &channel_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcPacketError")]
fn test_channel_sequence_send_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_send(mock_deps.as_mut().storage, &port_id, &channel_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcPacketError")]
fn test_channel_sequence_recv_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_recv(mock_deps.as_mut().storage, &port_id, &channel_id)
        .unwrap();
}

#[test]
fn create_channel_id_event_test() {
    let client_id = ChannelId::new(10);
    let event = create_channel_id_generated_event(client_id);

    assert_eq!("channel_id_created", event.ty);
    assert_eq!("channel-10", event.attributes[0].value);
    assert_eq!("channel_id", event.attributes[0].key)
}

#[test]
fn create_open_ack_channel_event_test() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);

    let version = Version::from_str(&default_raw_msg.counterparty_version).unwrap();
    let dest_channel = to_ibc_channel_id(&default_raw_msg.counterparty_channel_id).unwrap();
    let port_id = to_ibc_port_id(&default_raw_msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&default_raw_msg.channel_id).unwrap();

    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: IbcPortId::default(),
            channel_id: Some(dest_channel),
        },
        connection_hops: vec![ConnectionId::default()],
        version,
    };
    let event = create_channel_event(
        IbcEventType::OpenAckChannel,
        port_id.as_str(),
        channel_id.as_str(),
        &channel_end,
    )
    .unwrap();

    assert_eq!(IbcEventType::OpenAckChannel.as_str(), event.ty);
    assert_eq!("channel-0", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("channel_id", event.attributes[1].key);
}

#[test]
fn create_open_confirm_channel_event_test() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);

    let port_id = to_ibc_port_id(&default_raw_msg.port_id).unwrap();
    let channel_id = to_ibc_channel_id(&default_raw_msg.channel_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Open,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: IbcPortId::default(),
            channel_id: Some(ChannelId::default()),
        },
        connection_hops: vec![ConnectionId::default()],
        version: Version::empty(),
    };

    let event = create_channel_event(
        IbcEventType::OpenConfirmChannel,
        port_id.as_str(),
        channel_id.as_str(),
        &channel_end,
    )
    .unwrap();

    assert_eq!(IbcEventType::OpenConfirmChannel.as_str(), event.ty);
    assert_eq!("channel-0", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("port_id", event.attributes[0].key);
}

#[test]
fn create_open_init_channel_event_test() {
    let default_raw_msg = get_dummy_raw_msg_chan_open_init(Some(10));
    let message = default_raw_msg;
    let channel = to_ibc_channel(message.channel).unwrap();
    let channel_id = ChannelId::new(10);
    let dest_port = channel.remote.port_id.clone();
    let src_port = to_ibc_port_id(&message.port_id).unwrap();
    let channel_end = ChannelEnd {
        state: State::Init,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: dest_port,
            channel_id: None,
        },
        connection_hops: channel.connection_hops.clone(),
        version: channel.version().clone(),
    };
    let event = create_channel_event(
        IbcEventType::OpenInitChannel,
        src_port.as_ref(),
        channel_id.as_str(),
        &channel_end,
    )
    .unwrap();

    assert_eq!(IbcEventType::OpenInitChannel.as_str(), event.ty);
    assert_eq!("channel-10", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("version", event.attributes[4].key);
}

#[test]
fn create_open_try_channel_event_test() {
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(10);
    let message = default_raw_msg;
    let channel_id = ChannelId::new(11);
    let port_id = to_ibc_port_id(&message.port_id).unwrap();
    let channel = to_ibc_channel(message.channel).unwrap();

    let channel_end = ChannelEnd {
        state: State::TryOpen,
        ordering: Order::Unordered,
        remote: Counterparty {
            port_id: channel.remote.port_id.clone(),
            channel_id: channel.remote.channel_id.clone(),
        },
        connection_hops: channel.connection_hops.clone(),
        version: channel.version().clone(),
    };
    let event = create_channel_event(
        IbcEventType::OpenTryChannel,
        port_id.as_str(),
        channel_id.as_str(),
        &channel_end,
    )
    .unwrap();

    assert_eq!(IbcEventType::OpenTryChannel.as_str(), event.ty);
    assert_eq!("counterparty_port_id", event.attributes[2].key);
    assert_eq!("channel-11", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
}

#[test]
fn test_create_send_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);

    //  let event = create_send_packet_event(msg_back, &Order::Ordered, &IbcConnectionId::default());
    let event = create_packet_event(
        IbcEventType::SendPacket,
        raw,
        &Order::Ordered,
        &IbcConnectionId::default(),
        None,
    );
    assert_eq!(IbcEventType::SendPacket.as_str(), event.unwrap().ty)
}

#[test]
fn test_create_send_packet_with_invalid_utf_ok() {
    let raw = get_dummy_raw_packet(15, 0);

    let raw = RawPacket {
        data: vec![u8::MAX],
        ..raw
    };
    let _event = create_packet_event(
        IbcEventType::SendPacket,
        raw,
        &Order::Ordered,
        &IbcConnectionId::default(),
        None,
    )
    .unwrap();
}

#[test]
fn test_create_write_ack_packet_event() {
    let _raw = get_dummy_raw_packet(15, 0);

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 10,
    };
    let timeout = IbcTimeout::with_both(timeout_block, cosmwasm_std::Timestamp::from_nanos(100));
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let packet = IbcPacket::new(vec![0, 1, 2, 3], src, dst, 1, timeout);
    let ibc_packet_recv_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relayer"));

    let event = create_packet_event(
        IbcEventType::WriteAck,
        to_raw_packet(ibc_packet_recv_message.packet),
        &Order::Unordered,
        &IbcConnectionId::default(),
        Some(Vec::<u8>::new()),
    );
    assert_eq!(IbcEventType::WriteAck.as_str(), event.unwrap().ty)
}

#[test]
fn test_create_write_ack_packet_event_with_invalidutf8_ok() {
    let raw = get_dummy_raw_packet(15, 0);
    let _event = create_packet_event(
        IbcEventType::SendPacket,
        raw,
        &Order::Ordered,
        &IbcConnectionId::default(),
        None,
    )
    .unwrap();
}

#[test]
fn test_create_ack_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);
    let event = create_packet_event(
        IbcEventType::AckPacket,
        raw,
        &Order::Ordered,
        &IbcConnectionId::default(),
        None,
    )
    .unwrap();
    assert_eq!("acknowledge_packet", event.ty)
}

#[test]
fn test_create_timout_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);

    let event = create_packet_event(
        IbcEventType::Timeout,
        raw,
        &Order::Ordered,
        &IbcConnectionId::default(),
        None,
    )
    .unwrap();
    assert_eq!("timeout_packet", event.ty)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_open_init_channel_fail_missing_connection_end() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_init(None);

    contract
        .validate_channel_open_init(deps.as_mut(), info, &raw)
        .unwrap();
}

#[test]
pub fn test_create_close_init_channel_event() {
    let raw = get_dummy_raw_msg_chan_close_init();

    let channel_end = ChannelEnd {
        state: State::Closed,
        ordering: Order::Ordered,
        remote: Counterparty::default(),
        connection_hops: vec![ConnectionId::default()],
        version: Version::default(),
    };

    let event = create_channel_event(
        IbcEventType::CloseInitChannel,
        &raw.port_id,
        &raw.channel_id,
        &channel_end,
    )
    .unwrap();

    assert_eq!(event.ty, IbcEventType::CloseInitChannel.as_str())
}

#[test]
pub fn test_create_close_confirm_channel_event() {
    let proof_height = 10;
    let raw = get_dummy_raw_msg_chan_close_confirm(proof_height);

    let channel_end = ChannelEnd {
        state: State::Closed,
        ordering: Order::Ordered,
        remote: Counterparty::default(),
        connection_hops: vec![ConnectionId::default()],
        version: Version::default(),
    };

    let event = create_channel_event(
        IbcEventType::CloseConfirmChannel,
        raw.port_id.as_str(),
        raw.channel_id.as_str(),
        &channel_end,
    )
    .unwrap();

    assert_eq!(event.ty, IbcEventType::CloseConfirmChannel.as_str())
}

#[test]
fn test_validate_open_init_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_init(None);

    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let src_channel = ChannelId::new(0);
    let src_port_id = to_ibc_port_id(&raw.port_id).unwrap();

    let module = Addr::unchecked("contractaddress");
    let _cx_module_id = module_id;

    let channel_end = to_ibc_channel(raw.channel.clone()).unwrap();

    contract
        .claim_capability(
            &mut deps.storage,
            src_port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();

    let conn_end = get_dummy_connection();
    let conn_id = ConnectionId::new(0);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();

    let res = contract.validate_channel_open_init(deps.as_mut(), info.clone(), &raw);

    let expected = on_chan_open_init_submessage(&channel_end, &src_port_id, &src_channel, &conn_id);
    let data = cw_common::ibc_dapp_msg::ExecuteMsg::IbcChannelOpen { msg: expected };
    let data = to_binary(&data).unwrap();
    let on_chan_open_init = create_channel_submesssage(
        "contractaddress".to_string(),
        data,
        info.funds,
        EXECUTE_ON_CHANNEL_OPEN_INIT,
    );
    println!("{:?}", res);

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0], on_chan_open_init)
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_validate_open_init_channel_fail_missing_module_id() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_init(None);
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(0);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();

    let res = contract.validate_channel_open_init(deps.as_mut(), info, &raw);
    res.unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_open_try_channel_fail_missing_connection_end() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    //  let msg = MsgChannelOpenTry::try_from(raw).unwrap();
    let _channel = to_ibc_channel(raw.channel.clone()).unwrap();

    contract
        .validate_channel_open_try(deps.as_mut(), info, &raw)
        .unwrap();
}

#[test]
fn test_validate_open_try_channel() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let raw = get_dummy_raw_msg_chan_open_try(10);

    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let channel = to_ibc_channel(raw.channel.clone()).unwrap();
    let port_id = channel.counterparty().port_id.clone();

    let light_client = LightClient::new("lightclient".to_string());

    contract
        .bind_port(&mut deps.storage, &port_id, "moduleaddress".to_string())
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(5);
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id),
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(0);
    //  msg.connection_hops_on_b = vec![conn_id.clone()];
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();

    let client_state: ClientState = get_dummy_client_state();

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &env,
            &IbcClientId::default(),
            client,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let client_type = IbcClientType::new("iconclient".to_string());

    contract
        .store_client_into_registry(
            &mut deps.storage,
            client_type,
            "contractaddress".to_string(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = to_ibc_height(raw.proof_height.clone()).unwrap();
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let res = contract.validate_channel_open_try(deps.as_mut(), info, &raw);

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, EXECUTE_ON_CHANNEL_OPEN_TRY)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_open_try_channel_fail_missing_client_state() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(0);
    //   msg.connection_hops_on_b = vec![conn_id.clone()];
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();

    contract
        .validate_channel_open_try(deps.as_mut(), info, &raw)
        .unwrap();
}

#[test]
fn test_execute_open_try_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let msg = get_dummy_raw_msg_chan_open_try(10);
    let channel = to_ibc_channel(msg.channel).unwrap();
    let port_id = to_ibc_port_id(&msg.port_id).unwrap();
    let channel_id = ChannelId::new(0);
    let counter_port_id = channel.remote.port_id.clone();
    let counter_channel_id = channel.remote.channel_id.clone();

    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());

    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);

    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, &conn_id, &conn_end)
        .unwrap();

    let counter_party = Counterparty::new(counter_port_id, counter_channel_id);
    // creating new channel_id
    let channel_end = ChannelEnd::new(
        State::Uninitialized,
        *channel.ordering(),
        counter_party,
        channel.connection_hops.clone(),
        channel.version().clone(),
    );
    contract
        .store_channel_end(&mut deps.storage, &port_id, &channel_id, &channel_end)
        .unwrap();

    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    contract
        .store_callback_data(
            deps.as_mut().storage,
            EXECUTE_ON_CHANNEL_OPEN_TRY,
            &expected_data,
        )
        .unwrap();

    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("action").add_attribute("action", "channel open try execution")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_TRY,
        result,
    };

    let result = contract.execute_channel_open_try(deps.as_mut(), reply);
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap().events[0].ty, "channel_id_created");
    assert_eq!(result.unwrap().events[1].ty, "channel_open_try")
}

#[test]
fn test_get_channel() {
    let ctx = CwIbcCoreContext::new();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let channel_end = ChannelEnd::new(
        State::Init,
        Order::None,
        Counterparty::default(),
        Vec::default(),
        Version::from("ics-20".to_string()),
    );
    let mut mock_deps = deps();
    ctx.store_channel_end(
        mock_deps.as_mut().storage,
        &port_id,
        &channel_id,
        &channel_end,
    )
    .unwrap();
    let retrived_channel_end =
        ctx.get_channel_end(mock_deps.as_ref().storage, &port_id, &channel_id);

    assert_eq!(channel_end, retrived_channel_end.unwrap())
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_get_channel_fail() {
    let ctx = CwIbcCoreContext::new();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let mock_deps = deps();
    ctx.get_channel_end(mock_deps.as_ref().storage, &port_id, &channel_id)
        .unwrap();
}
