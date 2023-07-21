use cw_common::from_binary_response;
use cw_ibc_core::{light_client::light_client::LightClient, VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE};

use super::*;

#[test]
fn test_execute_timeout_packet() {
    let height = 2;
    let timeout_timestamp = 5;
    let msg = MsgTimeoutOnClose::try_from(get_dummy_raw_msg_timeout_on_close(
        height,
        timeout_timestamp,
    ))
    .unwrap();

    let packet = msg.packet;
    // Set up test environment
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let data = Binary::from(b"test-data".to_vec());
    let (src, dst) = get_dummy_endpoints();
    let timeout = IbcTimeoutBlock {
        revision: 6,
        height: 6,
    };
    let timeout = IbcTimeout::with_block(timeout);
    // Set up test input data
    let data = IbcPacket::new(data, src, dst, 1, timeout);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
            &data,
        )
        .unwrap();

    let data_bin = to_binary(&data).unwrap();
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
            chan_end_on_a_ordered,
        )
        .unwrap();
    let commitment = common::ibc::core::ics04_channel::commitment::PacketCommitment::from(
        "asdfd".as_bytes().to_vec(),
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &packet.port_id_on_a,
            &packet.chan_id_on_a,
            packet.sequence,
            commitment,
        )
        .unwrap();
    // Call the function being tested
    let res = contract.execute_timeout_packet(deps.as_mut(), message);

    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().attributes[1].value, "execute_timeout_packet",)
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_execute_timeout_packet_fails() {
    let height = 2;
    let timeout_timestamp = 5;
    let msg = MsgTimeoutOnClose::try_from(get_dummy_raw_msg_timeout_on_close(
        height,
        timeout_timestamp,
    ))
    .unwrap();

    let packet = msg.packet;
    // Set up test environment
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let data = Binary::from(b"test-data".to_vec());
    let (src, dst) = get_dummy_endpoints();
    let timeout = IbcTimeoutBlock {
        revision: 6,
        height: 6,
    };
    let timeout = IbcTimeout::with_block(timeout);
    // Set up test input data
    let data = IbcPacket::new(data, src, dst, 1, timeout);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
            &data,
        )
        .unwrap();

    let data_bin = to_binary(&data).unwrap();
    let result = SubMsgResponse {
        data: Some(data_bin),
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let message = Reply { id: 0, result };

    let commitment = common::ibc::core::ics04_channel::commitment::PacketCommitment::from(
        "asdfd".as_bytes().to_vec(),
    );
    contract
        .store_packet_commitment(
            &mut deps.storage,
            &packet.port_id_on_a,
            &packet.chan_id_on_a,
            packet.sequence,
            commitment,
        )
        .unwrap();
    contract
        .execute_timeout_packet(deps.as_mut(), message)
        .unwrap();
}

#[test]
fn test_packet_data() {
    let proof_height = 50;
    let timeout_height = proof_height;
    let timeout_timestamp = 0;
    let default_raw_msg =
        get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = MsgTimeout::try_from(default_raw_msg).unwrap();
    let message_info = cw_common::types::MessageInfo {
        sender: info.sender,
        funds: info.funds,
    };
    let packet_data = PacketData {
        packet: msg.packet.clone(),
        signer: msg.signer.clone(),
        acknowledgement: None,
        message_info,
    };
    let bin = to_binary(&packet_data).unwrap();
    let data = from_binary_response::<PacketData>(&bin);
    let packet_date = data.unwrap().packet;

    assert_eq!(packet_date, msg.packet);
}

#[test]
fn test_timeout_packet_validate_to_light_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let proof_height = 50;
    let timeout_height = proof_height - 1;
    let timeout_timestamp = 0;
    let default_raw_msg =
        get_dummy_raw_msg_timeout(proof_height, timeout_height, timeout_timestamp);
    let msg = MsgTimeout::try_from(default_raw_msg).unwrap();
    let packet = msg.packet.clone();
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
    let _client_type = IbcClientType::new("iconclient".to_string());

    let light_client = LightClient::new("lightclient".to_string());

    contract
        .bind_port(
            &mut deps.storage,
            &packet.port_id_on_a,
            "moduleaddress".to_string(),
        )
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3],
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
    let env = get_mock_env();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let res = contract.timeout_packet_validate_to_light_client(deps.as_mut(), info, env, msg);

    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE
    )
}
