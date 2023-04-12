use super::*;

#[test]
fn test_packet_send() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();

    let chan_end_on_a = ChannelEnd::new(
        State::TryOpen,
        Order::default(),
        Counterparty::new(IbcPortId::default(), Some(IbcChannelId::default())),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );

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
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let mut packet: Packet =
        get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds())
            .try_into()
            .unwrap();
    packet.seq_on_a = 1.into();
    packet.data = vec![0];

    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
            chan_end_on_a.clone(),
        )
        .unwrap();
    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
    contract
        .store_connection(
            &mut deps.storage,
            conn_id_on_a.clone().into(),
            conn_end_on_a.clone(),
        )
        .unwrap();
    contract
        .store_next_sequence_send(
            &mut deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
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

    let client = to_vec(&client_state);
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client.unwrap())
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
    let consenus_state = to_vec(&consenus_state).unwrap();
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
    assert_eq!(res.clone().attributes[0].value, "send_packet");
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
    packet.seq_on_a = 1.into();
    packet.data = vec![0];
    contract.send_packet(deps.as_mut(), packet).unwrap();
}

#[test]
#[should_panic(expected = "ibc::core::ics04_channel::packet::Sequence")]
fn test_packet_send_fail_misiing_sequense() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();

    let chan_end_on_a = ChannelEnd::new(
        State::TryOpen,
        Order::default(),
        Counterparty::new(IbcPortId::default(), Some(IbcChannelId::default())),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );

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
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let mut packet: Packet =
        get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds())
            .try_into()
            .unwrap();
    packet.seq_on_a = 1.into();
    packet.data = vec![0];

    contract
        .store_channel_end(
            &mut deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
            chan_end_on_a.clone(),
        )
        .unwrap();
    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
    contract
        .store_connection(
            &mut deps.storage,
            conn_id_on_a.clone().into(),
            conn_end_on_a.clone(),
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

    let client = to_vec(&client_state);
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client.unwrap())
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
    let consenus_state = to_vec(&consenus_state).unwrap();
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
