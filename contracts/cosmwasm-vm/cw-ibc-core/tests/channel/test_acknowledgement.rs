use cw_common::client_response::PacketResponse;
use cw_ibc_core::VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE;

use super::*;

#[test]
fn test_acknowledgement_packet_execute() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let height = 50;
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();
    let src = IbcEndpoint {
        port_id: msg.packet.port_id_on_a.to_string(),
        channel_id: msg.packet.chan_id_on_a.to_string(),
    };
    let dest = IbcEndpoint {
        port_id: msg.packet.port_id_on_b.to_string(),
        channel_id: msg.packet.chan_id_on_b.to_string(),
    };
    let timeoutblock = match msg.packet.timeout_height_on_b {
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::Never => IbcTimeoutBlock {
            revision: 1,
            height: 1,
        },
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::At(x) => IbcTimeoutBlock {
            revision: x.revision_number(),
            height: x.revision_height(),
        },
    };
    let timestamp = msg.packet.timeout_timestamp_on_b.nanoseconds();
    let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
    let timeout = IbcTimeout::with_both(timeoutblock, ibctimestamp);
    let ibc_packet = IbcPacket::new(
        msg.packet.data,
        src,
        dest,
        msg.packet.sequence.into(),
        timeout,
    );
    let ack = IbcAcknowledgement::new(msg.acknowledgement.as_bytes());
    let address = Addr::unchecked(msg.signer.to_string());
    let cosm_msg = cosmwasm_std::IbcPacketAckMsg::new(ack, ibc_packet, address);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
            &cosm_msg,
        )
        .unwrap();

    let data_bin = to_binary(&cosm_msg).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let message = Reply { id: 0, result };
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(
            msg.packet.port_id_on_b.clone(),
            Some(msg.packet.chan_id_on_b.clone()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            msg.packet.port_id_on_a.clone(),
            msg.packet.chan_id_on_a.clone(),
            chan_end_on_a_ordered,
        )
        .unwrap();
    let commitment = common::ibc::core::ics04_channel::commitment::PacketCommitment::from(
        "asdfd".as_bytes().to_vec(),
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &msg.packet.port_id_on_a,
            &msg.packet.chan_id_on_a,
            msg.packet.sequence,
            commitment,
        )
        .unwrap();

    let res = contract.acknowledgement_packet_execute(deps.as_mut(), message);
    assert!(res.is_ok());
    assert_eq!(
        "execute_acknowledgement_packet",
        res.unwrap().attributes[1].value
    )
}

#[test]
fn test_acknowledgement_packet_execute_ordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let height = 50;
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();
    let src = IbcEndpoint {
        port_id: msg.packet.port_id_on_a.to_string(),
        channel_id: msg.packet.chan_id_on_a.to_string(),
    };
    let dest = IbcEndpoint {
        port_id: msg.packet.port_id_on_b.to_string(),
        channel_id: msg.packet.chan_id_on_b.to_string(),
    };
    let timeoutblock = match msg.packet.timeout_height_on_b {
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::Never => IbcTimeoutBlock {
            revision: 1,
            height: 1,
        },
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::At(x) => IbcTimeoutBlock {
            revision: x.revision_number(),
            height: x.revision_height(),
        },
    };
    let timestamp = msg.packet.timeout_timestamp_on_b.nanoseconds();
    let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
    let timeout = IbcTimeout::with_both(timeoutblock, ibctimestamp);
    let ibc_packet = IbcPacket::new(
        msg.packet.data,
        src,
        dest,
        msg.packet.sequence.into(),
        timeout,
    );
    let ack = IbcAcknowledgement::new(msg.acknowledgement.as_bytes());
    let address = Addr::unchecked(msg.signer.to_string());
    let cosm_msg = cosmwasm_std::IbcPacketAckMsg::new(ack, ibc_packet, address);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
            &cosm_msg,
        )
        .unwrap();

    let data_bin = to_binary(&cosm_msg).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let message = Reply { id: 0, result };
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Ordered,
        Counterparty::new(
            msg.packet.port_id_on_b.clone(),
            Some(msg.packet.chan_id_on_b.clone()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            msg.packet.port_id_on_a.clone(),
            msg.packet.chan_id_on_a.clone(),
            chan_end_on_a_ordered,
        )
        .unwrap();
    let commitment = common::ibc::core::ics04_channel::commitment::PacketCommitment::from(
        "asdfd".as_bytes().to_vec(),
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &msg.packet.port_id_on_a,
            &msg.packet.chan_id_on_a,
            msg.packet.sequence,
            commitment,
        )
        .unwrap();
    contract
        .store_next_sequence_ack(
            &mut deps.storage,
            msg.packet.port_id_on_b.clone(),
            msg.packet.chan_id_on_b,
            1.into(),
        )
        .unwrap();

    let res = contract.acknowledgement_packet_execute(deps.as_mut(), message);
    assert!(res.is_ok());
    assert_eq!(
        "execute_acknowledgement_packet",
        res.unwrap().attributes[1].value
    )
}

#[test]
#[should_panic(expected = "MissingNextAckSeq")]
fn test_acknowledgement_packet_execute_fail() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let height = 50;
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();
    let src = IbcEndpoint {
        port_id: msg.packet.port_id_on_a.to_string(),
        channel_id: msg.packet.chan_id_on_a.to_string(),
    };
    let dest = IbcEndpoint {
        port_id: msg.packet.port_id_on_b.to_string(),
        channel_id: msg.packet.chan_id_on_b.to_string(),
    };
    let timeoutblock = match msg.packet.timeout_height_on_b {
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::Never => IbcTimeoutBlock {
            revision: 1,
            height: 1,
        },
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::At(x) => IbcTimeoutBlock {
            revision: x.revision_number(),
            height: x.revision_height(),
        },
    };
    let timestamp = msg.packet.timeout_timestamp_on_b.nanoseconds();
    let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
    let timeout = IbcTimeout::with_both(timeoutblock, ibctimestamp);
    let ibc_packet = IbcPacket::new(
        msg.packet.data,
        src,
        dest,
        msg.packet.sequence.into(),
        timeout,
    );
    let ack = IbcAcknowledgement::new(msg.acknowledgement.as_bytes());
    let address = Addr::unchecked(msg.signer.to_string());
    let cosm_msg = cosmwasm_std::IbcPacketAckMsg::new(ack, ibc_packet, address);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
            &cosm_msg,
        )
        .unwrap();

    let data_bin = to_binary(&cosm_msg).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let message = Reply { id: 0, result };
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Ordered,
        Counterparty::new(
            msg.packet.port_id_on_b.clone(),
            Some(msg.packet.chan_id_on_b.clone()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            msg.packet.port_id_on_a.clone(),
            msg.packet.chan_id_on_a.clone(),
            chan_end_on_a_ordered,
        )
        .unwrap();
    let commitment = common::ibc::core::ics04_channel::commitment::PacketCommitment::from(
        "asdfd".as_bytes().to_vec(),
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &msg.packet.port_id_on_a,
            &msg.packet.chan_id_on_a,
            msg.packet.sequence,
            commitment,
        )
        .unwrap();

    contract
        .acknowledgement_packet_execute(deps.as_mut(), message)
        .unwrap();
}

#[test]
fn acknowledgement_packet_validate_reply_from_light_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    let height = 10;
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();
    let packet_repsone = PacketResponse {
        seq_on_a: msg.packet.sequence,
        port_id_on_a: msg.packet.port_id_on_a.clone(),
        chan_id_on_a: msg.packet.chan_id_on_a,
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
    let packet_data = PacketDataResponse {
        message_info,
        packet: packet_repsone,
        signer: msg.signer.clone(),

        acknowledgement: Some(msg.acknowledgement.clone()),
    };

    let data_bin = to_binary(&packet_data).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply_message = Reply { id: 0, result };
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.packet.port_id_on_a;

    let module = Addr::unchecked("contractaddress");
    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();

    let res = contract
        .acknowledgement_packet_validate_reply_from_light_client(deps.as_mut(), reply_message);
    println!("{res:?}");
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().clone().messages[0].id, 532);
}

#[test]
#[should_panic(expected = "PacketAcknowledgementNotFound")]
fn acknowledgement_packet_validate_reply_from_light_client_fail() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);

    let height = 10;
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();
    let packet_repsone = PacketResponse {
        seq_on_a: msg.packet.sequence,
        port_id_on_a: msg.packet.port_id_on_a.clone(),
        chan_id_on_a: msg.packet.chan_id_on_a,
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

    let packet_data = PacketDataResponse {
        message_info,
        packet: packet_repsone,
        signer: msg.signer.clone(),

        acknowledgement: None,
    };

    let data_bin = to_binary(&packet_data).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply_message = Reply { id: 0, result };
    let _module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.packet.port_id_on_a;

    let module = Addr::unchecked("contractaddress");

    contract
        .claim_capability(
            &mut deps.storage,
            port_id.as_bytes().to_vec(),
            module.to_string(),
        )
        .unwrap();
    contract
        .acknowledgement_packet_validate_reply_from_light_client(deps.as_mut(), reply_message)
        .unwrap();
}

#[test]
fn test_acknowledgement_packet_validate_ordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let env = get_mock_env();

    let height = 10;
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();
    let packet = msg.packet.clone();
    //Store channel, connection and packet commitment
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Ordered,
        Counterparty::new(
            packet.port_id_on_b.clone(),
            Some(packet.chan_id_on_b.clone()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a.clone(),
            chan_end_on_a_ordered.clone(),
        )
        .unwrap();
    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let conn_end_on_a = ConnectionEnd::new(
        ConnectionState::Open,
        ClientId::default(),
        ConnectionCounterparty::new(
            ClientId::default(),
            Some(ConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );
    contract
        .store_connection(
            &mut deps.storage,
            chan_end_on_a_ordered.connection_hops()[0].clone(),
            conn_end_on_a,
        )
        .unwrap();
    let packet_commitment = compute_packet_commitment(
        &msg.packet.data,
        &msg.packet.timeout_height_on_b,
        &msg.packet.timeout_timestamp_on_b,
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &packet.port_id_on_a,
            &packet.chan_id_on_a,
            packet.sequence,
            packet_commitment,
        )
        .unwrap();
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();
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
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_b;
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
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            IbcClientId::default(),
            light_client.to_string(),
        )
        .unwrap();
    contract
        .store_next_sequence_ack(
            &mut deps.storage,
            packet.port_id_on_b.clone(),
            packet.chan_id_on_b,
            1.into(),
        )
        .unwrap();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let res = contract.acknowledgement_packet_validate(deps.as_mut(), info, env, &msg);
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages[0].id, 531)
}

#[test]
fn test_acknowledgement_packet_validate_unordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let mut env = get_mock_env();
    env.block.height = 100;

    let height = 10;
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();
    let packet = msg.packet.clone();
    //Store channel, connection and packet commitment
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(
            packet.port_id_on_b.clone(),
            Some(packet.chan_id_on_b.clone()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a.clone(),
            chan_end_on_a_ordered.clone(),
        )
        .unwrap();
    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let conn_end_on_a = ConnectionEnd::new(
        ConnectionState::Open,
        ClientId::default(),
        ConnectionCounterparty::new(
            ClientId::default(),
            Some(ConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );
    contract
        .store_connection(
            &mut deps.storage,
            chan_end_on_a_ordered.connection_hops()[0].clone(),
            conn_end_on_a,
        )
        .unwrap();
    let packet_commitment = compute_packet_commitment(
        &msg.packet.data,
        &msg.packet.timeout_height_on_b,
        &msg.packet.timeout_timestamp_on_b,
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &packet.port_id_on_a,
            &packet.chan_id_on_a,
            packet.sequence,
            packet_commitment,
        )
        .unwrap();
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();
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
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_b;
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

    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            IbcClientId::default(),
            light_client.to_string(),
        )
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let res = contract.acknowledgement_packet_validate(deps.as_mut(), info, env, &msg);
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().messages[0].id, 531)
}

#[test]
fn test_acknowledgement_packet_validate_without_commitment() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let env = get_mock_env();

    let height = 10;
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();
    let packet = msg.packet.clone();
    //Store channel, connection and packet commitment
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(
            packet.port_id_on_b.clone(),
            Some(packet.chan_id_on_b.clone()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a,
            chan_end_on_a_ordered.clone(),
        )
        .unwrap();
    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let conn_end_on_a = ConnectionEnd::new(
        ConnectionState::Open,
        ClientId::default(),
        ConnectionCounterparty::new(
            ClientId::default(),
            Some(ConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );
    contract
        .store_connection(
            &mut deps.storage,
            chan_end_on_a_ordered.connection_hops()[0].clone(),
            conn_end_on_a,
        )
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
        ..get_default_icon_client_state()
    }
    .try_into()
    .unwrap();
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
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_b;
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
    let light_client = Addr::unchecked("lightclient");
    contract
        .store_client_implementations(
            &mut deps.storage,
            IbcClientId::default(),
            light_client.to_string(),
        )
        .unwrap();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let res = contract.acknowledgement_packet_validate(deps.as_mut(), info, env, &msg);
    assert!(res.is_ok());
    assert!(res.as_ref().unwrap().messages.is_empty())
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_acknowledgement_packet_validate_fail_missing_channel() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let height = 10;
    let env = get_mock_env();
    let msg = MsgAcknowledgement::try_from(get_dummy_raw_msg_acknowledgement(height)).unwrap();

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, env, &msg)
        .unwrap();
}
