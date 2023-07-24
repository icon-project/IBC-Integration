use cw_ibc_core::{
    light_client::light_client::LightClient, VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
};

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
            msg.packet.port_id_on_a.clone(),
            msg.packet.chan_id_on_a,
            1.into(),
        )
        .unwrap();

    let res = contract.acknowledgement_packet_execute(deps.as_mut(), message);
    println!("{:?}", res);
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

    let conn_end_on_a = get_dummy_connection();

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
            &packet.port_id_on_b,
            &packet.chan_id_on_b,
            packet.sequence,
            packet_commitment,
        )
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
    let consenus_state: ConsensusState = get_dummy_consensus_state();
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
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, IbcClientId::default(), light_client)
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
    mock_lightclient_reply(&mut deps);
    contract
        .bind_port(
            &mut deps.storage,
            &packet.port_id_on_b,
            "moduleaddress".to_string(),
        )
        .unwrap();

    let res = contract.acknowledgement_packet_validate(deps.as_mut(), info, env, &msg);
    println!("{:?}", res);
    assert!(res.is_ok());
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
    let conn_end_on_a = get_dummy_connection();
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
    let consenus_state: ConsensusState = get_dummy_consensus_state();
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

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, IbcClientId::default(), light_client)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();
    mock_lightclient_reply(&mut deps);
    contract
        .bind_port(
            &mut deps.storage,
            &packet.port_id_on_b,
            "moduleaddress".to_string(),
        )
        .unwrap();

    let res = contract.acknowledgement_packet_validate(deps.as_mut(), info, env, &msg);
    assert!(res.is_ok());
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

    let conn_end_on_a = get_dummy_connection();
    contract
        .store_connection(
            &mut deps.storage,
            chan_end_on_a_ordered.connection_hops()[0].clone(),
            conn_end_on_a,
        )
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
    let consenus_state: ConsensusState = get_dummy_consensus_state();
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
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, IbcClientId::default(), light_client)
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
