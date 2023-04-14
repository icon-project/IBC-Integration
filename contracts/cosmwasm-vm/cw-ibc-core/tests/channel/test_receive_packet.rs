use cosmwasm_std::Empty;
use cosmwasm_std::IbcReceiveResponse;
use ibc::core::ics03_connection::connection::Counterparty as ConnectionCounterparty;
use ibc::core::ics03_connection::connection::State as ConnectionState;
use ibc::core::ics04_channel::msgs::recv_packet::MsgRecvPacket;
use ibc::core::ics04_channel::packet::Receipt;
use ibc::timestamp::Timestamp;
use ibc_proto::ibc::core::channel::v1::MsgRecvPacket as RawMsgRecvPacket;
use ibc_proto::ibc::core::client::v1::Height as RawHeight;

use super::*;

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
    let info = create_mock_info("channel-creater", "umlg", 2000);

    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();
    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::default(),
        Counterparty::new(packet.port_id_on_a, Some(packet.chan_id_on_a)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    let conn_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
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
            packet.port_id_on_b.clone().into(),
            packet.chan_id_on_b.clone().into(),
            chan_end_on_b.clone(),
        )
        .unwrap();
    let conn_id_on_b = &chan_end_on_b.connection_hops()[0];
    contract
        .store_connection(
            &mut deps.storage,
            conn_id_on_b.clone().into(),
            conn_end_on_b.clone(),
        )
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

    let client = to_vec(&client_state);
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client.unwrap())
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let height = msg.proof_height_on_a;
    let consenus_state = to_vec(&consenus_state).unwrap();
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
            IbcClientId::default().into(),
            light_client.to_string(),
        )
        .unwrap();
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone().into(),
            packet.chan_id_on_b.clone().into(),
            chan_end_on_b.clone(),
        )
        .unwrap();
    let res = contract.validate_receive_packet(deps.as_mut(), info, &msg);

    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 531);
}

#[test]
fn test_receive_packet_validate_reply_from_light_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.packet.port_id_on_b.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();
    let module = Addr::unchecked("contractaddress");
    contract
        .add_route(&mut deps.storage, module_id.clone().into(), &module)
        .unwrap();

    let data = PacketData {
        packet: msg.packet.clone(),
        signer: msg.signer,
        acknowledgement:None
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
            packet.port_id_on_b.clone().into(),
            packet.chan_id_on_b.clone().into(),
            chan_end_on_b.clone(),
        )
        .unwrap();
    contract
        .store_packet_receipt(
            &mut deps.storage,
            &msg.packet.port_id_on_a.into(),
            &msg.packet.chan_id_on_a.into(),
            msg.packet.seq_on_a,
            Receipt::Ok,
        )
        .unwrap();

    let res =
        contract.receive_packet_validate_reply_from_light_client(deps.as_mut(), info, message);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 532)
}

#[test]
#[should_panic(expected = "PacketCommitmentNotFound")]
fn test_receive_packet_validate_reply_from_light_client_fail() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();
    let module_id = ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = PortId::from(msg.packet.port_id_on_b.clone());
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();
    let module = Addr::unchecked("contractaddress");
    contract
        .add_route(&mut deps.storage, module_id.clone().into(), &module)
        .unwrap();

    let data = PacketData {
        packet: msg.packet.clone(),
        signer: msg.signer,
        acknowledgement:None
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
            packet.port_id_on_b.clone().into(),
            packet.chan_id_on_b.clone().into(),
            chan_end_on_b.clone(),
        )
        .unwrap();

    contract
        .receive_packet_validate_reply_from_light_client(deps.as_mut(), info, message)
        .unwrap();
}

#[test]
fn execute_receive_packet() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();

    let ack: IbcReceiveResponse<Empty> = IbcReceiveResponse::default();
    let event = Event::new("test")
        .add_attribute("key", packet.port_id_on_b.as_str())
        .add_attribute("key", packet.chan_id_on_b.as_str())
        .add_attribute("key", packet.seq_on_a.to_string());
    let acknowledgement = cw_xcall::ack::make_ack_success();
    let ack = ack.add_event(event);
    let ack = ack.set_ack(acknowledgement);
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
        Counterparty::new(packet.port_id_on_a, Some(packet.chan_id_on_a)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone().into(),
            packet.chan_id_on_b.clone().into(),
            chan_end_on_b.clone(),
        )
        .unwrap();

    let res = contract.execute_receive_packet(deps.as_mut(), reply);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().events[0].ty, "test")
}

#[test]
fn execute_receive_packet_ordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();

    let ack: IbcReceiveResponse<Empty> = IbcReceiveResponse::default();
    let event = Event::new("test")
        .add_attribute("key", packet.port_id_on_b.as_str())
        .add_attribute("key", packet.chan_id_on_b.as_str())
        .add_attribute("key", packet.seq_on_a.to_string());
    let acknowledgement = cw_xcall::ack::make_ack_success();
    let ack = ack.add_event(event);
    let ack = ack.set_ack(acknowledgement);
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
        Counterparty::new(packet.port_id_on_a, Some(packet.chan_id_on_a)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone().into(),
            packet.chan_id_on_b.clone().into(),
            chan_end_on_b.clone(),
        )
        .unwrap();
    contract
        .store_next_sequence_recv(
            &mut deps.storage,
            packet.port_id_on_b.clone().into(),
            packet.chan_id_on_b.clone().into(),
            1.into(),
        )
        .unwrap();

    let res = contract.execute_receive_packet(deps.as_mut(), reply);
    let seq = contract.get_next_sequence_recv(
        &deps.storage,
        packet.port_id_on_b.clone().into(),
        packet.chan_id_on_b.clone().into(),
    );
    assert!(res.is_ok());
    assert_eq!(res.unwrap().events[0].ty, "test");
    assert!(seq.is_ok());
    assert_eq!(seq.unwrap(), 2.into())
}
#[test]
#[should_panic(expected = "ibc::core::ics04_channel::packet::Sequence")]
fn execute_receive_packet_ordered_fail_missing_sequence() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let msg = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(12)).unwrap();
    let packet = msg.packet.clone();

    let ack: IbcReceiveResponse<Empty> = IbcReceiveResponse::default();
    let event = Event::new("test")
        .add_attribute("key", packet.port_id_on_b.as_str())
        .add_attribute("key", packet.chan_id_on_b.as_str())
        .add_attribute("key", packet.seq_on_a.to_string());
    let acknowledgement = cw_xcall::ack::make_ack_success();
    let ack = ack.add_event(event);
    let ack = ack.set_ack(acknowledgement);
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
        Counterparty::new(packet.port_id_on_a, Some(packet.chan_id_on_a)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_b.clone().into(),
            packet.chan_id_on_b.clone().into(),
            chan_end_on_b.clone(),
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
