use super::*;

#[test]
fn test_packet_send() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = mock_env();

    let chan_end_on_a = ChannelEnd::new(
        State::TryOpen,
        Order::default(),
        Counterparty::new(IbcPortId::default(), Some(IbcChannelId::default())),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );

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
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let mut packet: Packet =
        get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds())
            .try_into()
            .unwrap();
    packet.sequence = 1.into();
    packet.data = vec![0];

    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a.clone(),
            chan_end_on_a.clone(),
        )
        .unwrap();
    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, conn_id_on_a.clone(), conn_end_on_a)
        .unwrap();
    contract
        .store_next_sequence_send(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a.clone(),
            1.into(),
        )
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 10,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(&mut deps.storage, &env, &IbcClientId::default(), client)
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = RawHeight {
        revision_number: 0,
        revision_height: 10,
    }
    .try_into()
    .unwrap();
    let consenus_state = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();

    let res = contract.send_packet(deps.as_mut(), packet);
    assert!(res.is_ok());
    let res = res.unwrap();
    assert_eq!(res.attributes[0].value, "send_packet");
    assert_eq!(res.events[0].ty, IbcEventType::SendPacket.as_str())
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_packet_send_fail_channel_not_found() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let mut packet: Packet =
        get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds())
            .try_into()
            .unwrap();
    packet.sequence = 1.into();
    packet.data = vec![0];
    contract.send_packet(deps.as_mut(), packet).unwrap();
}

#[test]
#[should_panic(
    expected = "Std(NotFound { kind: \"common::ibc::core::ics04_channel::packet::Sequence\" })"
)]
fn test_packet_send_fail_misiing_sequense() {
    let env = mock_env();
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();

    let chan_end_on_a = ChannelEnd::new(
        State::TryOpen,
        Order::default(),
        Counterparty::new(IbcPortId::default(), Some(IbcChannelId::default())),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );

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
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let mut packet: Packet =
        get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds())
            .try_into()
            .unwrap();
    packet.sequence = 1.into();
    packet.data = vec![0];

    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a.clone(),
            chan_end_on_a.clone(),
        )
        .unwrap();
    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, conn_id_on_a.clone(), conn_end_on_a)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 10,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(&mut deps.storage, &env, &IbcClientId::default(), client)
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = RawHeight {
        revision_number: 0,
        revision_height: 10,
    }
    .try_into()
    .unwrap();
    let consenus_state = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();

    contract.send_packet(deps.as_mut(), packet).unwrap();
}
