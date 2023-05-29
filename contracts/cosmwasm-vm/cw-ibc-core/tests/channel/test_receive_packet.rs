use cosmwasm_std::Empty;
use cosmwasm_std::IbcReceiveResponse;
use cw_common::client_response::PacketResponse;
use cw_common::client_response::XcallPacketResponseData;

use common::ibc::core::ics03_connection::connection::Counterparty as ConnectionCounterparty;
use common::ibc::core::ics03_connection::connection::State as ConnectionState;
use common::ibc::core::ics04_channel::msgs::recv_packet::MsgRecvPacket;
use common::ibc::core::ics04_channel::msgs::PacketMsg;
use common::ibc::core::ics04_channel::packet::Receipt;
use common::ibc::timestamp::Timestamp;
use cw_common::raw_types::channel::RawMsgRecvPacket;
use cw_common::types::Ack;

use super::*;

pub fn make_ack_success() -> Binary {
    let res = Ack::Result(b"1".into());

    to_binary(&res).unwrap()
}

pub fn get_dummy_raw_packet_recv(timeout_height: u64, timeout_timestamp: u64) -> RawPacket {
    RawPacket {
        sequence: 1,
        source_port: PortId::default().to_string(),
        source_channel: ChannelId::default().to_string(),
        destination_port: PortId::default().to_string(),
        destination_channel: ChannelId::default().to_string(),
        data: vec![0],
        timeout_height: Some(RawHeight {
            revision_number: 12,
            revision_height: timeout_height,
        }),
        timeout_timestamp,
    }
}

pub fn get_dummy_raw_msg_recv_packet(height: u64) -> RawMsgRecvPacket {
    let timestamp = Timestamp::default();
    RawMsgRecvPacket {
        packet: Some(get_dummy_raw_packet_recv(height, timestamp.nanoseconds())),
        proof_commitment: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

#[test]
fn test_receive_packet() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);

    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();
    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::default(),
        Counterparty::new(packet.port_id_on_a, Some(packet.chan_id_on_a)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );

    let conn_end_on_b = ConnectionEnd::new(
        ConnectionState::Open,
        IbcClientId::default(),
        ConnectionCounterparty::new(
            IbcClientId::default(),
            Some(IbcConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );

    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone(),
            packet.chan_id_on_b.clone(),
            chan_end_on_b.clone(),
        )
        .unwrap();
    let conn_id_on_b = &chan_end_on_b.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, conn_id_on_b.clone(), conn_end_on_b)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 12,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client)
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let height = msg.proof_height_on_a;
    let consenus_state = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();
    let env = mock_env();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            IbcClientId::default(),
            light_client.to_string(),
        )
        .unwrap();
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone(),
            packet.chan_id_on_b,
            chan_end_on_b.clone(),
        )
        .unwrap();
    let res = contract.validate_receive_packet(deps.as_mut(), info, &msg);

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 521);
}

#[test]
fn test_receive_packet_validate_reply_from_light_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 200000000);

    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.packet.port_id_on_b.clone();
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();
    let module = Addr::unchecked("contractaddress");
    contract
        .add_route(&mut deps.storage, module_id, &module)
        .unwrap();
    let packet_repsone = PacketResponse {
        seq_on_a: msg.packet.sequence,
        port_id_on_a: msg.packet.port_id_on_a.clone(),
        chan_id_on_a: msg.packet.chan_id_on_a.clone(),
        port_id_on_b: msg.packet.port_id_on_b,
        chan_id_on_b: msg.packet.chan_id_on_b,
        data: msg.packet.data,
        timeout_height_on_b: msg.packet.timeout_height_on_b,
        timeout_timestamp_on_b: msg.packet.timeout_timestamp_on_b,
    };
    let message_info = cw_common::types::MessageInfo {
        sender: info.sender,
        funds: info.funds,
    };

    let data = PacketDataResponse {
        packet: packet_repsone,
        signer: msg.signer,
        acknowledgement: None,
        message_info,
    };
    let data_bin = to_binary(&data).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let message = Reply { id: 0, result };

    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(packet.port_id_on_a, Some(packet.chan_id_on_a)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone(),
            packet.chan_id_on_b,
            chan_end_on_b,
        )
        .unwrap();
    contract
        .store_packet_receipt(
            &mut deps.storage,
            &msg.packet.port_id_on_a,
            &msg.packet.chan_id_on_a,
            msg.packet.sequence,
            Receipt::Ok,
        )
        .unwrap();

    let res = contract.receive_packet_validate_reply_from_light_client(deps.as_mut(), message);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 522)
}

#[test]
#[should_panic(expected = "PacketCommitmentNotFound")]
fn test_receive_packet_validate_reply_from_light_client_fail() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.packet.port_id_on_b.clone();
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();
    let module = Addr::unchecked("contractaddress");
    contract
        .add_route(&mut deps.storage, module_id, &module)
        .unwrap();

    let packet_repsone = PacketResponse {
        seq_on_a: msg.packet.sequence,
        port_id_on_a: msg.packet.port_id_on_a.clone(),
        chan_id_on_a: msg.packet.chan_id_on_a.clone(),
        port_id_on_b: msg.packet.port_id_on_b,
        chan_id_on_b: msg.packet.chan_id_on_b,
        data: msg.packet.data,
        timeout_height_on_b: msg.packet.timeout_height_on_b,
        timeout_timestamp_on_b: msg.packet.timeout_timestamp_on_b,
    };

    let message_info = cw_common::types::MessageInfo {
        sender: info.sender,
        funds: info.funds,
    };

    let data = PacketDataResponse {
        packet: packet_repsone,
        signer: msg.signer,
        acknowledgement: None,
        message_info,
    };
    let data_bin = to_binary(&data).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let message = Reply { id: 0, result };

    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(packet.port_id_on_a, Some(packet.chan_id_on_a)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone(),
            packet.chan_id_on_b,
            chan_end_on_b,
        )
        .unwrap();

    contract
        .receive_packet_validate_reply_from_light_client(deps.as_mut(), message)
        .unwrap();
}

#[test]
fn execute_receive_packet() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();

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

    let packet = IbcPacket::new(vec![0, 1, 2, 3], src, dst, 0, timeout);

    let ack: IbcReceiveResponse<Empty> = IbcReceiveResponse::default();
    let event = Event::new("test");
    let acknowledgement = XcallPacketResponseData {
        packet: packet.clone(),
        acknowledgement: make_ack_success().to_vec(),
    };
    let ack = ack.add_event(event);
    let ack_data_bin = to_binary(&acknowledgement).unwrap();
    let ack = ack.set_ack(ack_data_bin);
    let data_bin = to_binary(&ack).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply = Reply { id: 0, result };

    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(
            IbcPortId::from_str(&packet.src.port_id).unwrap(),
            Some(IbcChannelId::from_str(&packet.src.channel_id).unwrap()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            IbcPortId::from_str(&packet.src.port_id).unwrap(),
            IbcChannelId::from_str(&packet.src.channel_id).unwrap(),
            chan_end_on_b,
        )
        .unwrap();

    let res = contract.execute_receive_packet(deps.as_mut(), reply);

    assert_eq!(res.unwrap().events[0].ty, "receive_packet")
}

#[test]
fn execute_receive_packet_ordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
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

    let ack: IbcReceiveResponse<Empty> = IbcReceiveResponse::default();
    let event = Event::new("test");
    let acknowledgement = XcallPacketResponseData {
        packet: packet.clone(),
        acknowledgement: make_ack_success().to_vec(),
    };
    let ack = ack.add_event(event);
    let ack_data_bin = to_binary(&acknowledgement).unwrap();
    let ack = ack.set_ack(ack_data_bin);
    let data_bin = to_binary(&ack).unwrap();

    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply = Reply { id: 0, result };

    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::Ordered,
        Counterparty::new(
            IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            Some(IbcChannelId::from_str(&packet.dest.channel_id).unwrap()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            IbcPortId::from_str(&packet.src.port_id).unwrap(),
            IbcChannelId::from_str(&packet.src.channel_id).unwrap(),
            chan_end_on_b,
        )
        .unwrap();
    contract
        .store_next_sequence_recv(
            &mut deps.storage,
            IbcPortId::from_str(&packet.src.port_id).unwrap(),
            IbcChannelId::from_str(&packet.src.channel_id).unwrap(),
            1.into(),
        )
        .unwrap();

    let res = contract.execute_receive_packet(deps.as_mut(), reply);
    let seq = contract.get_next_sequence_recv(
        &deps.storage,
        IbcPortId::from_str(&packet.src.port_id).unwrap(),
        IbcChannelId::from_str(&packet.src.channel_id).unwrap(),
    );
    assert!(res.is_ok());
    assert_eq!(res.unwrap().events[0].ty, "receive_packet");
    assert!(seq.is_ok());
    assert_eq!(seq.unwrap(), 2.into())
}
#[test]
#[should_panic(
    expected = "Std(NotFound { kind: \"common::ibc::core::ics04_channel::packet::Sequence\" })"
)]
fn execute_receive_packet_ordered_fail_missing_seq_on_a() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
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

    let ack: IbcReceiveResponse<Empty> = IbcReceiveResponse::default();
    let event = Event::new("test");
    let acknowledgement = XcallPacketResponseData {
        packet: packet.clone(),
        acknowledgement: make_ack_success().to_vec(),
    };
    let ack = ack.add_event(event);
    let ack_data_bin = to_binary(&acknowledgement).unwrap();
    let ack = ack.set_ack(ack_data_bin);
    let data_bin = to_binary(&ack).unwrap();

    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply = Reply { id: 0, result };

    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::Ordered,
        Counterparty::new(
            IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            Some(IbcChannelId::from_str(&packet.dest.channel_id).unwrap()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            IbcPortId::from_str(&packet.src.port_id).unwrap(),
            IbcChannelId::from_str(&packet.src.channel_id).unwrap(),
            chan_end_on_b,
        )
        .unwrap();
    contract
        .execute_receive_packet(deps.as_mut(), reply)
        .unwrap();
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_receive_packet_fail_missing_channel() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();

    contract
        .validate_receive_packet(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_lookup_module_packet() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    ctx.store_module_by_port(
        &mut deps.storage,
        msg.packet.port_id_on_a.clone(),
        module_id,
    )
    .unwrap();
    let channel_msg = PacketMsg::Recv(msg);
    let res = ctx.lookup_module_packet(&mut deps.storage, &channel_msg);

    assert!(res.is_ok());
    assert_eq!("contractaddress", res.unwrap().to_string())
}

#[test]
#[should_panic(expected = "UnknownPort")]
fn test_lookup_module_packet_fail() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let channel_msg = PacketMsg::Recv(msg);

    ctx.lookup_module_packet(&mut deps.storage, &channel_msg)
        .unwrap();
}
