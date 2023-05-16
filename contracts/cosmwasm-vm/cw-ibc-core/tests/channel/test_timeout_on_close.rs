use super::*;

#[test]
fn test_timeout_on_close_packet_validate_to_light_client() {
    use ibc::core::ics03_connection::connection::Counterparty as ConnectionCounterparty;
    use ibc::core::ics03_connection::connection::State as ConnectionState;

    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let height = 2;
    let timeout_timestamp = 5;
    let msg = MsgTimeoutOnClose::try_from(get_dummy_raw_msg_timeout_on_close(
        height,
        timeout_timestamp,
    ))
    .unwrap();
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
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
            chan_end_on_a_ordered.clone(),
        )
        .unwrap();
    let conn_prefix = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );

    let conn_end_on_a = ConnectionEnd::new(
        ConnectionState::Open,
        ClientId::default().ibc_client_id().clone(),
        ConnectionCounterparty::new(
            ClientId::default().ibc_client_id().clone(),
            Some(ConnectionId::default().connection_id().clone()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );
    contract
        .store_connection(
            &mut deps.storage,
            chan_end_on_a_ordered.connection_hops()[0].clone().into(),
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
            &packet.port_id_on_a.clone().into(),
            &packet.chan_id_on_a.clone().into(),
            packet.seq_on_a.clone(),
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
    }
    .try_into()
    .unwrap();

    let client = client_state.encode_to_vec();
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client)
        .unwrap();
    let client_type = ClientType::from(IbcClientType::new("iconclient".to_string()));

    contract
        .store_client_into_registry(
            &mut deps.storage,
            client_type,
            "contractaddress".to_string(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_b;
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

    let res = contract.timeout_on_close_packet_validate_to_light_client(deps.as_mut(), info, msg);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 541)
}
