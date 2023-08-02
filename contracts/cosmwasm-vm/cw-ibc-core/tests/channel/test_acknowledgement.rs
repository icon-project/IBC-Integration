use cw_ibc_core::{
    conversions::{
        to_ibc_channel_id, to_ibc_packet, to_ibc_timeout_block, to_ibc_timeout_height,
        to_ibc_timestamp,
    },
    light_client::light_client::LightClient,
    VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
};

use super::*;

#[test]
fn test_acknowledgement_packet_execute() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let height = 50;
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let ibc_packet = to_ibc_packet(packet.clone()).unwrap();
    let ack = IbcAcknowledgement::new(msg.acknowledgement);
    let address = Addr::unchecked(msg.signer);
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
        Counterparty::new(dst_port, Some(dst_channel)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port,
            &src_channel,
            &chan_end_on_a_ordered,
        )
        .unwrap();
    let commitment = common::ibc::core::ics04_channel::commitment::PacketCommitment::from(
        "asdfd".as_bytes().to_vec(),
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &src_port,
            &src_channel,
            packet.sequence.into(),
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
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let ibc_packet = to_ibc_packet(packet.clone()).unwrap();
    let ack = IbcAcknowledgement::new(msg.acknowledgement);
    let address = Addr::unchecked(msg.signer);
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
        Counterparty::new(dst_port, Some(dst_channel)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port,
            &src_channel,
            &chan_end_on_a_ordered,
        )
        .unwrap();
    let commitment = common::ibc::core::ics04_channel::commitment::PacketCommitment::from(
        "asdfd".as_bytes().to_vec(),
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &src_port,
            &src_channel,
            packet.sequence.into(),
            commitment,
        )
        .unwrap();
    contract
        .store_next_sequence_ack(&mut deps.storage, &src_port, &src_channel, &1.into())
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
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();
    let ibc_packet = to_ibc_packet(packet.clone()).unwrap();
    let ack = IbcAcknowledgement::new(msg.acknowledgement);
    let address = Addr::unchecked(msg.signer);
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
        Counterparty::new(dst_port, Some(dst_channel)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port,
            &src_channel,
            &chan_end_on_a_ordered,
        )
        .unwrap();
    let commitment = common::ibc::core::ics04_channel::commitment::PacketCommitment::from(
        "asdfd".as_bytes().to_vec(),
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &src_port,
            &src_channel,
            packet.sequence.into(),
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
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone()).unwrap();
    let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp).unwrap();
    let packet_sequence = Sequence::from(packet.sequence);
    let proof_height = to_ibc_height(msg.proof_height.clone()).unwrap();
    //Store channel, connection and packet commitment
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Ordered,
        Counterparty::new(dst_port.clone(), Some(dst_channel.clone())),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port,
            &src_channel,
            &chan_end_on_a_ordered,
        )
        .unwrap();

    let conn_end_on_a = get_dummy_connection();

    contract
        .store_connection(
            &mut deps.storage,
            &chan_end_on_a_ordered.connection_hops()[0],
            &conn_end_on_a,
        )
        .unwrap();
    let packet_commitment =
        compute_packet_commitment(&packet.data, &packet_timeout_height, &packet_timestamp);
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &dst_port,
            &dst_channel,
            packet_sequence,
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
    // let height = msg.proof_height_on_b;
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    contract
        .store_next_sequence_ack(&mut deps.storage, &dst_port, &dst_channel, &1.into())
        .unwrap();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();
    mock_lightclient_reply(&mut deps);
    contract
        .bind_port(&mut deps.storage, &dst_port, "moduleaddress".to_string())
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
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone()).unwrap();
    let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp).unwrap();
    let packet_sequence = Sequence::from(packet.sequence);
    let proof_height = to_ibc_height(msg.proof_height.clone()).unwrap();

    //Store channel, connection and packet commitment
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(dst_port.clone(), Some(dst_channel)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port.clone(),
            &src_channel.clone(),
            &chan_end_on_a_ordered,
        )
        .unwrap();
    let conn_end_on_a = get_dummy_connection();
    contract
        .store_connection(
            &mut deps.storage,
            &chan_end_on_a_ordered.connection_hops()[0].clone(),
            &conn_end_on_a,
        )
        .unwrap();
    let packet_commitment =
        compute_packet_commitment(&packet.data, &packet_timeout_height, &packet_timestamp);
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &src_port,
            &src_channel,
            packet_sequence,
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
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();

    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();

    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();
    mock_lightclient_reply(&mut deps);
    contract
        .bind_port(&mut deps.storage, &dst_port, "moduleaddress".to_string())
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
    let msg = get_dummy_raw_msg_acknowledgement(height);
    let packet = msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let _packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone()).unwrap();
    let _packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp).unwrap();
    let _packet_sequence = Sequence::from(packet.sequence);
    let proof_height = to_ibc_height(msg.proof_height.clone()).unwrap();
    //Store channel, connection and packet commitment
    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(dst_port, Some(dst_channel)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port,
            &src_channel,
            &chan_end_on_a_ordered,
        )
        .unwrap();

    let conn_end_on_a = get_dummy_connection();
    contract
        .store_connection(
            &mut deps.storage,
            &chan_end_on_a_ordered.connection_hops()[0].clone(),
            &conn_end_on_a,
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
    // let height = msg.proof_height_on_b;
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let light_client = LightClient::new("lightclient".to_string());
    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
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
    let msg = get_dummy_raw_msg_acknowledgement(height);

    contract
        .acknowledgement_packet_validate(deps.as_mut(), info, env, &msg)
        .unwrap();
}
